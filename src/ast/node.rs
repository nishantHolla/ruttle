use super::define_node::DefineNode;
use super::include_node::IncludeNode;
use super::interpolate_node::InterpolateNode;
use super::root_node::RootNode;
use super::text_node::TextNode;

pub enum Node {
    Root(RootNode),
    Text(TextNode),
    Define(DefineNode),
    Interpolate(InterpolateNode),
    Include(IncludeNode),
}
