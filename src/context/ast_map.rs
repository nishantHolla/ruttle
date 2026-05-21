use crate::ast::Node;
use crate::store::{FileId, NodeId, NodeStore};
use std::collections::HashMap;

pub struct AstMap {
    map: HashMap<FileId, NodeId>,
}

impl AstMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn has_ast_for(&self, file_id: FileId) -> bool {
        self.map.contains_key(&file_id)
    }

    pub fn insert(&mut self, file_id: FileId, node_id: NodeId) {
        self.map.insert(file_id, node_id);
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
