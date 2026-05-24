use crate::store::FileId;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

pub enum File {
    Markdown(MarkdownFile),
    Json(JsonFile),
}

pub struct MarkdownFile {
    file_id: FileId,
    frontmatter: Option<YamlValue>,
    content: String,
}

impl MarkdownFile {
    pub fn new(file_id: FileId, frontmatter: Option<YamlValue>, content: String) -> Self {
        Self {
            file_id,
            frontmatter,
            content,
        }
    }

    pub fn resolve(&self, parts: &[&str]) -> Option<&str> {
        let (first, rest) = parts.split_first()?;

        let mut value = self.frontmatter.as_ref()?.get(*first)?;

        for part in rest {
            value = value.get(*part)?;
        }

        value.as_str()
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn frontmatter(&self) -> &Option<YamlValue> {
        &self.frontmatter
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct JsonFile {
    file_id: FileId,
    value: Option<JsonValue>,
}

impl JsonFile {
    pub fn new(file_id: FileId, value: Option<JsonValue>) -> Self {
        Self { file_id, value }
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn value(&self) -> &Option<JsonValue> {
        &self.value
    }

    pub fn reslove(&self, parts: &[&str]) -> Option<&str> {
        let (first, rest) = parts.split_first()?;

        let mut value = self.value.as_ref()?.get(*first)?;

        for part in rest {
            value = value.get(*part)?;
        }

        value.as_str()
    }
}
