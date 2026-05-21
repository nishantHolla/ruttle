use std::path::{Path, PathBuf};

pub fn has_extension(path: &Path, ext: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |name| name.ends_with(ext))
}

pub fn replace_extension(path: &Path, ext: &str, to: &str) -> Option<PathBuf> {
    let file_name = path.file_name()?.to_str()?;

    if let Some(stripped) = file_name.strip_suffix(ext) {
        let new_name = format!("{stripped}{to}");
        Some(path.with_file_name(new_name))
    } else {
        None
    }
}

pub fn get_row_col(path: &Path, index: usize) -> Option<(usize, usize)> {
    let content = std::fs::read_to_string(path).ok()?;
    super::string::get_row_col(&content, index)
}

pub fn get_substr(path: &Path, start: usize, end: usize) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    super::string::get_substr(&content, start, end)
}
