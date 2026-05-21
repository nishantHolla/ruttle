use crate::ast::Hint;
use crate::store::FileStore;

pub struct HintStack {
    stack: Vec<Hint>,
}

impl HintStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, hint: Hint) {
        self.stack.push(hint);
    }

    pub fn pop(&mut self) -> Option<Hint> {
        self.stack.pop()
    }

    pub fn to_string(&self, indent: usize, fs: &FileStore) -> String {
        let mut result = String::new();
        let indent_str = " ".repeat(indent);

        for (i, hint) in self.stack.iter().rev().enumerate() {
            let hint_str = hint.expand(fs);

            let mut lines = hint_str.lines().peekable();
            while let Some(line) = lines.next() {
                result.push_str(&indent_str);
                result.push_str(line);

                if lines.peek().is_some() {
                    result.push('\n');
                }
            }

            if i != self.stack.len() - 1 {
                result.push_str("\n\n");
            }
        }

        return result;
    }
}
