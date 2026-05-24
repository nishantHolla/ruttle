use super::error::OpenFilesError;
use crate::store::FileId;
use std::collections::HashMap;
use std::path::Path;

use gray_matter::{Matter, engine::TOML, engine::YAML};
use pulldown_cmark::{Parser, html};
use serde_yaml::Value;

struct MarkdownFile {
    file_id: FileId,
    frontmatter: Option<Value>,
    content: String,
}

impl MarkdownFile {
    pub fn resolve(&self, parts: &[&str]) -> Option<&str> {
        let (first, rest) = parts.split_first()?;

        let mut value = self.frontmatter.as_ref()?.get(*first)?;

        for part in rest {
            value = value.get(*part)?;
        }

        value.as_str()
    }
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

    pub fn get(&self, key: &str) -> Option<&str> {
        let mut parts = key.split('.');
        let identifier = parts.next().unwrap();

        if !self.identifier_map.contains_key(identifier) {
            return None;
        }

        let file_id = self.identifier_map.get(identifier).unwrap();
        let file = self.file_map.get(file_id).unwrap();

        let parts: Vec<&str> = parts.collect();

        if parts.len() == 0 {
            return None;
        }

        match file {
            File::Markdown(m) => {
                if parts.len() == 1 && parts[0] == "content" {
                    return Some(&m.content);
                } else {
                    return m.resolve(&parts);
                }
            }
        }
    }

    fn parse_frontmatter(&self, s: &str) -> Option<(Option<Value>, String)> {
        if s.starts_with("---") {
            let mut matter: Matter<YAML> = Matter::new();
            matter.delimiter = "---".to_owned();
            matter.close_delimiter = Some("---".to_owned());
            if let Ok(parsed) = matter.parse::<Value>(s) {
                return Some((parsed.data, parsed.content));
            }
        } else if s.starts_with("+++") {
            let mut matter: Matter<TOML> = Matter::new();
            matter.delimiter = "+++".to_owned();
            matter.close_delimiter = Some("+++".to_owned());
            if let Ok(parsed) = matter.parse::<Value>(s) {
                return Some((parsed.data, parsed.content));
            }
        }

        None
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
            let (frontmatter, content) = self.parse_frontmatter(&s).ok_or_else(|| {
                let s = format!(
                    "Failed to parse front matter in {}",
                    path.as_ref().display()
                );
                OpenFilesError::FileOpenFailed(s)
            })?;

            let parser = Parser::new(&content);
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
