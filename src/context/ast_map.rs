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
}
