use std::sync::{Mutex, Arc};

use async_trait::async_trait;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

use crate::Document;


#[async_trait]
pub trait VectorStore {
    async fn add(
        &self,
        collection_name: &str,
        docs: &[Document],
    );

    async fn search(&self, vector: Vec<f32>, collection_name: &str) -> Vec<String>;

    async fn get_all_collections(&self) -> Vec<String>;

    fn embed_documents(&self, documents: &[Document]) -> Vec<Vec<f32>>;
    
    fn embed_query(&self, query: &str) -> Vec<Vec<f32>>;
}
