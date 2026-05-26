use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use super::node::Node;
use crate::config::{DIRECTIVE_END, IF_DIRECTIVE_START};
use crate::context::Context;
use crate::store::{NodeId, NodeStore};

#[derive(Debug)]
struct ConditionalBranch {
    left: Literal,
    right: Literal,
    root_node_id: NodeId,
}

impl ConditionalBranch {
    pub fn left(&self) -> &Literal {
        &self.left
    }

    pub fn right(&self) -> &Literal {
        &self.right
    }

    pub fn root_node_id(&self) -> NodeId {
        self.root_node_id
    }
}

#[derive(Debug)]
struct UnconditionalBranch {
    root_node_id: NodeId,
}

impl UnconditionalBranch {
    pub fn root_node_id(&self) -> NodeId {
        self.root_node_id
    }
}

#[derive(Debug)]
enum Branch {
    Equals(ConditionalBranch),
    NotEquals(ConditionalBranch),
    Less(ConditionalBranch),
    Greater(ConditionalBranch),
    LessOrEquals(ConditionalBranch),
    GreaterOrEquals(ConditionalBranch),
    Unconditional(UnconditionalBranch),
}

impl Branch {
    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let (left, right, node_id) = match self {
            Branch::Equals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::NotEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Less(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Greater(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::LessOrEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::GreaterOrEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Unconditional(e) => (None, None, e.root_node_id()),
        };

        println!(
            "{:?}({}, {})",
            self,
            left.unwrap().to_string(),
            right.unwrap().to_string()
        );
        let node = ns.get(node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}

pub struct IfNode {
    hint: Hint,
    branches: Vec<Branch>,
}

impl IfNode {
    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let initial_s = s.to_string();

        let inner = s
            .trim_start_matches(IF_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();
        unimplemented!("Not implemented yet");
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        unimplemented!("Not implemented yet");
    }

    pub fn to_string(&self) -> String {
        format!("IfNode({}, {})", self.branches.len(), self.hint.to_string())
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        for branch in &self.branches {
            branch.debug(indent + 4, ns);
        }
    }
}
