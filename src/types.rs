use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SquishResult {
    scanned: Vec<Scanned>,
    squished: Vec<Squished>,
    failed: Vec<Failed>,
    ignored: u64,
}

impl SquishResult {
    pub fn new() -> Self {
        Self {
            scanned: Vec::new(),
            squished: Vec::new(),
            failed: Vec::new(),
            ignored: 0,
        }
    }

    pub fn iter_scanned(&self) -> &[Scanned] {
        &self.scanned
    }

    pub fn scanned(&mut self, path: &Path) {
        let result = Scanned::new(path);
        self.scanned.push(result);
    }

    pub fn success(&mut self, path: &Path, size: u64) {
        let result = Squished::new(path, size);
        self.squished.push(result);
    }

    pub fn failure(&mut self, path: &Path, error: &str) {
        let result = Failed::new(path, error);
        self.failed.push(result);
    }

    pub fn ignored(&mut self) {
        self.ignored += 1;
    }

    fn size(&self) -> u64 {
        self.squished.iter().map(|sq| sq.size).sum()
    }

    pub fn extend(&mut self, other: SquishResult) {
        self.scanned.extend(other.scanned);
        self.squished.extend(other.squished);
        self.failed.extend(other.failed);
        self.ignored += other.ignored;
    }

    pub fn summary(&self, output: &Path) {
        let scanned = self.scanned.len() as u64;
        let total = scanned + self.ignored;
        let success = self.squished.len() as u64;

        println!(
            "Squishy file: {}
Scanned {scanned} of {total} files
Processed {success} of {scanned} ({} total bytes)",
            output.display(),
            self.size()
        );

        self.squished
            .iter()
            .for_each(|Squished { path, size }| println!("  ✓ {} ({size} bytes)", path.display()));

        self.failed
            .iter()
            .for_each(|Failed { path, error }| println!("  ✗ {} ({error})", path.display()));
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scanned {
    pub path: PathBuf,
}

impl Scanned {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Squished {
    path: PathBuf,
    size: u64,
}

impl Squished {
    pub fn new(path: &Path, size: u64) -> Self {
        Self {
            path: path.to_path_buf(),
            size,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Failed {
    path: PathBuf,
    error: String,
}

impl Failed {
    pub fn new(path: &Path, error: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            error: error.to_string(),
        }
    }
}
