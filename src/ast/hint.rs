use crate::store::{FileId, FileStore};
use crate::util;

#[derive(Copy, Clone, Debug)]
pub struct Hint {
    file_id: FileId,
    start: usize,
    end: usize,
}

impl Hint {
    pub fn new(file_id: FileId, start: usize, end: usize) -> Self {
        Self {
            file_id,
            start,
            end,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Hint({:?}, {}, {})", self.file_id, self.start, self.end)
    }

    pub fn expand(&self, fs: &FileStore) -> String {
        let Some(path) = fs.get_by_id(self.file_id) else {
            return String::new();
        };

        let Some((row, col)) = util::file::get_row_col(path, self.start) else {
            return format!("{}", path.display());
        };

        let Some(line) = util::file::get_substr(path, self.start, self.end) else {
            return format!("{}:{}:{}", path.display(), row, col);
        };

        let line = util::string::normalize_whitespace(&line);
        let indented_line = util::string::indent_with_pipes(&line);
        return format!("{}:{}:{}\n{}", path.display(), row, col, indented_line);
    }
}
