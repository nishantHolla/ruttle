use crate::store::{FileId, NodeId};
use std::collections::HashMap;

pub struct AstMap {
    map: HashMap<FileId, NodeId>,
}
