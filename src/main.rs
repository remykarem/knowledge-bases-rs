pub mod document_loader;
pub mod embedding;
pub mod qdrant_store;
pub mod retriever;
pub mod vector_store;

use document_loader::{DirectoryLoader, Document};
use std::{io::Write, vec};
use tokio::runtime::Handle;
use vector_store::VectorStore;

use std::path::Path;

use inquire::Select;

#[tokio::main]
async fn main() {
    // create(
    //     "/Users/raimibinkarim/Desktop/_/Utils/remykarem.github.io/books/ai-book/src".into(),
    //     "**/*.md".into(),
    //     "ai-book-4".into(),
    // ).await.unwrap();

    let loader = DirectoryLoader::find_files_with_pattern(
        Path::new("/Users/raimibinkarim/Desktop/_/Utils/remykarem.github.io/books/ai-book/src"),
        "**/*.md",
    );
    let docs = loader.load();
    let mut embedding = tokio::task::spawn_blocking(embedding::SomeEmbedding::new)
        .await
        .unwrap();
    let vector_store = qdrant_store::QdrantStore::new().await;

    loop {
        let select = Select::new("hello", vec!["search", "add"])
            .prompt()
            .unwrap();

        if select == "search" {
        } else {

            let embedding2 = vector_store
                .add(embedding, "collection_name", &docs)
                .await
                ;
        }
    }
}
