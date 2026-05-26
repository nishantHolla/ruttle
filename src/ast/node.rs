use super::error::AstError;
use crate::context::Context;
use crate::store::NodeStore;

pub type Node = Box<dyn AstNode>;

pub trait AstNodeClone {
    fn clone_box(&self) -> Box<dyn AstNode>;
}

impl<T> AstNodeClone for T
where
    T: 'static + AstNode + Clone,
{
    fn clone_box(&self) -> Box<dyn AstNode> {
        Box::new(self.clone())
    }
}

pub trait AstNode: AstNodeClone {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError>;
    fn debug(&self, indent: usize, ns: &NodeStore);
    fn to_string(&self) -> String;
}

impl Clone for Box<dyn AstNode> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
