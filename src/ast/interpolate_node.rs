use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DIRECTIVE_END, INTERPOLATE_DIRECTIVE_START};

pub struct InterpolateNode {
    key: String,
    hint: Hint,
}

impl InterpolateNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let inner = s
            .trim_start_matches(INTERPOLATE_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        if inner.len() == 0 {
            let s = format!("Could not find 'value' of INTERPOLATE directive");
            return Err(AstError::InvalidSyntax(s));
        }

        Ok(Node::Interpolate(Self {
            key: inner.to_string(),
            hint,
        }))
    }
}
