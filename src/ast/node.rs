use super::error::AstError;
use crate::context::Context;
use crate::store::NodeStore;

pub type Node = Box<dyn AstNode>;

pub trait AstNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError>;
    fn debug(&self, indent: usize, ns: &NodeStore);
    fn to_string(&self) -> String;
}
