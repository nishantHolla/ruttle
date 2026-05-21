use crate::store::FileId;

#[derive(Copy, Clone, Debug)]
pub struct Hint {
    file_id: FileId,
    start: usize,
    end: usize,
}

impl Hint {
    pub fn new(file_id: FileId, start: usize, end: usize) -> Self {
        Self {
            file_id,
            start,
            end,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Hint({:?}, {}, {})", self.file_id, self.start, self.end)
    }
}
