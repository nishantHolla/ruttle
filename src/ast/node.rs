use super::define_node::DefineNode;
use super::error::AstError;
use super::include_node::IncludeNode;
use super::interpolate_node::InterpolateNode;
use super::root_node::RootNode;
use super::text_node::TextNode;
use super::with_node::WithNode;
use crate::context::Context;
use crate::store::NodeStore;

pub enum Node {
    Root(RootNode),
    Text(TextNode),
    Define(DefineNode),
    Interpolate(InterpolateNode),
    Include(IncludeNode),
    With(WithNode),
}

impl Node {
    pub fn to_string(&self) -> String {
        match self {
            Node::Root(n) => n.to_string(),
            Node::Text(n) => n.to_string(),
            Node::Define(n) => n.to_string(),
            Node::Interpolate(n) => n.to_string(),
            Node::Include(n) => n.to_string(),
            Node::With(n) => n.to_string(),
        }
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        match self {
            Node::Root(n) => n.evaluate(ctx),
            Node::Text(n) => n.evaluate(ctx),
            Node::Define(n) => n.evaluate(ctx),
            Node::Interpolate(n) => n.evaluate(ctx),
            Node::Include(n) => n.evaluate(ctx),
            Node::With(n) => n.evaluate(ctx),
        }
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        match self {
            Node::Root(n) => n.debug(indent, ns),
            Node::Text(n) => n.debug(indent),
            Node::Define(n) => n.debug(indent),
            Node::Interpolate(n) => n.debug(indent),
            Node::Include(n) => n.debug(indent),
            Node::With(n) => n.debug(indent, ns),
        }
    }
}
