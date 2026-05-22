use crate::store::{FileId, NodeId, NodeStore};
use std::collections::HashMap;

pub struct AstMap {
    map: HashMap<FileId, NodeId>,
    todo: Vec<FileId>,
}

impl AstMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            todo: Vec::new(),
        }
    }

    pub fn todo_is_empty(&self) -> bool {
        self.todo.len() == 0
    }

    pub fn has_todo(&self, file_id: FileId) -> bool {
        self.todo.contains(&file_id)
    }

    pub fn add_todo(&mut self, file_id: FileId) {
        self.todo.push(file_id)
    }

    pub fn pop_todo(&mut self) -> Option<FileId> {
        self.todo.pop()
    }

    pub fn has_ast_for(&self, file_id: FileId) -> bool {
        self.map.contains_key(&file_id)
    }

    pub fn insert(&mut self, file_id: FileId, node_id: NodeId) {
        self.map.insert(file_id, node_id);
    }

    pub fn get(&self, file_id: FileId) -> Option<NodeId> {
        self.map.get(&file_id).cloned()
    }

    pub fn debug(&self) {
        println!("debug: AstMap\n");

        for (file_id, node_id) in &self.map {
            println!("       {:?}: {:?}", file_id, node_id);
        }
        println!();
    }

    pub fn debug_ast(&self, file_id: FileId, ns: &NodeStore) {
        if !self.has_ast_for(file_id) {
            return;
        }

        println!("debug: AST for {:?}\n", file_id);

        let node = *self.map.get(&file_id).unwrap();
        if let Some(node) = ns.get(node) {
            node.debug(7, ns);
        }

        println!();
    }

    pub fn debug_all_ast(&self, ns: &NodeStore) {
        for (file_id, _) in &self.map {
            self.debug_ast(*file_id, ns);
        }
    }
}
