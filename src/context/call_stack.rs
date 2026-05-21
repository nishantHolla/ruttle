use super::frame::Frame;
use super::scope::ScopeDef;
use crate::store::FileId;

pub struct CallStack {
    stack: Vec<Frame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, file_id: FileId, initial_scope: Option<ScopeDef>) {
        let frame = Frame::new(file_id, initial_scope);
        self.stack.push(frame);
    }

    pub fn debug(&self) {
        println!("debug: CallStack\n");

        for (i, frame) in self.stack.iter().enumerate() {
            println!("       {}: {}", i, frame.to_string());
        }
        println!();
    }
}
