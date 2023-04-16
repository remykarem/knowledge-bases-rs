use anyhow::Result;
use std::{fs, path::PathBuf};

use globwalk::{GlobWalker, GlobWalkerBuilder};
use std::path::Path;

#[derive(Debug)]
pub struct Document {
    id: String,
    text: String,
}

pub struct DocumentCollection(Vec<Document>);

impl Document {
    pub fn new(id: String, text: String) -> Self {
        Self { id, text }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl AsRef<str> for Document {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

pub struct DirectoryLoader {
    paths: Vec<PathBuf>,
}

impl DirectoryLoader {
    pub fn find_files_with_pattern(path: &Path, pattern: &str) -> Self {
        let walker = GlobWalkerBuilder::new(path, pattern)
            .build()
            .map_err(|e| format!("Failed to build GlobWalker: {}", e))
            .unwrap();

        let paths = walker
            .into_iter()
            .map(|entry| match entry {
                Ok(file) => file.into_path(),
                Err(e) => panic!("Error processing entry: {}", e),
            })
            .collect();

        // Ok(file_paths)

        Self { paths }
    }

    pub fn load(&self) -> Vec<Document> {
        self.paths
            .iter()
            .filter_map(|path| {
                if let Some(file_str) = path.to_str() {
                    let file_path = file_str.to_string();
                    let content = fs::read_to_string(&file_path)
                        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))
                        .unwrap();
                    Some(Document {
                        id: file_path,
                        text: content,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    // async fn create(&self, path: String, pattern: String, collection_name: String) -> Result<()> {
    //     let docs = tokio::task::spawn_blocking(move || {
    //         DirectoryLoader::find_files_with_pattern(Path::new(&path), &pattern).unwrap()
    //     })
    //     .await?;

    //     let (embs, docus) = tokio::task::spawn_blocking(move || embed(docs)).await?;

    //     self.add(embs, &collection_name, &docus).await?;

    //     Ok(())
    // }
}
