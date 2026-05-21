use thiserror::Error;

use crate::args;
use crate::context::{self, Context};

#[derive(Debug, Error)]
pub enum TerusError {
    #[error(transparent)]
    Args(#[from] args::error::ArgsError),

    #[error(transparent)]
    Context(#[from] context::error::ContextError),
}

impl TerusError {
    pub fn print(&self, ctx: Option<Context>) {
        let s = self.to_string();
        let mut lines = s.lines();

        if let Some(first) = lines.next() {
            eprintln!("error: {}", first);
        }

        for line in lines {
            eprintln!("       {}", line);
        }

        if let Some(ctx) = ctx {
            let hint_str = ctx.hint_stack.to_string(7, &ctx.file_store);
            eprintln!("\n{}", hint_str);
        }
    }
}
