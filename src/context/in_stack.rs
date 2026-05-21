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

    pub fn debug(&self) {
        println!("debug: InStack\n");

        for (i, file_id) in self.stack.iter().enumerate() {
            println!("       {}: {:?}", i, file_id);
        }
        println!();
    }
}
