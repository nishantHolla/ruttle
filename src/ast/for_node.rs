use super::ast;
use super::error::AstError;
use super::hint::Hint;
use super::literal::Literal;
use super::node::{AstNode, Node};
use crate::config::{self, DIRECTIVE_END, FOR_DIRECTIVE_START};
use crate::context::Context;
use crate::store::{FileId, NodeId, NodeStore};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Clone)]
enum ForType {
    Iteration(ForIteration),
    Json(ForJson),
    Collection(ForCollection),
}

impl ForType {
    pub fn to_string(&self) -> String {
        match self {
            ForType::Iteration(i) => format!("Iteration({}, {}, {})", i.start(), i.end(), i.step()),
            ForType::Json(j) => format!("Json({:?})", j.file_id()),
            ForType::Collection(c) => format!("Collection({:?})", c.identifier()),
        }
    }
}

#[derive(Clone)]
pub struct ForIteration {
    start: i64,
    end: i64,
    step: i64,
}

impl ForIteration {
    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.end
    }

    pub fn step(&self) -> i64 {
        self.step
    }
}

#[derive(Clone)]
pub struct ForJson {
    file_id: FileId,
}

impl ForJson {
    pub fn file_id(&self) -> FileId {
        self.file_id
    }
}

#[derive(Clone)]
pub struct ForCollection {
    identifier: String,
}

impl ForCollection {
    pub fn identifier(&self) -> String {
        self.identifier.clone()
    }
}

#[derive(Clone)]
pub struct ForNode {
    for_type: ForType,
    l_identifier: String,
    r_identifier: String,
    hint: Hint,
    root_node_id: NodeId,
}

impl AstNode for ForNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String, AstError> {
        ctx.hint_stack.push(self.hint);
        ctx.call_stack
            .get_mut_current_frame()
            .ok_or_else(|| {
                let s = format!("Failed to find current frame");
                AstError::EvaluationFailed(s)
            })?
            .enter_new_scope();

        let root_node_id = self.root_node_id;
        let root = ctx.node_store.get_clone(root_node_id).ok_or_else(|| {
            let s = format!("Failed to find node with id {:?}", root_node_id);
            AstError::EvaluationFailed(s)
        })?;

        let r = match &self.for_type {
            ForType::Iteration(i) => ForNode::evaluate_iteration(&self, i, &root, ctx),
            ForType::Json(j) => ForNode::evaluate_json(&self, j, &root, ctx),
            ForType::Collection(c) => ForNode::evaluate_collection(&self, c, &root, ctx),
        }?;

        ctx.call_stack
            .get_mut_current_frame()
            .ok_or_else(|| {
                let s = format!("Failed to find current frame");
                AstError::EvaluationFailed(s)
            })?
            .exit_current_scope();
        ctx.hint_stack.pop();
        Ok(r)
    }

    fn to_string(&self) -> String {
        format!(
            "ForNode({}, {}, {}, {})",
            self.l_identifier,
            self.r_identifier,
            self.for_type.to_string(),
            self.hint.to_string()
        )
    }

    fn debug(&self, indent: usize, ns: &NodeStore) {
        let indent_str = " ".repeat(indent);
        println!("{}{}", indent_str, self.to_string());

        let node = ns.get(self.root_node_id).unwrap();
        node.debug(indent + 4, ns);
    }
}

impl ForNode {
    pub fn l_identifier(&self) -> &str {
        &self.l_identifier
    }

    pub fn r_identifier(&self) -> &str {
        &self.r_identifier
    }

    pub fn parse(s: &str, hint: Hint, ctx: &mut Context) -> Result<Node, AstError> {
        let initial_s = s.to_string();

        let inner = s
            .trim_start_matches(FOR_DIRECTIVE_START)
            .trim_end_matches(DIRECTIVE_END)
            .trim();

        if let Some(mat) = config::FOR_DIRECTIVE_RE.captures(inner) {
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
            } else if !path.is_empty() && path.contains(".") && !path.contains("/") {
                Ok(ForType::Collection(ForCollection {
                    identifier: path.to_string(),
                }))
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

            let node = Self {
                for_type,
                hint,
                l_identifier: l_identifier.to_string(),
                r_identifier: r_identifier.to_string(),
                root_node_id,
            };

            return Ok(Box::new(node));
        } else {
            let s = format!("Failed to parse FOR directive");
            return Err(AstError::InvalidSyntax(s));
        }
    }

    fn evaluate_iteration(
        for_node: &ForNode,
        node: &ForIteration,
        root: &Node,
        ctx: &mut Context,
    ) -> Result<String, AstError> {
        if (node.start() < node.end() && node.step() < 0)
            || (node.start() > node.end() && node.step() > 0)
            || node.step == 0
        {
            let s = format!(
                "Infinite loop detected: start value of {} will never meet end value of {} with step size of {}",
                node.start(),
                node.end(),
                node.step()
            );
            return Err(AstError::EvaluationFailed(s));
        }

        let mut l_counter = 0;
        let mut r_counter = node.start();
        let mut result = String::new();

        while (node.step() > 0 && r_counter < node.end())
            || (node.step < 0 && r_counter > node.end())
        {
            ctx.call_stack
                .get_mut_current_scope()
                .ok_or_else(|| {
                    let s = format!("Failed to find current scope");
                    AstError::EvaluationFailed(s)
                })?
                .set(for_node.l_identifier(), Literal::Integer(l_counter));

            ctx.call_stack
                .get_mut_current_scope()
                .ok_or_else(|| {
                    let s = format!("Failed to find current scope");
                    AstError::EvaluationFailed(s)
                })?
                .set(for_node.r_identifier(), Literal::Integer(r_counter));

            let iteration_result = root.evaluate(ctx).map_err(|e| {
                let s = format!("Failed to evaluate FOR directive\n{}", e.to_string());
                AstError::EvaluationFailed(s)
            })?;
            result.push_str(&iteration_result);

            r_counter += node.step();
            l_counter += 1;
        }

        Ok(result)
    }

    fn evaluate_json(
        for_node: &ForNode,
        node: &ForJson,
        root: &Node,
        ctx: &mut Context,
    ) -> Result<String, AstError> {
        let path = ctx.file_store.get_by_id(node.file_id()).ok_or_else(|| {
            let s = format!("Failed to find path for file id {:?}", node.file_id());
            AstError::EvaluationFailed(s)
        })?;

        let content = std::fs::read_to_string(path).map_err(|e| {
            let s = format!(
                "Failed to read json file {}\n{}",
                path.display(),
                e.to_string()
            );
            AstError::EvaluationFailed(s)
        })?;

        let json: Value = serde_json::from_str(&content).map_err(|e| {
            let s = format!(
                "Failed to parse json at path {}\n{}",
                path.display(),
                e.to_string()
            );
            AstError::EvaluationFailed(s)
        })?;

        let mut result = String::new();

        match &json {
            Value::Array(arr) => {
                for (i, j) in arr.iter().enumerate() {
                    ctx.call_stack
                        .get_mut_current_scope()
                        .ok_or_else(|| {
                            let s = format!("Failed to find current scope");
                            AstError::EvaluationFailed(s)
                        })?
                        .set(
                            for_node.l_identifier(),
                            Literal::Integer(i64::try_from(i).unwrap()),
                        );

                    match j {
                        Value::String(value) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(value.clone()));
                        }

                        Value::Number(value) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(value.to_string()));
                        }

                        Value::Bool(b) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(b.to_string()));
                        }

                        Value::Object(_) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .open_pseudo(for_node.r_identifier(), j)
                                .map_err(|e| AstError::EvaluationFailed(e.to_string()))?;
                        }

                        Value::Array(_) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .open_pseudo(for_node.r_identifier(), j)
                                .map_err(|e| AstError::EvaluationFailed(e.to_string()))?;
                        }

                        _ => {
                            let s = format!("Unsupported json format");
                            return Err(AstError::EvaluationFailed(s));
                        }
                    }

                    let iteration_result = root.evaluate(ctx).map_err(|e| {
                        let s = format!("Failed to evaluate FOR directive\n{}", e.to_string());
                        AstError::EvaluationFailed(s)
                    })?;

                    result.push_str(&iteration_result);
                }
            }

            Value::Object(obj) => {
                for (i, j) in obj.iter() {
                    ctx.call_stack
                        .get_mut_current_scope()
                        .ok_or_else(|| {
                            let s = format!("Failed to find current scope");
                            AstError::EvaluationFailed(s)
                        })?
                        .set(for_node.l_identifier(), Literal::String(i.clone()));

                    match j {
                        Value::String(s) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(s.clone()));
                        }

                        Value::Number(i) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(i.to_string()));
                        }

                        Value::Bool(b) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(b.to_string()));
                        }

                        _ => {
                            let s = format!("Unsupported json format");
                            return Err(AstError::EvaluationFailed(s));
                        }
                    }

                    let iteration_result = root.evaluate(ctx).map_err(|e| {
                        let s = format!("Failed to evaluate FOR directive\n{}", e.to_string());
                        AstError::EvaluationFailed(s)
                    })?;

                    result.push_str(&iteration_result);
                }
            }

            _ => {
                let s = format!("Unknown json value at {}", path.display());
                return Err(AstError::EvaluationFailed(s));
            }
        }

        Ok(result)
    }

    fn evaluate_collection(
        for_node: &ForNode,
        node: &ForCollection,
        root: &Node,
        ctx: &mut Context,
    ) -> Result<String, AstError> {
        let value = ctx
            .call_stack
            .get_current_frame()
            .ok_or_else(|| {
                let s = format!("Failed to find current frame");
                AstError::EvaluationFailed(s)
            })?
            .resolve_to_value(&node.identifier())
            .ok_or_else(|| {
                let s = format!("Failed to resolve identifier '{}'", node.identifier());
                AstError::EvaluationFailed(s)
            })?;

        let mut result = String::new();

        match &value {
            Value::Array(arr) => {
                for (i, j) in arr.iter().enumerate() {
                    ctx.call_stack
                        .get_mut_current_scope()
                        .ok_or_else(|| {
                            let s = format!("Failed to find current scope");
                            AstError::EvaluationFailed(s)
                        })?
                        .set(
                            for_node.l_identifier(),
                            Literal::Integer(i64::try_from(i).unwrap()),
                        );

                    match j {
                        Value::String(value) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(value.clone()));
                        }

                        Value::Number(value) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(value.to_string()));
                        }

                        Value::Bool(b) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(b.to_string()));
                        }

                        Value::Object(_) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .open_pseudo(for_node.r_identifier(), j)
                                .map_err(|e| AstError::EvaluationFailed(e.to_string()))?;
                        }

                        Value::Array(_) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .open_pseudo(for_node.r_identifier(), j)
                                .map_err(|e| AstError::EvaluationFailed(e.to_string()))?;
                        }

                        _ => {
                            let s = format!("Unsupported json format");
                            return Err(AstError::EvaluationFailed(s));
                        }
                    }

                    let iteration_result = root.evaluate(ctx).map_err(|e| {
                        let s = format!("Failed to evaluate FOR directive\n{}", e.to_string());
                        AstError::EvaluationFailed(s)
                    })?;

                    result.push_str(&iteration_result);
                }
            }
            Value::Object(obj) => {
                for (i, j) in obj.iter() {
                    ctx.call_stack
                        .get_mut_current_scope()
                        .ok_or_else(|| {
                            let s = format!("Failed to find current scope");
                            AstError::EvaluationFailed(s)
                        })?
                        .set(for_node.l_identifier(), Literal::String(i.clone()));

                    match j {
                        Value::String(s) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(s.clone()));
                        }

                        Value::Number(i) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(i.to_string()));
                        }

                        Value::Bool(b) => {
                            ctx.call_stack
                                .get_mut_current_scope()
                                .ok_or_else(|| {
                                    let s = format!("Failed to find current scope");
                                    AstError::EvaluationFailed(s)
                                })?
                                .set(for_node.r_identifier(), Literal::String(b.to_string()));
                        }

                        _ => {
                            let s = format!("Unsupported json format");
                            return Err(AstError::EvaluationFailed(s));
                        }
                    }

                    let iteration_result = root.evaluate(ctx).map_err(|e| {
                        let s = format!("Failed to evaluate FOR directive\n{}", e.to_string());
                        AstError::EvaluationFailed(s)
                    })?;

                    result.push_str(&iteration_result);
                }
            }

            _ => {
                let s = format!("Unsupported json structure");
                return Err(AstError::EvaluationFailed(s));
            }
        }

        Ok(result)
    }
}
