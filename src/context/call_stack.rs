use super::frame::Frame;

pub struct CallStack {
    stack: Vec<Frame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}
