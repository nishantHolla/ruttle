use super::hint::Hint;
use crate::store::FileId;
use std::collections::HashMap;

pub struct IncludeNode {
    file_id: FileId,
    props: HashMap<String, String>,
    hint: Hint,
}
