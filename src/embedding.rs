use async_trait::async_trait;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType, SentenceEmbeddingsModel,
};

use crate::document_loader::Document;

#[async_trait]
pub trait Embedding {
    async fn embed_documents(model: SentenceEmbeddingsModel, documents: &[Document]) -> Vec<Vec<f32>>;
    async fn embed_query(model: SentenceEmbeddingsModel, query: &str) -> Vec<Vec<f32>>;
}

pub struct SomeEmbedding {
    model: SentenceEmbeddingsModel,
}

impl SomeEmbedding {
    pub fn new() -> SentenceEmbeddingsModel {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .unwrap();

        model
    }
}

#[async_trait]
impl Embedding for SomeEmbedding {
    async fn embed_documents(model: SentenceEmbeddingsModel, documents: &[Document]) -> Vec<Vec<f32>> {
        // let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
        // .create_model()
        // .unwrap();
        model.encode(documents).unwrap()
    }

    async fn embed_query(model: SentenceEmbeddingsModel, query: &str) -> Vec<Vec<f32>> {
        model.encode(&[query]).unwrap()
    }
}
