use crate::ast::Node;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

pub struct NodeStore {
    node_list: Vec<Node>,
}

impl NodeStore {
    pub fn new() -> Self {
        Self {
            node_list: Vec::new(),
        }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.node_list.len());
        self.node_list.push(node);
        return id;
    }
}
