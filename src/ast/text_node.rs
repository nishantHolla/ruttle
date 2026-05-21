use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::context::Context;
use crate::util;

pub struct TextNode {
    hint: Hint,
}

impl TextNode {
    pub fn parse(hint: Hint) -> Result<Node, AstError> {
        Ok(Node::Text(Self { hint }))
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
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

    pub fn to_string(&self) -> String {
        format!("TextNode({})", self.hint.to_string())
    }

    pub fn debug(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}
