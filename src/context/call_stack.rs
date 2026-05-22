use super::error::ContextError;
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

    pub fn push(
        &mut self,
        file_id: FileId,
        initial_scope: Option<ScopeDef>,
    ) -> Result<(), ContextError> {
        let frame = Frame::new(file_id, initial_scope);

        for existing_frame in &self.stack {
            if frame.fingerprint() == existing_frame.fingerprint() {
                let s = format!("Duplicate frame detected");
                return Err(ContextError::DuplicatePush(s));
            }
        }

        self.stack.push(frame);
        Ok(())
    }

    pub fn set_definition(&mut self, key: &str, lit: Literal) {
        if self.stack.len() == 0 {
            return;
        }

        self.stack.last_mut().unwrap().set_definition(key, lit);
    }

    pub fn get_definition(&self, key: &str) -> Option<&Literal> {
        if self.stack.len() == 0 {
            return None;
        }

        self.stack.last().unwrap().get_definition(key)
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
