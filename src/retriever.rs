// use crate::{document_loader::Document, embedding::Embedding, vector_store::VectorStore};
// use async_trait::async_trait;
// use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;
// use tokio::runtime::{Builder, Runtime};

// #[async_trait]
// pub trait Retriever {
//     async fn get_relevant_documents(&self, model: SentenceEmbeddingsModel, text: &str) -> Vec<String>;
// }

// pub struct VectorStoreRetriever<V>
// where
//     V: VectorStore,
// {
//     pub vector_store: V,
//     rt: Runtime,
// }

// #[async_trait]
// impl<V> Retriever for VectorStoreRetriever<V>
// where
//     V: VectorStore,
// {
//     async fn get_relevant_documents(
//         &self,
//         model: SentenceEmbeddingsModel,
//         text: &str,
//     ) -> Vec<String> {
//         let docs = vec![Document::new("0".to_string(), text.to_string())];

//         let mut embs = self.vector_store.embed_documents(model, &docs);

//         self.vector_store.search(embs.pop().unwrap(), "ai-book-4").await
//     }
// }

// impl<V> VectorStoreRetriever<V>
// where
//     V: VectorStore,
// {
//     pub fn from(vector_store: V) -> Self {
//         let rt = Builder::new_current_thread().build().unwrap();
//         Self { vector_store, rt }
//     }
// }
