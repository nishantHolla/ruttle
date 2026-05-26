use super::error::AstError;
use super::node::{AstNode, Node};
use crate::context::Context;
use crate::store::{NodeId, NodeStore};

pub struct RootNode {
    children: Vec<NodeId>,
}

impl AstNode for RootNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        let mut result = String::new();

        for node_id in &self.children {
            let node = ctx.node_store.take(*node_id).ok_or_else(|| {
                let s = format!("Failed to find node with id {:?}", node_id);
                AstError::EvaluationFailed(s)
            })?;

            let eval = node.evaluate(ctx)?;
            result.push_str(&eval);

            ctx.node_store.put_back(*node_id, node);
        }

        Ok(result)
    }

    fn to_string(&self) -> String {
        format!("RootNode({:?})", self.children)
    }

    fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        for node_id in &self.children {
            let node = ns.get(*node_id).unwrap();
            node.debug(indent + 4, ns);
        }
    }
}

impl RootNode {
    pub fn new(children: Vec<NodeId>) -> Node {
        let node = Self { children };
        Box::new(node)
    }
}
