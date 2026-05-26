use super::ast;
use super::branch::Branch;
use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use super::node::Node;
use crate::config::{self, DIRECTIVE_END, IF_DIRECTIVE_START};
use crate::context::Context;
use crate::store::NodeStore;

pub struct IfNode {
    hint: Hint,
    branches: Vec<Branch>,
}

impl IfNode {
    fn split_directive(input: &str) -> Result<Vec<&str>, String> {
        let mut parts = Vec::new();
        let mut in_quotes = false;
        let mut seen_else = false;
        let mut start = 0;

        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'"' => {
                    in_quotes = !in_quotes;
                    i += 1;
                }

                b'#' if !in_quotes => {
                    if input[i..].starts_with("#elseif") {
                        if seen_else {
                            let s = format!("#elseif appears after #else in IF directive");
                            return Err(s);
                        }

                        parts.push(input[start..i].trim());
                        start = i;
                        i += "#elseif".len();
                    } else if input[i..].starts_with("#else") {
                        if seen_else {
                            let s = format!("Multiple #else in IF directive");
                            return Err(s);
                        }

                        seen_else = true;
                        parts.push(input[start..i].trim());
                        start = i;
                        i += "#else".len();
                    } else {
                        i += 1;
                    }
                }

                _ => i += 1,
            }
        }

        parts.push(input[start..].trim());

        Ok(parts)
    }

    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let initial_s = s.to_string();

        let inner = s
            .trim_start_matches(IF_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        let parts = IfNode::split_directive(inner).map_err(|e| {
            let s = format!("Failed to parse IF directive\n{}", e);
            AstError::InvalidSyntax(s)
        })?;

        if parts.is_empty() {
            let s = format!("Failed to identify branches of the IF directive");
            return Err(AstError::InvalidSyntax(s));
        }

        let mut branches: Vec<Branch> = Vec::new();
        for &part in parts.iter() {
            let branch = Branch::parse(part, &initial_s, hint, ctx)?;
            branches.push(branch)
        }

        Ok(Node::If(Self { branches, hint }))
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        Ok(String::new())
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
