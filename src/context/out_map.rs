use super::error::OutMapError;
use crate::config;
use crate::store::{FileId, FileStore};
use crate::util;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct OutMap {
    map: HashMap<FileId, String>,
    base: PathBuf,
}

impl OutMap {
    pub fn new(base: impl AsRef<Path>) -> Self {
        Self {
            map: HashMap::new(),
            base: base.as_ref().to_path_buf(),
        }
    }

    pub fn save(&self, file_store: &FileStore) -> Result<(), OutMapError> {
        for (file_id, output) in &self.map {
            let file_path = file_store.get_by_id(*file_id).ok_or_else(|| {
                let s = format!("Could not find the stored file");
                OutMapError::FileMissingError(s)
            })?;

            let file_name = file_path.file_name().ok_or_else(|| {
                let s = format!("Could not find file name for path {}", file_path.display());
                OutMapError::FileMissingError(s)
            })?;

            let output_path = self.base.join(file_name);
            let output_path = util::file::replace_extension(
                &output_path,
                &config::PART_EXTENSION,
                &config::OUTPUT_EXTENSION,
            )
            .unwrap();

            let mut file = File::create(&output_path).map_err(|e| {
                let s = format!(
                    "Failed to open file {} for writing\n{}",
                    output_path.display(),
                    e.to_string()
                );
                OutMapError::WriteError(s)
            })?;

            file.write_all(output.as_bytes()).map_err(|e| {
                let s = format!(
                    "Failed to write file {}\n{}",
                    output_path.display(),
                    e.to_string()
                );
                OutMapError::WriteError(s)
            })?;
        }

        Ok(())
    }
}
