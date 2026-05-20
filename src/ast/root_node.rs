use super::node::Node;
use crate::store::NodeId;

pub struct RootNode {
    children: Vec<NodeId>,
}

impl RootNode {
    pub fn new(children: Vec<NodeId>) -> Node {
        Node::Root(Self { children })
    }
}
