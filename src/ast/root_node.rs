use super::node::Node;
use crate::store::{NodeId, NodeStore};

pub struct RootNode {
    children: Vec<NodeId>,
}

impl RootNode {
    pub fn new(children: Vec<NodeId>) -> Node {
        Node::Root(Self { children })
    }

    pub fn to_string(&self) -> String {
        format!("RootNode({:?})", self.children)
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        for node_id in &self.children {
            let node = ns.get(*node_id).unwrap();
            node.debug(indent + 4, ns);
        }
    }
}
