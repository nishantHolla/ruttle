use super::error::ContextError;
use super::frame::Frame;
use super::scope::{Scope, ScopeDef};
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

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn get_current_scope(&self) -> Option<&Scope> {
        if self.stack.len() == 0 {
            None
        } else {
            self.stack.last().unwrap().get_current_scope()
        }
    }

    pub fn get_mut_current_scope(&mut self) -> Option<&mut Scope> {
        if self.stack.len() == 0 {
            None
        } else {
            self.stack.last_mut().unwrap().get_mut_current_scope()
        }
    }

    // pub fn set_definition(&mut self, key: &str, lit: Literal) {
    //     if self.stack.len() == 0 {
    //         return;
    //     }
    //
    //     self.stack.last_mut().unwrap().set_definition(key, lit);
    // }
    //
    // pub fn get_definition(&self, key: &str) -> Option<&Literal> {
    //     if self.stack.len() == 0 {
    //         return None;
    //     }
    //
    //     self.stack.last().unwrap().get_definition(key)
    // }
    //
    // pub fn open_file(
    //     &mut self,
    //     identifier: &str,
    //     path: impl AsRef<Path>,
    //     file_id: FileId,
    // ) -> Result<(), ContextError> {
    //     if self.stack.len() == 0 {
    //         let s = format!("No frame to open current file in");
    //         return Err(ContextError::NoFrameError(s));
    //     }
    //
    //     self.stack
    //         .last_mut()
    //         .unwrap()
    //         .open_file(identifier, path, file_id)
    // }

    pub fn get_current_frame(&self) -> Option<&Frame> {
        if self.stack.len() == 0 {
            None
        } else {
            self.stack.last()
        }
    }

    pub fn get_mut_current_frame(&mut self) -> Option<&mut Frame> {
        if self.stack.len() == 0 {
            None
        } else {
            self.stack.last_mut()
        }
    }

    // pub fn enter_new_scope(&mut self) {
    //     if self.stack.len() == 0 {
    //         return;
    //     }
    //
    //     self.stack.last_mut().unwrap().enter_new_scope();
    // }
    //
    // pub fn exit_current_scope(&mut self) {
    //     if self.stack.len() == 0 {
    //         return;
    //     }
    //
    //     self.stack.last_mut().unwrap().exit_current_scope();
    // }

    pub fn debug(&self) {
        println!("debug: CallStack({})\n", self.stack.len());

        for (i, frame) in self.stack.iter().enumerate() {
            println!("       {}: {}", i, frame.to_string());
        }
        println!();
    }
}
