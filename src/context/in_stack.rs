use crate::store::FileId;

pub struct InStack {
    stack: Vec<FileId>,
}

impl InStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, id: FileId) {
        self.stack.push(id);
    }
}
