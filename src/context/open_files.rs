use super::error::OpenFilesError;
use crate::handler::{JsonFile, MarkdownFile};
use crate::store::FileId;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

enum FileType {
    Concrete(FileId),
    Pseudo(Value),
}

enum FileHandler {
    Markdown(MarkdownFile),
    Json(JsonFile),
}

pub struct OpenFiles {
    identifier_map: HashMap<String, FileType>,
    file_map: HashMap<FileId, FileHandler>,
}

impl OpenFiles {
    pub fn new() -> Self {
        Self {
            identifier_map: HashMap::new(),
            file_map: HashMap::new(),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<Value> {
        let mut parts = key.split('.');
        let identifier = parts.next().unwrap();
        let parts: Vec<&str> = parts.collect();

        if parts.len() == 0 {
            return None;
        }

        if !self.identifier_map.contains_key(identifier) {
            return None;
        }

        let file_type = self.identifier_map.get(identifier).unwrap();
        if let FileType::Pseudo(p) = file_type {
            return JsonFile::resolve_to_value(p, &parts);
        } else {
            return None;
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let mut parts = key.split('.');
        let identifier = parts.next().unwrap();
        let parts: Vec<&str> = parts.collect();
        if parts.len() == 0 {
            return None;
        }

        if !self.identifier_map.contains_key(identifier) {
            return None;
        }

        let file_type = self.identifier_map.get(identifier).unwrap();
        let file_id = match file_type {
            FileType::Concrete(c) => c,
            FileType::Pseudo(v) => return JsonFile::resolve(v, &parts),
        };

        let file = self.file_map.get(file_id).unwrap();
        match file {
            FileHandler::Markdown(m) => {
                if parts.len() == 1 && parts[0] == "content" {
                    return Some(m.content().to_string());
                } else {
                    return m.resolve(&parts);
                }
            }
            FileHandler::Json(j) => {
                return j.reslove(&parts);
            }
        }
    }

    pub fn open_pseudo(&mut self, identifier: &str, v: &Value) -> Result<(), OpenFilesError> {
        self.identifier_map
            .insert(identifier.to_string(), FileType::Pseudo(v.clone()));

        Ok(())
    }

    pub fn open(
        &mut self,
        identifier: &str,
        path: impl AsRef<Path>,
        file_id: FileId,
    ) -> Result<(), OpenFilesError> {
        if self.identifier_map.contains_key(identifier) {
            let s = format!("File with identifier '{}' is already opened", identifier);
            return Err(OpenFilesError::FileOpenFailed(s));
        }

        self.identifier_map
            .insert(identifier.to_string(), FileType::Concrete(file_id));

        if path.as_ref().extension() == Some("md".as_ref()) {
            let file = MarkdownFile::new(file_id, &path).map_err(|e| {
                let s = format!(
                    "Failed to open markdown file {}\n{}",
                    path.as_ref().display(),
                    e
                );
                OpenFilesError::FileOpenFailed(s)
            })?;

            self.file_map.insert(file_id, FileHandler::Markdown(file));
        } else if path.as_ref().extension() == Some("json".as_ref()) {
            let file = JsonFile::new(file_id, &path).map_err(|e| {
                let s = format!(
                    "Failed to open json file {}\n{}",
                    path.as_ref().display(),
                    e
                );
                OpenFilesError::FileOpenFailed(s)
            })?;

            self.file_map.insert(file_id, FileHandler::Json(file));
        } else {
            let s = format!(
                "Unknown file extension {} to open",
                path.as_ref().extension().unwrap().to_string_lossy()
            );
            return Err(OpenFilesError::FileOpenFailed(s));
        }

        Ok(())
    }
}
