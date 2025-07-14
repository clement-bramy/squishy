#![allow(unused)]
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FileDescriptor {
    name: String,
    path: PathBuf,
}

impl FileDescriptor {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}
