use super::error::FileStoreError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

pub struct FileStore {
    file_list: Vec<PathBuf>,
    file_map: HashMap<PathBuf, FileId>,
}

impl FileStore {
    pub fn new() -> Self {
        Self {
            file_list: Vec::new(),
            file_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: impl AsRef<Path>) -> Result<FileId, FileStoreError> {
        let path = path.as_ref().canonicalize().map_err(|e| {
            let s = format!(
                "Failed to canonicalize path {}\n{}",
                path.as_ref().display(),
                e.to_string()
            );
            FileStoreError::InvalidPath(s)
        })?;

        if self.file_map.contains_key(&path) {
            let s = format!("Path {} already exists in the file store", path.display());
            return Err(FileStoreError::DuplicatePath(s));
        }

        let id = FileId(self.file_list.len());
        self.file_list.push(path.clone());
        self.file_map.insert(path, id);
        Ok(id)
    }

    pub fn get_by_id(&self, id: FileId) -> Option<&Path> {
        self.file_list.get(id.0).map(|p| p.as_path())
    }

    pub fn get_by_path(&self, path: impl AsRef<Path>) -> Option<FileId> {
        let path = path.as_ref().canonicalize().ok()?;
        self.file_map.get(&path).copied()
    }

    pub fn debug(&self) {
        println!("debug: FileStore\n");
        for (pos, file_path) in self.file_list.iter().enumerate() {
            println!("       {}: {}", pos, file_path.display());
        }
        println!();
    }
}
