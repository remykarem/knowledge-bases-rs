use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType, SentenceEmbeddingsModel,
};

use crate::document_loader::Document;

#[async_trait]
pub trait Embedding {
    async fn embed_documents(&self, documents: &[Document]) -> Vec<Vec<f32>>;
    async fn embed_query(&self, query: &str) -> Vec<Vec<f32>>;
}

pub struct SomeEmbedding {
    model: Arc<Mutex<SentenceEmbeddingsModel>>,
}

impl SomeEmbedding {
    pub fn new() -> Arc<Mutex<SentenceEmbeddingsModel>> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .unwrap();

        Arc::new(Mutex::new(model))
    }
}

#[async_trait]
impl Embedding for SomeEmbedding {
    async fn embed_documents(&self, documents: &[Document]) -> Vec<Vec<f32>> {
        todo!()
    }

    async fn embed_query(&self, query: &str) -> Vec<Vec<f32>> {
        todo!()
    }
}
