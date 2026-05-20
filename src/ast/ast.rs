use super::error::AstError;
use super::hint::Hint;
use crate::config;
use crate::store::{FileId, FileStore, NodeId, NodeStore};
use crate::util;
use regex::Regex;
use std::path::Path;

use super::define_node::DefineNode;
use super::include_node::IncludeNode;
use super::interpolate_node::InterpolateNode;
use super::node::Node;
use super::root_node::RootNode;
use super::text_node::TextNode;

fn parse_directive(s: &str, hint: Hint, fs: &mut FileStore) -> Result<Node, AstError> {
    if s.starts_with(config::DEFINE_DIRECTIVE_START) {
        DefineNode::parse(s, hint)
    } else if s.starts_with(config::INTERPOLATE_DIRECTIVE_START) {
        InterpolateNode::parse(s, hint)
    } else if s.starts_with(config::INCLUDE_DIRECTIVE_START) {
        IncludeNode::parse(s, hint, fs)
    } else {
        let s = format!("Found unknown directive {}", s);
        Err(AstError::UnknownDirective(s))
    }
}

static DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(config::DIRECTIVE_REGEX).unwrap());

fn parse(
    input: &str,
    file_id: FileId,
    fs: &mut FileStore,
    ns: &mut NodeStore,
) -> Result<NodeId, AstError> {
    let mut nodes: Vec<NodeId> = Vec::new();
    let mut cursor = 0;

    while let Some(mat) = DIRECTIVE_RE.find_at(input, cursor) {
        // Emit text before directive
        if mat.start() > cursor {
            let text = &input[cursor..mat.start()];

            if text.trim().len() > 0 {
                let hint = Hint::new(file_id, cursor, mat.start() - 1);
                let node = TextNode::parse(hint)?;
                let node_id = ns.add(node);
                nodes.push(node_id);
            }
        }

        // Find directive end
        let end = util::string::find_directive_end(&input, mat.start()).ok_or_else(|| {
            let s = format!("Failed to find directive end");
            AstError::UnclosedDirective(s)
        })?;

        let directive_str = &input[mat.start()..=end];

        // Parse directive
        let hint = Hint::new(file_id, mat.start(), end);
        let node = parse_directive(directive_str, hint, fs)?;
        let node_id = ns.add(node);
        nodes.push(node_id);

        // Advance cursor
        cursor = end + 1;
    }

    // Remaining text
    if cursor < input.len() {
        let text = &input[cursor..];

        if text.trim().len() > 0 {
            let hint = Hint::new(file_id, cursor, input.len() - 1);
            let node = TextNode::parse(hint)?;
            let node_id = ns.add(node);
            nodes.push(node_id);
        }
    }

    // Create the root and return it
    let root = RootNode::new(nodes);
    let root_node_id = ns.add(root);
    Ok(root_node_id)
}

pub fn from_file(
    file_id: FileId,
    fs: &mut FileStore,
    ns: &mut NodeStore,
) -> Result<NodeId, AstError> {
    let path = fs.get_by_id(file_id).ok_or_else(|| {
        let s = format!(
            "Failed to find file for AST construction with file id {:?}",
            file_id
        );
        AstError::FileNotFound(s)
    })?;

    if !path.is_file() {
        let s = format!(
            "Failed to construct AST for path non file {}",
            path.display()
        );
        return Err(AstError::FileNotFound(s));
    }

    let content = std::fs::read_to_string(path).map_err(|e| {
        let s = format!(
            "Failed to read file {} for AST construction\n{}",
            path.display(),
            e.to_string()
        );
        AstError::ConstructionFailed(s)
    })?;

    parse(&content, file_id, fs, ns)
}
