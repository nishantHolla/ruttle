use crate::store::FileId;
use serde_json::Value;
use std::path::Path;

pub struct JsonFile {
    file_id: FileId,
    value: Value,
}

impl JsonFile {
    fn parse(s: &str) -> Option<Value> {
        let value = serde_json::from_str(&s).ok();
        value
    }

    pub fn new(file_id: FileId, path: impl AsRef<Path>) -> Result<Self, String> {
        let s = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            format!(
                "Failed to read file {}\n{}",
                path.as_ref().display(),
                e.to_string()
            )
        })?;

        let value = JsonFile::parse(&s)
            .ok_or_else(|| format!("Failed to parse file {}", path.as_ref().display()))?;

        Ok(Self { file_id, value })
    }

    pub fn resolve_to_value(value: &Value, parts: &[&str]) -> Option<Value> {
        let mut v = value;

        for part in parts {
            if let Ok(num) = part.parse::<usize>() {
                match v {
                    Value::Array(a) => {
                        v = a.get(num)?;
                    }
                    _ => return None,
                }
            } else {
                v = v.get(*part)?;
            }
        }

        Some(v.clone())
    }

    pub fn resolve(value: &Value, parts: &[&str]) -> Option<String> {
        let mut v = value;

        for part in parts {
            if let Ok(num) = part.parse::<usize>() {
                match v {
                    Value::Array(a) => {
                        v = a.get(num)?;
                    }
                    _ => return None,
                }
            } else {
                v = v.get(*part)?;
            }
        }

        match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    pub fn reslove(&self, parts: &[&str]) -> Option<String> {
        let (first, rest) = parts.split_first()?;
        let value = self.value.get(*first)?;

        JsonFile::resolve(value, rest)
    }
}
