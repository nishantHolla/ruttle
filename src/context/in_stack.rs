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

    pub fn pop(&mut self) -> Option<FileId> {
        self.stack.pop()
    }

    pub fn empty(&self) -> bool {
        self.stack.len() == 0
    }
}
