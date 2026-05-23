use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::node::Node;
use crate::config::{DIRECTIVE_END, WITH_DIRECTIVE_START};
use crate::context::Context;
use crate::store::{FileId, NodeId, NodeStore};
use crate::util;
use std::path::PathBuf;

pub struct WithNode {
    file_id: FileId,
    root_node_id: NodeId,
    identifier: String,
}

impl WithNode {
    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let s = util::string::normalize_whitespace(s, Some(4));

        let inner = s
            .trim_start_matches(WITH_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        let mut parts = inner.splitn(4, char::is_whitespace);

        let identifier = parts.next().ok_or_else(|| {
            let s = format!("Failed to find 'identifier' for WITH directive");
            AstError::InvalidSyntax(s)
        })?;

        let keyword = parts.next().ok_or_else(|| {
            let s = format!("Failed to find keyword 'as' in WITH directive");
            AstError::InvalidSyntax(s)
        })?;

        if keyword != "as" {
            let s = format!("Failed to find keyword 'as' in WITH directive");
            return Err(AstError::InvalidSyntax(s));
        }

        let with_path = parts.next().ok_or_else(|| {
            let s = format!("Failed to find 'path' for WITH directive");
            AstError::InvalidSyntax(s)
        })?;
        let mut with_path = PathBuf::from(with_path);

        if with_path.is_relative() {
            let current_path = ctx.file_store.get_by_id(hint.file_id()).ok_or_else(|| {
                let s = format!("Failed to find path of file id {:?}", hint.file_id());
                AstError::InvalidSyntax(s)
            })?;

            let current_base = current_path.parent().unwrap();
            with_path = current_base.join(with_path);
        }

        let with_path = with_path.canonicalize().map_err(|e| {
            let s = format!(
                "Failed to find the 'with' path {} in WITH directive\n{}",
                with_path.display(),
                e.to_string()
            );
            AstError::InvalidSyntax(s)
        })?;

        let file_id = match ctx.file_store.get_by_path(&with_path) {
            Some(id) => Ok(id),
            None => ctx.file_store.add(&with_path).map_err(|e| {
                let s = format!(
                    "Failed to find with path {} in WITH directive\n{}",
                    with_path.display(),
                    e.to_string()
                );
                AstError::InvalidSyntax(s)
            }),
        }?;

        let body = parts.next().ok_or_else(|| {
            let s = format!("Failed to find 'body' of WITH directive");
            AstError::InvalidSyntax(s)
        })?;

        let root_node_id = ast::from_string(body, file_id, ctx).map_err(|e| {
            let s = format!(
                "Failed to parse 'body' of WITH directive\n{}",
                e.to_string()
            );
            AstError::InvalidSyntax(s)
        })?;

        Ok(Node::With(Self {
            identifier: identifier.to_string(),
            file_id,
            root_node_id,
        }))
    }

    pub fn to_string(&self) -> String {
        format!(
            "WithNode({}, {:?}, {:?})",
            self.identifier, self.file_id, self.root_node_id
        )
    }

    pub fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        let node = ns.get(self.root_node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}
