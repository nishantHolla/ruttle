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

    pub fn to_string(&self) -> String {
        format!("TextNode({})", self.hint.to_string())
    }

    pub fn debug(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}
