use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileId(usize);

pub struct FileStore {
    file_list: Vec<PathBuf>,
    file_map: HashMap<PathBuf, FileId>,
}
