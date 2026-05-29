use super::error::AstError;
use super::hint::Hint;
use super::node::{AstNode, Node};
use crate::config::{DIRECTIVE_END, INTERPOLATE_DIRECTIVE_START};
use crate::context::Context;
use crate::store::NodeStore;
use crate::util;

#[derive(Clone)]
pub struct InterpolateNode {
    key: String,
    hint: Hint,
}

impl AstNode for InterpolateNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);

        let current_frame = ctx.call_stack.get_current_frame().ok_or_else(|| {
            let s = format!("Failed to find current frame");
            AstError::EvaluationFailed(s)
        })?;

        let lit = current_frame.resolve_to_lit(&self.key).ok_or_else(|| {
            let s = format!("Failed to resolve symbol '{}'", self.key);
            AstError::EvaluationFailed(s)
        })?;

        let result = if self.is_content_string()
            && let Some(file_id) = current_frame.get_open_file_id(&self.key)
        {
            if !ctx.ast_map.has_ast_for(file_id) {
                let s = format!("Failed to find ast of file id {:?}", file_id);
                return Err(AstError::EvaluationFailed(s));
            }

            let root_node_id = ctx.ast_map.get(file_id).unwrap();
            let root_node = ctx.node_store.get_clone(root_node_id).unwrap();

            let result = root_node.evaluate(ctx).map_err(|e| {
                let s = format!("Failed to evaluate open file\n{}", e.to_string());
                AstError::EvaluationFailed(s)
            })?;

            util::string::remove_frontmatter(&result).to_string()
        } else {
            lit.evaluate(ctx).ok_or_else(|| {
                let s = format!("Failed to evaluate literal {}", lit.to_string());
                AstError::EvaluationFailed(s)
            })?
        };

        ctx.hint_stack.pop();
        Ok(result)
    }

    fn to_string(&self) -> String {
        format!("InterpolateNode({}, {})", self.key, self.hint.to_string())
    }

    fn debug(&self, indent: usize, _: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
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

        let node = Self {
            key: inner.to_string(),
            hint,
        };

        Ok(Box::new(node))
    }

    pub fn is_content_string(&self) -> bool {
        let parts: Vec<&str> = self.key.split('.').collect();
        parts.len() == 2 && *parts.last().unwrap() == "content"
    }
}
