use super::error::OpenFilesError;
use crate::store::FileId;
use std::collections::HashMap;
use std::path::Path;

use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Parser, html};
use serde_yaml::Value;

struct MarkdownFile {
    file_id: FileId,
    frontmatter: Option<Value>,
    content: String,
}

enum File {
    Markdown(MarkdownFile),
}

pub struct OpenFiles {
    identifier_map: HashMap<String, FileId>,
    file_map: HashMap<FileId, File>,
}

impl OpenFiles {
    pub fn new() -> Self {
        Self {
            identifier_map: HashMap::new(),
            file_map: HashMap::new(),
        }
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

        self.identifier_map.insert(identifier.to_string(), file_id);

        let s = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            let s = format!(
                "Failed to read file {}\n{}",
                path.as_ref().display(),
                e.to_string()
            );
            OpenFilesError::FileOpenFailed(s)
        })?;

        if path.as_ref().extension() == Some("md".as_ref()) {
            let matter = Matter::<YAML>::new();
            let parsed = matter.parse::<Value>(&s).map_err(|e| {
                let s = format!(
                    "Failed to parse file {} for front matter\n{}",
                    path.as_ref().display(),
                    e.to_string()
                );
                OpenFilesError::FileOpenFailed(s)
            })?;
            let frontmatter = parsed.data;
            let parser = Parser::new(&parsed.content);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            let file = MarkdownFile {
                file_id,
                frontmatter,
                content: html_output,
            };

            self.file_map.insert(file_id, File::Markdown(file));
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
