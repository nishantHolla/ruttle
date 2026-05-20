use crate::store::FileId;

#[derive(Copy, Clone, Debug)]
pub struct Hint {
    file_id: FileId,
    start: usize,
    end: usize,
}
