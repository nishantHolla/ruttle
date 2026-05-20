use crate::store::FileId;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct OutMap {
    map: HashMap<FileId, String>,
    base: PathBuf,
}
