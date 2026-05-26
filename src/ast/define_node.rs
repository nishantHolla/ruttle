use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use super::node::{AstNode, Node};
use crate::config::{DEFINE_DIRECTIVE_START, DIRECTIVE_END, KV_SPLIT};
use crate::context::Context;
use crate::store::NodeStore;
use crate::util;

pub struct DefineNode {
    key: String,
    value: Literal,
    hint: Hint,
}

impl AstNode for DefineNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);

        ctx.call_stack
            .get_mut_current_scope()
            .ok_or_else(|| {
                let s = format!("Failed to find current scope");
                AstError::EvaluationFailed(s)
            })?
            .set(&self.key, self.value.clone());

        ctx.hint_stack.pop();
        Ok(String::new())
    }

    fn to_string(&self) -> String {
        format!(
            "DefineNode({}, {}, {})",
            self.key,
            self.value.display(),
            self.hint.to_string()
        )
    }

    fn debug(&self, indent: usize, _: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}

impl DefineNode {
    pub fn parse(s: &str, hint: Hint) -> Result<Node, AstError> {
        let s = util::string::normalize_whitespace(s, None);

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

        let value = &value[1..value.len() - 1];
        let value = value.replace("\\\"", "\"");

        let node = Self {
            key: key.to_string(),
            value: Literal::parse(&value),
            hint,
        };

        Ok(Box::new(node))
    }
}
