use super::error::OpenFilesError;
use super::files::{File, JsonFile, MarkdownFile};
use crate::store::FileId;
use std::collections::HashMap;
use std::path::Path;

use gray_matter::{Matter, engine::TOML, engine::YAML};
use pulldown_cmark::{Parser, html};
use serde_json::Value as JsonValue;
use serde_yaml::Value as MdValue;

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

    pub fn get(&self, key: &str) -> Option<String> {
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
                    return Some(m.content().to_string());
                } else {
                    return m.resolve(&parts);
                }
            }
            File::Json(j) => {
                return j.reslove(&parts);
            }
        }
    }

    fn parse_frontmatter(&self, s: &str) -> Option<(Option<MdValue>, String)> {
        if s.starts_with("---") {
            let mut matter: Matter<YAML> = Matter::new();
            matter.delimiter = "---".to_owned();
            matter.close_delimiter = Some("---".to_owned());
            if let Ok(parsed) = matter.parse::<MdValue>(s) {
                return Some((parsed.data, parsed.content));
            }
        } else if s.starts_with("+++") {
            let mut matter: Matter<TOML> = Matter::new();
            matter.delimiter = "+++".to_owned();
            matter.close_delimiter = Some("+++".to_owned());
            if let Ok(parsed) = matter.parse::<MdValue>(s) {
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

            let file = MarkdownFile::new(file_id, frontmatter, html_output);
            self.file_map.insert(file_id, File::Markdown(file));
        } else if path.as_ref().extension() == Some("json".as_ref()) {
            let value: JsonValue = serde_json::from_str(&s).map_err(|e| {
                let s = format!(
                    "Failed to parse json file at {}\n{}",
                    path.as_ref().display(),
                    e.to_string()
                );
                OpenFilesError::FileOpenFailed(s)
            })?;

            let file = JsonFile::new(file_id, Some(value));
            self.file_map.insert(file_id, File::Json(file));
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
