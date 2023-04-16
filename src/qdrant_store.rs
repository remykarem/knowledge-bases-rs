use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use qdrant_client::{
    prelude::*,
    qdrant::{
        vectors_config::Config, with_payload_selector::SelectorOptions, PayloadIncludeSelector,
        VectorParams, VectorsConfig, WithPayloadSelector,
    },
};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

use crate::Document;

pub struct QdrantStore {
    client: QdrantClient,
    model: Arc<Mutex<SentenceEmbeddingsModel>>,
}

impl QdrantStore {
    pub async fn new(model: Arc<Mutex<SentenceEmbeddingsModel>>) -> Self {
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        let client = QdrantClient::new(Some(config))
            .await
            .expect("Failed to create client");

        Self { client, model }
    }
}

impl QdrantStore {
    pub async fn get_all_collections(&self) -> Vec<String> {
        let collections = self
            .client
            .list_collections()
            .await
            .expect("Failed to list collections");

        collections
            .collections
            .into_iter()
            .map(|collection| collection.name)
            .collect()
    }

    pub async fn add(&self, collection_name: &str, docs: Vec<Document>) {
        let model = self.model.clone();

        // TODO
        // Clone docs
        let docs2 = docs.clone();

        let data = tokio::task::spawn_blocking(move || {
            let model_lock = model.lock().unwrap();
            model_lock.encode(&docs).unwrap()
        })
        .await
        .unwrap();

        self.client
            .create_collection(&CreateCollection {
                collection_name: collection_name.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 384,
                        distance: Distance::Cosine.into(),
                    })),
                }),
                ..Default::default()
            })
            .await
            .unwrap();

        let points = data
            .into_iter()
            .enumerate()
            .map(|(i, vector)| {
                PointStruct::new(
                    i as u64,
                    vector,
                    vec![("source", docs2.get(i).unwrap().id().into())]
                        .into_iter()
                        .collect::<HashMap<&str, Value>>()
                        .into(),
                )
            })
            .collect::<Vec<_>>();

        self.client
            .upsert_points_blocking(collection_name, points, None)
            .await
            .unwrap();
    }

    pub async fn search(&self, query: String, collection_name: &str) -> Vec<String> {
        let vector = self.embed_query(query).await;

        let search_result = self
            .client
            .search_points(&SearchPoints {
                collection_name: collection_name.into(),
                vector,
                filter: None,
                limit: 3,
                with_vectors: None,
                with_payload: Some(WithPayloadSelector {
                    selector_options: Some(SelectorOptions::Include(PayloadIncludeSelector {
                        fields: vec!["source".to_string()],
                    })),
                }),
                params: None,
                score_threshold: None,
                offset: None,
                ..Default::default()
            })
            .await
            .expect("Error searching");

        search_result
            .result
            .into_iter()
            .map(|point| {
                let m = point.payload.get(&"source".to_string()).unwrap();
                match &m.kind {
                    Some(v) => match v {
                        qdrant_client::qdrant::value::Kind::StringValue(value) => value.clone(),
                        _ => todo!(),
                    },
                    None => todo!(),
                }
            })
            .collect()
    }

    pub async fn embed_documents(&self, docs: Vec<Document>) -> Vec<Vec<f32>> {
        let model = self.model.clone();

        tokio::task::spawn_blocking(move || {
            let model_lock = model.lock().unwrap();
            model_lock.encode(&docs).unwrap()
        })
        .await
        .unwrap()
    }

    async fn embed_query(&self, query: String) -> Vec<f32> {
        let model = self.model.clone();

        tokio::task::spawn_blocking(move || {
            let model_lock = model.lock().unwrap();
            model_lock.encode(&[&query]).unwrap()
        })
        .await
        .unwrap()
        .pop()
        .unwrap()
    }
}
