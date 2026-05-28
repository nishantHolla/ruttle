use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::node::{AstNode, Node};
use crate::config::{DIRECTIVE_END, ONCE_DIRECTIVE_START};
use crate::context::Context;
use crate::store::{NodeId, NodeStore};

#[derive(Clone)]
pub struct OnceNode {
    root_node_id: NodeId,
    hint: Hint,
}

impl AstNode for OnceNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        let node = ctx.node_store.get_clone(self.root_node_id).ok_or_else(|| {
            let s = format!("Failed to find node with id {:?}", self.root_node_id);
            AstError::EvaluationFailed(s)
        })?;

        let result = node.evaluate(ctx)?;
        Ok(result)
    }

    fn to_string(&self) -> String {
        format!(
            "OnceNode({:?}, {})",
            self.root_node_id,
            self.hint.to_string()
        )
    }

    fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        let node = ns.get(self.root_node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}

impl OnceNode {
    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let inner = s
            .trim_start_matches(ONCE_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        let start_pos = s.find(inner).unwrap() + hint.start();
        let root_node_id =
            ast::from_file(hint.file_id(), ctx, Some(start_pos), Some(hint.end() - 1)).map_err(
                |e| {
                    let s = format!(
                        "Failed to parse 'body' of ONCE directive\n{}",
                        e.to_string()
                    );
                    AstError::InvalidSyntax(s)
                },
            )?;

        let node = Self { root_node_id, hint };
        Ok(Box::new(node))
    }
}
