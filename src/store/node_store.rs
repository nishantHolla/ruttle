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
}
