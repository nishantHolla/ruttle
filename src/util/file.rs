use std::path::Path;

pub fn has_extension(path: &Path, ext: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |name| name.ends_with(ext))
}
