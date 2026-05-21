use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DEFINE_DIRECTIVE_START, DIRECTIVE_END, KV_SPLIT};

pub struct DefineNode {
    key: String,
    value: String,
    hint: Hint,
}

impl DefineNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let inner = s
            .trim_start_matches(DEFINE_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        let mut parts = inner.split(KV_SPLIT);

        let key = parts
            .next()
            .ok_or_else(|| {
                let s = format!("Could not find 'key' of DEFINE directive");
                AstError::InvalidSyntax(s)
            })?
            .trim();

        if key.len() == 0 {
            let s = format!("Could not find 'key' of DEFINE directive");
            return Err(AstError::InvalidSyntax(s));
        }

        let value = parts
            .next()
            .ok_or_else(|| {
                let s = format!("Could not find 'value' of DEFINE directive");
                AstError::InvalidSyntax(s)
            })?
            .trim();

        if !value.starts_with('"') || !value.ends_with('"') {
            let s = format!("'value' of DEFINE directive is not wrapped with double quotes");
            return Err(AstError::InvalidSyntax(s));
        }

        let value = value.trim_matches('"');

        Ok(Node::Define(Self {
            key: key.to_string(),
            value: value.to_string(),
            hint,
        }))
    }

    pub fn to_string(&self) -> String {
        format!(
            "DefineNode({}, \"{}\", {})",
            self.key,
            self.value,
            self.hint.to_string()
        )
    }

    pub fn debug(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}
