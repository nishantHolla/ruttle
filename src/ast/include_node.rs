use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DIRECTIVE_END, INCLUDE_DIRECTIVE_START, KV_SPLIT};
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
            .trim_start_matches(INCLUDE_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
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
            let mut kv = part.split(KV_SPLIT);

            let key = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'key' of prop in INCLUDE directive");
                AstError::InvalidSyntax(s)
            })?;

            let value = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'value' of prop in INCLUDE diretive");
                AstError::InvalidSyntax(s)
            })?;

            if !value.starts_with('"') || !value.ends_with('"') {
                let s = format!("'value' of INCLUDE directive is not wrapped with double quotes");
                return Err(AstError::InvalidSyntax(s));
            }

            let value = value.trim_matches('"');

            props.insert(key.to_string(), value.to_string());
        }

        Ok(Node::Include(Self {
            file_id,
            props,
            hint,
        }))
    }

    pub fn to_string(&self) -> String {
        let mut props_str = String::new();

        let mut counter = 1;
        for (k, v) in &self.props {
            props_str.push_str(&format!("{}=\"{}\"", k, v));

            if counter != self.props.len() {
                props_str.push_str(" ");
            }

            counter += 1;
        }

        format!(
            "IncludeNode({:?}, {}, {})",
            self.file_id,
            props_str,
            self.hint.to_string()
        )
    }

    pub fn debug(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}
