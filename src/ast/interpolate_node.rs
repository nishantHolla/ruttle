use super::error::AstError;
use super::hint::Hint;
use super::node::Node;

pub struct InterpolateNode {
    key: String,
    hint: Hint,
}

impl InterpolateNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let inner = s.trim_start_matches("{#value").trim_end_matches("}").trim();

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
