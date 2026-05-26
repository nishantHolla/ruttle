use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use super::node::{AstNode, Node};
use crate::config::{DIRECTIVE_END, INCLUDE_DIRECTIVE_START, KV_SPLIT};
use crate::context::Context;
use crate::store::{FileId, NodeStore};
use crate::util;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct IncludeNode {
    file_id: FileId,
    props: BTreeMap<String, Literal>,
    hint: Hint,
}

impl AstNode for IncludeNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);

        let mut props: BTreeMap<String, Literal> = BTreeMap::new();

        for (k, v) in &self.props {
            let value = v.evaluate(ctx).ok_or_else(|| {
                let s = format!("Failed to evaluate literal {}", v.to_string());
                AstError::EvaluationFailed(s)
            })?;
            props.insert(k.clone(), Literal::String(value));
        }

        ctx.call_stack
            .push(self.file_id, Some(props))
            .map_err(|e| {
                let path = ctx.file_store.get_by_id(self.file_id).unwrap();
                let s = format!(
                    "Recursive include detected in path {}\n{}",
                    path.display(),
                    e.to_string()
                );
                AstError::EvaluationFailed(s)
            })?;

        let result = ctx.generate(self.file_id).map_err(|e| {
            let s = format!("Failed to evaluate included path\n{}", e.to_string());
            AstError::EvaluationFailed(s)
        })?;

        let result = result.trim().to_string();

        ctx.call_stack.pop();
        ctx.hint_stack.pop();
        Ok(result)
    }

    fn to_string(&self) -> String {
        let mut props_str = String::new();

        let mut counter = 1;
        for (k, v) in &self.props {
            props_str.push_str(&format!("{}={}", k, v.display()));

            if counter != self.props.len() {
                props_str.push_str(" ");
            }

            counter += 1;
        }

        format!(
            "IncludeNode({:?}, {}, {})",
            self.file_id,
            props_str,
            self.hint.to_string()
        )
    }

    fn debug(&self, indent: usize, _: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());
    }
}

impl IncludeNode {
    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let s = util::string::normalize_whitespace(s, None);

        let inner = s
            .trim_start_matches(INCLUDE_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        let mut parts = util::string::split_quoted(inner);

        let include_path = parts.next().ok_or_else(|| {
            let s = format!("Failed to find 'path' for INCLUDE directive");
            AstError::InvalidSyntax(s)
        })?;
        let mut include_path = PathBuf::from(include_path);

        if include_path.is_relative() {
            let current_path = ctx.file_store.get_by_id(hint.file_id()).ok_or_else(|| {
                let s = format!("Failed to find path of file id {:?}", hint.file_id());
                AstError::InvalidSyntax(s)
            })?;

            let current_base = current_path.parent().unwrap();
            include_path = current_base.join(include_path);
        }

        let include_path = include_path.canonicalize().map_err(|e| {
            let s = format!(
                "Failed to find the include path {} in INCLUDE directive\n{}",
                include_path.display(),
                e.to_string()
            );
            AstError::InvalidSyntax(s)
        })?;

        let file_id = match ctx.file_store.get_by_path(&include_path) {
            Some(id) => Ok(id),
            None => ctx.file_store.add(&include_path).map_err(|e| {
                let s = format!(
                    "Failed to find the include path {} in INCLUDE directive\n{}",
                    include_path.display(),
                    e.to_string()
                );
                AstError::InvalidSyntax(s)
            }),
        }?;

        if !ctx.ast_map.has_ast_for(file_id) {
            ctx.ast_map.add_todo(file_id);
        }

        let mut props: BTreeMap<String, Literal> = BTreeMap::new();
        for part in parts {
            let mut kv = part.split(KV_SPLIT);

            let key = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'key' of prop in INCLUDE directive");
                AstError::InvalidSyntax(s)
            })?;

            let value = kv.next().ok_or_else(|| {
                let s = format!("Failed to find 'value' of prop in INCLUDE diretive");
                AstError::InvalidSyntax(s)
            })?;

            if !value.starts_with('"') || !value.ends_with('"') {
                let s = format!("'value' of INCLUDE directive is not wrapped with double quotes");
                return Err(AstError::InvalidSyntax(s));
            }

            let value = value.trim_matches('"');

            props.insert(key.to_string(), Literal::parse(value));
        }

        let node = Self {
            file_id,
            props,
            hint,
        };

        Ok(Box::new(node))
    }
}
