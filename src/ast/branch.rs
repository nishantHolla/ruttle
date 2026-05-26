use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use crate::config;
use crate::context::Context;
use crate::store::{NodeId, NodeStore};

#[derive(Debug)]
pub struct ConditionalBranch {
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
pub struct UnconditionalBranch {
    root_node_id: NodeId,
}

impl UnconditionalBranch {
    pub fn root_node_id(&self) -> NodeId {
        self.root_node_id
    }
}

#[derive(Debug)]
pub enum Branch {
    Equals(ConditionalBranch),
    NotEquals(ConditionalBranch),
    Less(ConditionalBranch),
    Greater(ConditionalBranch),
    LessOrEquals(ConditionalBranch),
    GreaterOrEquals(ConditionalBranch),
    Unconditional(UnconditionalBranch),
}

impl Branch {
    pub fn parse(
        branch_str: &str,
        initial_s: &str,
        hint: Hint,
        ctx: &mut Context,
    ) -> Result<Self, AstError> {
        let binding = branch_str.to_string();
        let branch_str = binding
            .trim_start_matches("#elseif")
            .trim_start_matches("#else")
            .trim();

        if let Some(mat) = config::IF_DIRECTIVE_RE.captures(branch_str) {
            let l_identifier = mat.name("l").map(|m| m.as_str()).unwrap_or("");
            let condition = mat.name("c").map(|m| m.as_str()).unwrap_or("");
            let r_identifier = mat.name("r").map(|m| m.as_str()).unwrap_or("");
            let body = mat.name("b").map(|m| m.as_str()).unwrap_or("");

            if l_identifier.is_empty() {
                let s = format!("Failed to find 'lhs' of IF directive in {}", branch_str);
                return Err(AstError::InvalidSyntax(s));
            }

            if condition.is_empty() {
                let s = format!(
                    "Failed to find 'condition' of IF directive in {}",
                    branch_str
                );
                return Err(AstError::InvalidSyntax(s));
            }

            if r_identifier.is_empty() {
                let s = format!("Failed to find 'rhs' of IF directive in {}", branch_str);
                return Err(AstError::InvalidSyntax(s));
            }

            if body.is_empty() {
                let s = format!("Failed to find body of IF directive, in {}", branch_str);
                return Err(AstError::InvalidSyntax(s));
            }

            let l_lit = Literal::String(l_identifier.to_string());
            let r_lit = Literal::String(r_identifier.to_string());

            let start_pos = initial_s.find(body).unwrap() + hint.start();
            let root_node_id = ast::from_file(
                hint.file_id(),
                ctx,
                Some(start_pos),
                Some(start_pos + body.len()),
            )
            .map_err(|e| {
                let s = format!(
                    "Failed to parse body of IF directive in {}\n{}",
                    branch_str,
                    e.to_string()
                );
                AstError::InvalidSyntax(s)
            })?;

            let branch = Branch::new_conditional(l_lit, r_lit, root_node_id, condition)
                .ok_or_else(|| {
                    let s = format!("Failed to parse branch of IF directive in {}", branch_str);
                    AstError::InvalidSyntax(s)
                })?;

            return Ok(branch);
        } else if binding.starts_with("#else") {
            let start_pos = initial_s.find(branch_str).unwrap() + hint.start();
            let root_node_id = ast::from_file(
                hint.file_id(),
                ctx,
                Some(start_pos),
                Some(start_pos + branch_str.len()),
            )
            .map_err(|e| {
                let s = format!(
                    "Failed to parse body of IF directive in {}\n{}",
                    branch_str,
                    e.to_string()
                );
                AstError::InvalidSyntax(s)
            })?;

            let branch = Branch::new_unconditional(root_node_id);
            return Ok(branch);
        }

        let s = format!("Failed to parse branch of IF directive in {}", branch_str);
        return Err(AstError::InvalidSyntax(s));
    }

    pub fn new_unconditional(root_node_id: NodeId) -> Self {
        Self::Unconditional(UnconditionalBranch { root_node_id })
    }

    pub fn new_conditional(
        left: Literal,
        right: Literal,
        root_node_id: NodeId,
        c: &str,
    ) -> Option<Self> {
        match c {
            "==" => Some(Self::Equals(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            "!=" => Some(Self::NotEquals(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            "<" => Some(Self::Less(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            ">" => Some(Self::Greater(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            "<=" => Some(Self::LessOrEquals(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            ">=" => Some(Self::GreaterOrEquals(ConditionalBranch {
                left,
                right,
                root_node_id,
            })),
            _ => None,
        }
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);

        let (left, right, node_id) = match self {
            Branch::Equals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::NotEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Less(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Greater(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::LessOrEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::GreaterOrEquals(e) => (Some(e.left()), Some(e.right()), e.root_node_id()),
            Branch::Unconditional(e) => (None, None, e.root_node_id()),
        };

        let left = left.map(|m| m.to_string()).unwrap_or(String::new());
        let right = right.map(|m| m.to_string()).unwrap_or(String::new());

        if !left.is_empty() || !right.is_empty() {
            println!(
                "{}Conditional({}, {}, {:?})",
                indent_str, left, right, node_id
            );
        } else {
            println!("{}Unconditional({:?})", indent_str, node_id);
        }
        let node = ns.get(node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}
