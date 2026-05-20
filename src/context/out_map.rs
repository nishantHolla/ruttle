use crate::store::FileId;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct OutMap {
    map: HashMap<FileId, String>,
    base: PathBuf,
}

impl OutMap {
    pub fn new(base: impl AsRef<Path>) -> Self {
        Self {
            map: HashMap::new(),
            base: base.as_ref().to_path_buf(),
        }
    }
}
