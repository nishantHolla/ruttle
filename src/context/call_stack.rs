use super::frame::Frame;
use super::scope::ScopeDef;
use crate::ast::Literal;
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

    pub fn set_definition(&mut self, key: &str, lit: Literal) {
        if self.stack.len() == 0 {
            return;
        }

        self.stack.last_mut().unwrap().set_definition(key, lit);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn debug(&self) {
        println!("debug: CallStack({})\n", self.stack.len());

        for (i, frame) in self.stack.iter().enumerate() {
            println!("       {}: {}", i, frame.to_string());
        }
        println!();
    }
}
