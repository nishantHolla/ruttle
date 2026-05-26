use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use crate::context::Context;
use crate::store::{NodeId, NodeStore};
use crate::util;

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

pub enum Comparison {
    Equals,
    NotEquals,
    Less,
    Greater,
    LessOrEquals,
    GreaterOrEquals,
    Unconditional,
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
    pub fn root_node_id(&self) -> NodeId {
        match self {
            Branch::Equals(e) => e.root_node_id(),
            Branch::NotEquals(e) => e.root_node_id(),
            Branch::Less(e) => e.root_node_id(),
            Branch::Greater(e) => e.root_node_id(),
            Branch::LessOrEquals(e) => e.root_node_id(),
            Branch::GreaterOrEquals(e) => e.root_node_id(),
            Branch::Unconditional(e) => e.root_node_id(),
        }
    }

    fn split_comparison(s: &str) -> Option<(&str, &str, &str)> {
        for op in ["==", "!=", "<=", ">=", ">", "<"] {
            if let Some((left, right)) = s.split_once(op) {
                return Some((left, op, right));
            }
        }
        None
    }

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

        if binding.starts_with("#elseif") || !binding.starts_with("#else") {
            let (left, op, right) = Branch::split_comparison(branch_str).ok_or_else(|| {
                let s = format!("Failed to find operator in IF directive");
                AstError::InvalidSyntax(s)
            })?;

            let (right, body) = right.split_once(char::is_whitespace).ok_or_else(|| {
                let s = format!("Failed to find body in IF directive");
                AstError::InvalidSyntax(s)
            })?;

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

            if !util::string::is_valid_quote(left) {
                let s = format!("Unclosed '\"' in 'lhs' of IF directive");
                return Err(AstError::InvalidSyntax(s));
            }

            if !util::string::is_valid_quote(right) {
                let s = format!("Unclosed '\"' in 'right' of IF directive");
                return Err(AstError::InvalidSyntax(s));
            }

            let left = if left.starts_with("\"") {
                let s = left.trim_start_matches("\"").trim_end_matches("\"").trim();
                Literal::parse(s)
            } else {
                let s = format!("{{#value {}}}", left);
                Literal::parse(&s)
            };

            let right = if right.starts_with("\"") {
                let s = right.trim_start_matches("\"").trim_end_matches("\"").trim();
                Literal::parse(s)
            } else {
                let s = format!("{{#value {}}}", right);
                Literal::parse(&s)
            };

            let branch =
                Branch::new_conditional(left, right, root_node_id, op).ok_or_else(|| {
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
        } else {
            let s = format!("Failed to identify type of IF directive");
            return Err(AstError::InvalidSyntax(s));
        }
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

    fn evaluate_conditional(
        &self,
        ctx: &mut Context,
        e: &ConditionalBranch,
    ) -> Result<bool, AstError> {
        let l = e.left().evaluate_to_lit(ctx).ok_or_else(|| {
            let s = format!("Failed to evaluate left literal");
            AstError::EvaluationFailed(s)
        })?;

        let r = e.right().evaluate_to_lit(ctx).ok_or_else(|| {
            let s = format!("Failed to evaluate right literal");
            AstError::EvaluationFailed(s)
        })?;

        let result = match self {
            Branch::Equals(_) => l.compare(&r, Comparison::Equals),
            Branch::NotEquals(_) => l.compare(&r, Comparison::NotEquals),
            Branch::Less(_) => l.compare(&r, Comparison::Less),
            Branch::Greater(_) => l.compare(&r, Comparison::Greater),
            Branch::LessOrEquals(_) => l.compare(&r, Comparison::LessOrEquals),
            Branch::GreaterOrEquals(_) => l.compare(&r, Comparison::GreaterOrEquals),
            Branch::Unconditional(_) => Ok(true),
        };

        result.map_err(|e| {
            let s = format!("Failed to compare condition\n{}", e);
            AstError::EvaluationFailed(s)
        })
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<bool, AstError> {
        match self {
            Branch::Equals(e) => self.evaluate_conditional(ctx, e),
            Branch::NotEquals(e) => self.evaluate_conditional(ctx, e),
            Branch::Less(e) => self.evaluate_conditional(ctx, e),
            Branch::Greater(e) => self.evaluate_conditional(ctx, e),
            Branch::LessOrEquals(e) => self.evaluate_conditional(ctx, e),
            Branch::GreaterOrEquals(e) => self.evaluate_conditional(ctx, e),
            Branch::Unconditional(_) => Ok(true),
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
