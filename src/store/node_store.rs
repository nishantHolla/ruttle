use crate::ast::Node;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

pub struct NodeStore {
    node_list: Vec<Node>,
}
