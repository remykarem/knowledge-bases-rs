use anyhow::Result;
use async_trait::async_trait;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

use crate::Document;


#[async_trait]
pub trait VectorStore {
    async fn add(
        &self,
        model: SentenceEmbeddingsModel,
        collection_name: &str,
        docs: &[Document],
    ) -> SentenceEmbeddingsModel;

    async fn search(&self, vector: Vec<f32>, collection_name: &str) -> Vec<String>;

    async fn get_all_collections(&self) -> Vec<String>;

    fn embed_documents(&self, model: SentenceEmbeddingsModel, documents: &[Document]) -> Vec<Vec<f32>>;
    
    fn embed_query(&self, model: SentenceEmbeddingsModel, query: &str) -> Vec<Vec<f32>>;
}
