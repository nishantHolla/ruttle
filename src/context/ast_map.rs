use crate::store::{FileId, NodeId};
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
}
