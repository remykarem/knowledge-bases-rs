use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{CreateCollection, SearchPoints, VectorParams, VectorsConfig};
use rust_bert::pipelines::{
    question_answering::{QaInput, QuestionAnsweringModel},
    sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType},
};
use std::collections::HashMap;
use std::fs;

use globwalk::GlobWalkerBuilder;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Document {
    id: String,
    text: String,
}

impl AsRef<str> for Document {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

pub fn find_files_with_pattern(path: &Path, pattern: &str) -> Result<Vec<Document>> {
    let mut file_paths: Vec<Document> = Vec::new();
    let walker = GlobWalkerBuilder::new(path, pattern)
        .build()
        .map_err(|e| format!("Failed to build GlobWalker: {}", e))
        .unwrap();

    for entry in walker {
        match entry {
            Ok(file) => {
                if let Some(file_str) = file.path().to_str() {
                    let file_path = file_str.to_string();
                    let content = fs::read_to_string(&file_path)
                        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))
                        .unwrap();
                    file_paths.push(Document {
                        id: file_path,
                        text: content,
                    });
                }
            }
            Err(e) => eprintln!("Error processing entry: {}", e),
        }
    }

    Ok(file_paths)
}

fn something(documents: &[Document]) -> Vec<Vec<f32>> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
        .create_model()
        .unwrap();

    // let sentences = ["this is an example sentence", "each sentence is converted"];

    model.encode(documents).unwrap()
}

async fn add(data: Vec<Vec<f32>>) -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config)).await?;

    let collection_name = "hmmm";

    // client
    //     .create_collection(&CreateCollection {
    //         collection_name: collection_name.into(),
    //         vectors_config: Some(VectorsConfig {
    //             config: Some(Config::Params(VectorParams {
    //                 size: 10,
    //                 distance: Distance::Cosine.into(),
    //             })),
    //         }),
    //         ..Default::default()
    //     })
    //     .await?;

    let payload = vec![("foo", "Bar".into()), ("bar", 12.into())]
        .into_iter()
        .collect::<HashMap<_, Value>>()
        .into();

    let points = vec![PointStruct::new(1, vec![10.; 10], payload)];

    client
        .upsert_points_blocking(collection_name, points, None)
        .await?;

    Ok(())
}

async fn search(vector: Vec<f32>) -> Result<()> {
    let collection_name = "hmmm";
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config)).await?;

    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector,
            filter: None,
            limit: 10,
            with_vectors: None,
            with_payload: None,
            params: None,
            score_threshold: None,
            offset: None,
            ..Default::default()
        })
        .await?;
    dbg!(search_result);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let docs = find_files_with_pattern(
        Path::new("/Users/raimibinkarim/Desktop/remykarem.github.io/books/ai-book/src"),
        "**/*.md",
    )
    .unwrap();

    // let query = "hello";

    let embs = tokio::task::spawn_blocking(move || something(&docs)).await?;

    dbg!(embs.len());

    add(embs).await?;
    // search(vec![9.; 10]).await?;

    Ok(())
}
