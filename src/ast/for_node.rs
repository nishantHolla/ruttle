use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DIRECTIVE_END, FOR_DIRECTIVE_START};
use crate::context::Context;
use crate::store::{FileId, NodeId, NodeStore};
use regex::Regex;
use std::path::PathBuf;

const FOR_DIRECTIVE_REGEX: &str = r"(?<l>\w+)[\s\n]*,[\s\n]*(?<r>\w+)[\s\n]+(?<keyword>\w+)[\s\n]+(?:(?<range>\d+\.\.\d+\.\.\d+)|(?<path>\S+))[\s\n]+(?<body>.*)$";
static FOR_DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(FOR_DIRECTIVE_REGEX).unwrap());

enum ForType {
    Iteration(ForIteration),
    Json(ForJson),
}

impl ForType {
    pub fn to_string(&self) -> String {
        match self {
            ForType::Iteration(_) => String::from("Iteration"),
            ForType::Json(_) => String::from("Json"),
        }
    }
}

pub struct ForIteration {
    start: i64,
    end: i64,
    step: i64,
}

pub struct ForJson {
    file_id: FileId,
}

pub struct ForNode {
    for_type: ForType,
    l_identifier: String,
    r_identifier: String,
    hint: Hint,
    root_node_id: NodeId,
}

impl ForNode {
    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let initial_s = s.to_string();

        let inner = s
            .trim_start_matches(FOR_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        if let Some(mat) = FOR_DIRECTIVE_RE.captures(inner) {
            let l_identifier = mat.name("l").map(|m| m.as_str()).unwrap_or("");
            let r_identifier = mat.name("r").map(|m| m.as_str()).unwrap_or("");
            let keyword = mat.name("keyword").map(|m| m.as_str()).unwrap_or("");
            let range = mat.name("range").map(|m| m.as_str()).unwrap_or("");
            let path = mat.name("path").map(|m| m.as_str()).unwrap_or("");
            let body = mat.name("body").map(|m| m.as_str()).unwrap_or("");

            if l_identifier.is_empty() {
                let s = format!("Failed to find 'first_identifier' of FOR directive");
                return Err(AstError::InvalidSyntax(s));
            }

            if r_identifier.is_empty() {
                let s = format!("Failed to find 'second_identifier' of FOR directive");
                return Err(AstError::InvalidSyntax(s));
            }

            if keyword != "in" {
                let s = format!("Failed to find 'in' keyword of FOR directive");
                return Err(AstError::InvalidSyntax(s));
            }

            let for_type = if !range.is_empty() {
                let mut parts = range.split("..");

                let start = parts.next().ok_or_else(|| {
                    let s = format!("Failed to find 'start' of FOR directive");
                    AstError::InvalidSyntax(s)
                })?;

                let start = start.parse::<i64>().map_err(|e| {
                    let s = format!(
                        "Failed to parse 'start' of FOR directive\n{}",
                        e.to_string()
                    );
                    AstError::InvalidSyntax(s)
                })?;

                let end = parts.next().ok_or_else(|| {
                    let s = format!("Failed to find 'end' of FOR directive");
                    AstError::InvalidSyntax(s)
                })?;

                let end = end.parse::<i64>().map_err(|e| {
                    let s = format!("Failed to parse 'end' of FOR directive\n{}", e.to_string());
                    AstError::InvalidSyntax(s)
                })?;

                let step = parts.next().ok_or_else(|| {
                    let s = format!("Failed to find 'step' of FOR directive");
                    AstError::InvalidSyntax(s)
                })?;

                let step = step.parse::<i64>().map_err(|e| {
                    let s = format!("Failed to parse 'step' of FOR directive\n{}", e.to_string());
                    AstError::InvalidSyntax(s)
                })?;

                Ok(ForType::Iteration(ForIteration { start, end, step }))
            } else if !path.is_empty() {
                let mut for_path = PathBuf::from(path);
                if for_path.is_relative() {
                    let current_path =
                        ctx.file_store.get_by_id(hint.file_id()).ok_or_else(|| {
                            let s = format!("Failed to find path of file id {:?}", hint.file_id());
                            AstError::InvalidSyntax(s)
                        })?;

                    let current_base = current_path.parent().unwrap();
                    for_path = current_base.join(for_path);
                }

                let for_path = for_path.canonicalize().map_err(|e| {
                    let s = format!(
                        "Failed to find 'for' path {} in FOR directive\n{}",
                        for_path.display(),
                        e.to_string()
                    );
                    AstError::InvalidSyntax(s)
                })?;

                let file_id = match ctx.file_store.get_by_path(&for_path) {
                    Some(id) => Ok(id),
                    None => ctx.file_store.add(&for_path).map_err(|e| {
                        let s = format!(
                            "Failed to find 'for' path {} in FOR directive\n{}",
                            for_path.display(),
                            e.to_string()
                        );
                        AstError::InvalidSyntax(s)
                    }),
                }?;
                Ok(ForType::Json(ForJson { file_id }))
            } else {
                let s = format!("Failed to find loop type of FOR directive");
                Err(AstError::InvalidSyntax(s))
            }?;

            if body.is_empty() {
                let s = format!("Failed to find 'body' of the FOR directive");
                return Err(AstError::InvalidSyntax(s));
            }

            let start_pos = initial_s.find(body).unwrap() + hint.start();
            let root_node_id =
                ast::from_file(hint.file_id(), ctx, Some(start_pos), Some(hint.end() - 1))
                    .map_err(|e| {
                        let s =
                            format!("Failed to parse 'body' of FOR directive\n{}", e.to_string());
                        AstError::InvalidSyntax(s)
                    })?;

            return Ok(Node::For(Self {
                for_type,
                hint,
                l_identifier: l_identifier.to_string(),
                r_identifier: r_identifier.to_string(),
                root_node_id,
            }));
        } else {
            let s = format!("Failed to parse FOR directive");
            return Err(AstError::InvalidSyntax(s));
        }
    }

    pub fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        Ok(String::new())
    }

    pub fn to_string(&self) -> String {
        format!(
            "ForNode({}, {}, {}, {})",
            self.l_identifier,
            self.r_identifier,
            self.for_type.to_string(),
            self.hint.to_string()
        )
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        let node = ns.get(self.root_node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}
