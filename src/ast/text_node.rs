use super::error::AstError;
use super::hint::Hint;
use super::node::Node;

pub struct TextNode {
    hint: Hint,
}

impl TextNode {
    pub fn parse(hint: Hint) -> Result<Node, AstError> {
        Ok(Node::Text(Self { hint }))
    }
}
