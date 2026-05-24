use super::error::AstError;
use super::hint::Hint;
use crate::config;
use crate::context::Context;
use crate::store::{FileId, NodeId};
use crate::util;
use regex::Regex;

use super::define_node::DefineNode;
use super::include_node::IncludeNode;
use super::interpolate_node::InterpolateNode;
use super::node::Node;
use super::root_node::RootNode;
use super::text_node::TextNode;
use super::with_node::WithNode;

static DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(config::DIRECTIVE_REGEX).unwrap());

fn parse_directive(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
    ctx.hint_stack.push(hint);
    let r: Result<Node, AstError>;

    if s.starts_with(config::DEFINE_DIRECTIVE_START) {
        r = DefineNode::parse(&s, hint);
    } else if s.starts_with(config::INTERPOLATE_DIRECTIVE_START) {
        r = InterpolateNode::parse(&s, hint);
    } else if s.starts_with(config::INCLUDE_DIRECTIVE_START) {
        r = IncludeNode::parse(&s, hint, ctx);
    } else if s.starts_with(config::WITH_DIRECTIVE_START) {
        r = WithNode::parse(&s, hint, ctx);
    } else {
        let s = format!("Found unknown directive {}", s);
        r = Err(AstError::UnknownDirective(s));
    }

    if let Ok(_) = r {
        ctx.hint_stack.pop();
    }

    return r;
}

fn parse(
    input: &str,
    file_id: FileId,
    ctx: &mut Context,
    start_pos: Option<usize>,
    end_pos: Option<usize>,
) -> Result<NodeId, AstError> {
    let mut nodes: Vec<NodeId> = Vec::new();
    let mut cursor = start_pos.unwrap_or(0);
    let end_pos = end_pos.unwrap_or(input.len() - 1);

    while cursor <= end_pos
        && let Some(mat) = DIRECTIVE_RE.find_at(input, cursor)
    {
        // Check if end is reached
        if mat.start() > end_pos {
            break;
        }

        // Emit text before directive
        if mat.start() > cursor {
            let text = &input[cursor..mat.start()];

            if text.trim().len() > 0 {
                let hint = Hint::new(file_id, cursor, mat.start() - 1);
                let node = TextNode::parse(hint)?;
                let node_id = ctx.node_store.add(node);
                nodes.push(node_id);
            }
        }

        // Find directive end
        let end = util::string::find_directive_end(&input, mat.start()).ok_or_else(|| {
            if let Some(end) = util::string::get_line_end_index(&input, mat.start()) {
                ctx.hint_stack.push(Hint::new(file_id, mat.start(), end));
            }

            let s = format!("Failed to find directive end");
            AstError::UnclosedDirective(s)
        })?;

        let directive_str = &input[mat.start()..=end];

        // Parse directive
        let hint = Hint::new(file_id, mat.start(), end);
        let node = parse_directive(directive_str, hint, ctx)?;
        let node_id = ctx.node_store.add(node);
        nodes.push(node_id);

        // Advance cursor
        cursor = end + 1;
    }

    // Remaining text
    if cursor < end_pos {
        let text = &input[cursor..=end_pos];

        if text.trim().len() > 0 {
            let hint = Hint::new(file_id, cursor, end_pos);
            let node = TextNode::parse(hint)?;
            let node_id = ctx.node_store.add(node);
            nodes.push(node_id);
        }
    }

    // Create the root and return it
    let root = RootNode::new(nodes);
    let root_node_id = ctx.node_store.add(root);
    Ok(root_node_id)
}

pub fn from_file(
    file_id: FileId,
    ctx: &mut Context,
    start_pos: Option<usize>,
    end_pos: Option<usize>,
) -> Result<NodeId, AstError> {
    let path = ctx.file_store.get_by_id(file_id).ok_or_else(|| {
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

    parse(&content, file_id, ctx, start_pos, end_pos)
}
