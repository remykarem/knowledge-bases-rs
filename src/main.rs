pub mod document_loader;
pub mod embedding;
pub mod qdrant_store;
pub mod retriever;
pub mod vector_store;

use document_loader::{DirectoryLoader, Document};
use std::vec;

use std::path::Path;

#[tokio::main]
async fn main() {
    // Load the embedding model
    let embedding = tokio::task::spawn_blocking(embedding::SomeEmbedding::new)
        .await
        .unwrap();

    // Load the vector store
    let vector_store = qdrant_store::QdrantStore::new(embedding.clone()).await;

    loop {
        // Which mode
        let select = inquire::Select::new("Create or search collection?", vec!["Create", "Search"])
            .prompt()
            .unwrap();

        match select {
            "Create" => {
                // Ask the relevant information
                let collection_name = inquire::Text::new("Collection name?").prompt().unwrap();
                let directory = inquire::Text::new("Directory?").prompt().unwrap();
                let pattern = inquire::Text::new("Pattern?").prompt().unwrap();

                // Load the documents
                let docs =
                    DirectoryLoader::find_files_with_pattern(Path::new(&directory), &pattern)
                        .load();

                // Add the documents to the vector store
                vector_store.add(&collection_name, docs).await;
            }
            "Search" => {
                // Ask the relevant information
                let collections = vector_store.get_all_collections().await;
                let collection_name = inquire::Select::new("Which collection?", collections)
                    .prompt()
                    .unwrap();
                let query = inquire::Text::new("Query?").prompt().unwrap();

                // Search the vector store
                let results = vector_store.search(query, &collection_name).await;
                println!("{:?}", results);
            }
            _ => {}
        }
    }
}
