use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DIRECTIVE_END, INTERPOLATE_DIRECTIVE_START};
use crate::context::Context;
use crate::util;

pub struct InterpolateNode {
    key: String,
    hint: Hint,
}

impl InterpolateNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let s = util::string::normalize_whitespace(s, None);

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

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);

        let current_scope = ctx.call_stack.get_current_scope().ok_or_else(|| {
            let s = format!("Failed to find current scope");
            AstError::EvaluationFailed(s)
        })?;

        let lit = current_scope.resolve_to_lit(&self.key).ok_or_else(|| {
            let s = format!("Failed to resolve symbol '{}'", self.key);
            AstError::EvaluationFailed(s)
        })?;

        let result = lit.evaluate(ctx).ok_or_else(|| {
            let s = format!("Failed to evaluate literal {}", lit.to_string());
            AstError::EvaluationFailed(s)
        })?;

        ctx.hint_stack.pop();
        Ok(result)
    }

    pub fn to_string(&self) -> String {
        format!("InterpolateNode({}, {})", self.key, self.hint.to_string())
    }

    pub fn debug(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}
