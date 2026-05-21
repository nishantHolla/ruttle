use super::define_node::DefineNode;
use super::include_node::IncludeNode;
use super::interpolate_node::InterpolateNode;
use super::root_node::RootNode;
use super::text_node::TextNode;
use crate::store::NodeStore;

pub enum Node {
    Root(RootNode),
    Text(TextNode),
    Define(DefineNode),
    Interpolate(InterpolateNode),
    Include(IncludeNode),
}

impl Node {
    pub fn to_string(&self) -> String {
        match self {
            Node::Root(n) => n.to_string(),
            Node::Text(n) => n.to_string(),
            Node::Define(n) => n.to_string(),
            Node::Interpolate(n) => n.to_string(),
            Node::Include(n) => n.to_string(),
        }
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        match self {
            Node::Root(n) => n.debug(indent, ns),
            Node::Text(n) => n.debug(indent),
            Node::Define(n) => n.debug(indent),
            Node::Interpolate(n) => n.debug(indent),
            Node::Include(n) => n.debug(indent),
        }
    }
}
