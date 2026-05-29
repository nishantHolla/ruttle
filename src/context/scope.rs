use super::error::ContextError;
use super::open_files::OpenFiles;
use crate::ast::Literal;
use crate::store::FileId;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub type ScopeDef = BTreeMap<String, Literal>;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Fingerprint(u64);

pub struct Scope {
    definitions: ScopeDef,
    open_files: OpenFiles,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            definitions: ScopeDef::new(),
            open_files: OpenFiles::new(),
        }
    }

    pub fn from(definitions: ScopeDef) -> Self {
        Self {
            definitions,
            open_files: OpenFiles::new(),
        }
    }

    pub fn set(&mut self, key: &str, lit: Literal) {
        self.definitions.insert(key.to_string(), lit);
    }

    pub fn get(&self, key: &str) -> Option<&Literal> {
        self.definitions.get(key)
    }

    pub fn resolve_to_value(&self, key: &str) -> Option<Value> {
        self.open_files.get_value(key)
    }

    pub fn get_open_file_id(&self, key: &str) -> Option<FileId> {
        self.open_files.get_open_file_id(key)
    }

    pub fn resolve_to_lit(&self, key: &str) -> Option<Literal> {
        if key.contains('.') {
            self.open_files.get(key).map(|s| Literal::String(s))
        } else {
            self.get(key).cloned()
        }
    }

    pub fn resolve(&self, key: &str) -> Option<String> {
        if key.contains('.') {
            self.open_files.get(key).map(|s| s.to_string())
        } else {
            self.get(key).map(|f| f.to_string())
        }
    }

    pub fn open_pseudo(&mut self, identifier: &str, value: &Value) -> Result<(), ContextError> {
        self.open_files.open_pseudo(identifier, value).map_err(|e| {
            let s = format!("Failed to open pseudo file\n{}", e.to_string());
            ContextError::ScopeError(s)
        })
    }

    pub fn open(
        &mut self,
        identifier: &str,
        path: impl AsRef<Path>,
        file_id: FileId,
    ) -> Result<(), ContextError> {
        self.open_files
            .open(identifier, &path, file_id)
            .map_err(|e| {
                let s = format!(
                    "Failed to open file {}\n{}",
                    path.as_ref().display(),
                    e.to_string()
                );
                ContextError::ScopeError(s)
            })
    }

    pub fn fingerprint(&self, extra: Option<impl Hash>) -> Fingerprint {
        let mut hasher = DefaultHasher::new();

        for (k, v) in &self.definitions {
            k.hash(&mut hasher);
            v.to_string().hash(&mut hasher);
        }

        if let Some(extra) = extra {
            extra.hash(&mut hasher);
        }

        Fingerprint(hasher.finish())
    }
}
