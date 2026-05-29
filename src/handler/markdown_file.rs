use crate::store::FileId;
use gray_matter::{Matter, engine::TOML, engine::YAML};
use pulldown_cmark::{Parser, html};
use serde_yaml::Value;
use std::path::Path;

pub struct MarkdownFile {
    file_id: FileId,
    frontmatter: Option<Value>,
    content: String,
}

impl MarkdownFile {
    fn parse(s: &str) -> Option<(Option<Value>, String)> {
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

    pub fn md_to_html(s: &str) -> Result<String, String> {
        let (_, content) =
            MarkdownFile::parse(&s).ok_or_else(|| format!("Failed to parse frontmatter"))?;

        let parser = Parser::new(&content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(html_output)
    }

    pub fn new(file_id: FileId, path: impl AsRef<Path>) -> Result<Self, String> {
        let s = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            format!(
                "Failed to read file {}\n{}",
                path.as_ref().display(),
                e.to_string()
            )
        })?;

        let (frontmatter, content) = MarkdownFile::parse(&s)
            .ok_or_else(|| format!("Failed to parse file {}", path.as_ref().display()))?;

        let parser = Parser::new(&content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(Self {
            file_id,
            frontmatter,
            content: html_output,
        })
    }

    pub fn resolve(&self, parts: &[&str]) -> Option<String> {
        let (first, rest) = parts.split_first()?;

        let mut value = self.frontmatter.as_ref()?.get(*first)?;

        for part in rest {
            value = value.get(*part)?;
        }

        match value {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
