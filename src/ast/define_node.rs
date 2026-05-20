use super::error::AstError;
use super::hint::Hint;
use super::node::Node;

pub struct DefineNode {
    key: String,
    value: String,
    hint: Hint,
}

impl DefineNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let inner = s
            .trim_start_matches("{#define")
            .trim_end_matches("}")
            .trim();

        let mut parts = inner.split("=");

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
}
