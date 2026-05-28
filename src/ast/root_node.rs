use super::error::AstError;
use super::node::{AstNode, Node};
use crate::context::Context;
use crate::store::{NodeId, NodeStore, NodeType};

#[derive(Clone)]
pub struct RootNode {
    children: Vec<NodeId>,
}

impl AstNode for RootNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        let mut result = String::new();

        for node_id in &self.children {
            if ctx.node_store.is_blacklisted(*node_id) {
                continue;
            }

            let node_type = ctx.node_store.get_type(*node_id).ok_or_else(|| {
                let s = format!("Failed to get type of node {:?}", node_id);
                AstError::EvaluationFailed(s)
            })?;

            let node = ctx.node_store.get_clone(*node_id).ok_or_else(|| {
                let s = format!("Failed to find node with id {:?}", node_id);
                AstError::EvaluationFailed(s)
            })?;

            let eval = node.evaluate(ctx)?;
            result.push_str(&eval);

            if matches!(node_type, NodeType::OnceNode) {
                ctx.node_store.add_to_blacklist(*node_id);
            }
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
