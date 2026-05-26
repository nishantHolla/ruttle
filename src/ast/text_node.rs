use super::error::AstError;
use super::hint::Hint;
use super::node::{AstNode, Node};
use crate::context::Context;
use crate::store::NodeStore;
use crate::util;

#[derive(Clone)]
pub struct TextNode {
    hint: Hint,
}

impl AstNode for TextNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);

        let file_id = self.hint.file_id();
        let start = self.hint.start();
        let end = self.hint.end();

        let path = ctx.file_store.get_by_id(file_id).ok_or_else(|| {
            let s = format!("Failed to find path for file id {:?}", file_id);
            AstError::EvaluationFailed(s)
        })?;

        let substr = util::file::get_substr(path, start, end).ok_or_else(|| {
            let s = format!("Failed to extract text from file {}", path.display());
            AstError::EvaluationFailed(s)
        })?;

        ctx.hint_stack.pop();
        Ok(substr)
    }

    fn to_string(&self) -> String {
        format!("TextNode({})", self.hint.to_string())
    }

    fn debug(&self, indent: usize, _: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}

impl TextNode {
    pub fn parse(hint: Hint) -> Result<Node, AstError> {
        let node = Self { hint };
        Ok(Box::new(node))
    }
}
