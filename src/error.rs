use thiserror::Error;

use crate::args;

#[derive(Debug, Error)]
pub enum TerusError {
    #[error(transparent)]
    Args(#[from] args::error::ArgsError),
}

impl TerusError {
    pub fn print(&self) {
        let s = self.to_string();
        let mut lines = s.lines();

        if let Some(first) = lines.next() {
            eprintln!("error: {}", first);
        }

        for line in lines {
            eprintln!("       {}", line);
        }
    }
}
