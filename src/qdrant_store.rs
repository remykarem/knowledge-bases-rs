use crate::{document_loader::DirectoryLoader, embedding::Embedding, vector_store::VectorStore};
use std::{collections::HashMap, path::Path};

use anyhow::Result;
use async_trait::async_trait;
use qdrant_client::{
    prelude::*,
    qdrant::{
        vectors_config::Config, with_payload_selector::SelectorOptions, PayloadIncludeSelector,
        VectorParams, VectorsConfig, WithPayloadSelector,
    },
};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;
use tokio::runtime::{Builder, Handle, Runtime};

use crate::Document;

pub struct QdrantStore
{
    client: QdrantClient,
    // embedding: E,
    // rt: Runtime,
}

impl QdrantStore
{
    pub async fn new() -> Self
    {
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        // let rt = Builder::new_current_thread().build().unwrap();
        // let client = rt.block_on(async {
           let client = QdrantClient::new(Some(config))
                .await
                .expect("Failed to create client");
        // });

        Self {
            client,
            // embedding,
            // rt,
        }
    }
}

#[async_trait]
impl VectorStore for QdrantStore
{
    async fn get_all_collections(&self) -> Vec<String> {
        let collections = 
            self.client
                .list_collections()
                .await
                .expect("Failed to list collections")
        ;

        collections
            .collections
            .into_iter()
            .map(|collection| collection.name)
            .collect()
    }

    async fn add(&self, model: SentenceEmbeddingsModel, collection_name: &str, docs: &[Document]) -> SentenceEmbeddingsModel {
        let data = model.encode(docs).unwrap();

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
                    vec![("source", docs.get(i).unwrap().id().into())]
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

        model
    }

    async fn search(&self, vector: Vec<f32>, collection_name: &str) -> Vec<String> {
        let search_result = 
            self.client
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

    fn embed_documents(&self, model: SentenceEmbeddingsModel, documents: &[Document]) -> Vec<Vec<f32>> {
        model.encode(documents).unwrap()
    }

    fn embed_query(&self, model: SentenceEmbeddingsModel, query: &str) -> Vec<Vec<f32>> {
        model.encode(&[query]).unwrap()
    }
}
