use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::store::{FileId, FileStore};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct IncludeNode {
    file_id: FileId,
    props: HashMap<String, String>,
    hint: Hint,
}

impl IncludeNode {
    pub fn parse(s: &str, hint: Hint, fs: &mut FileStore) -> Result<Node, AstError> {
        let inner = s
            .trim_start_matches("{#include")
            .trim_end_matches("}")
            .trim();

        let mut parts = inner.split_whitespace();

        let path = parts.next().ok_or_else(|| {
            let s = format!("Failed to find 'path' for INCLUDE directive");
            AstError::InvalidSyntax(s)
        })?;

        let path = PathBuf::from(path).canonicalize().map_err(|e| {
            let s = format!(
                "Failed to find the include path {} in INCLUDE directive\n{}",
                path,
                e.to_string()
            );
            AstError::InvalidSyntax(s)
        })?;

        let file_id = match fs.get_by_path(&path) {
            Some(id) => Ok(id),
            None => fs.add(&path).map_err(|e| {
                let s = format!(
                    "Failed to find the include path {} in INCLUDE directive\n{}",
                    path.display(),
                    e.to_string()
                );
                AstError::InvalidSyntax(s)
            }),
        }?;

        let mut props: HashMap<String, String> = HashMap::new();
        for part in parts {
            let mut kv = part.split('=');

            let key = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'key' of prop in INCLUDE directive");
                AstError::InvalidSyntax(s)
            })?;

            let value = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'value' of prop in INCLUDE diretive");
                AstError::InvalidSyntax(s)
            })?;

            props.insert(key.to_string(), value.to_string());
        }

        Ok(Node::Include(Self {
            file_id,
            props,
            hint,
        }))
    }
}
