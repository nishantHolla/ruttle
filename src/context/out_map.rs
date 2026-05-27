use super::error::OutMapError;
use crate::config;
use crate::store::{FileId, FileStore};
use crate::util;
use minify_html::{Cfg, minify};
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

    pub fn insert(&mut self, file_id: FileId, string: impl Into<String>) {
        self.map.insert(file_id, string.into());
    }

    pub fn minify(&mut self) {
        let cfg = Cfg {
            minify_js: true,
            minify_css: true,
            keep_comments: false,
            keep_closing_tags: false,
            ..Cfg::new()
        };

        for html in self.map.values_mut() {
            let minified = minify(html.as_bytes(), &cfg);
            *html = String::from_utf8(minified).unwrap();
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

    pub fn debug(&self) {
        println!("debug: OutMap\n");

        println!("       base: {}", self.base.display());
        for (file_id, output) in &self.map {
            println!("       {:?}: {}", file_id, output);
        }
        println!();
    }
}
