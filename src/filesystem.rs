use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

use crate::errors::Result;

/// Builder-ish pattern to attempt to create a file with fallback directories.
///
/// Usage examples:
/// ```no_run
///
/// // attempt to create a file named `filename.txt` in the current directory
/// let FileDescriptor { file, path } = FileCreator::new("filename.txt", PathBuf::from("."))
///     .with_fallback(Path::from("/tmp"))  // or attempts to fallback to `/tmp`
///     .allow_truncate_file(true)          // truncates the file if it existed
///     .allow_create_dirs(true)            // will try to create directories along the way
///     .create();
///
/// // do stuff with `file` and `path`
/// ```
pub struct FileCreator {
    name: String,
    paths: Vec<PathBuf>,
    create: bool,
    truncate: bool,
}

impl FileCreator {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            paths: vec![path.to_path_buf()],
            create: false,
            truncate: false,
        }
    }

    /// Add additional fallback directories in case the original directory
    /// is not a viable option, it will attempt to use the fallback directories.
    pub fn with_fallback(mut self, path: &Path) -> Self {
        let path = path.to_path_buf();
        if !self.paths.contains(&path) {
            self.paths.push(path);
        }

        self
    }

    /// By default directory creation is disabled, this behaviour can be
    /// controlled through this flag, setting it to true will allow attempting
    /// to create the directory to the target file, creation can still fail.
    pub fn allow_create_dirs(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    /// By default no file truncation will be done if the file already existed
    /// Set this flag to true to truncate an existing file.
    pub fn allow_truncate_file(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    /// Attempts to create the file with fallback options, it will create or not
    /// directories depending on the creation flag status.
    pub fn create(self) -> Result<FileDescriptor> {
        for path in self.paths {
            // directory is not writeable or does not exist
            if !is_writeable_dir(&path) {
                // and we are not allowed to create them
                if !self.create {
                    continue;
                }

                // or if we cannot create the directory
                if fs::create_dir_all(&path).is_err() {
                    continue;
                }

                // double checking does not hurt
                if !is_writeable_dir(&path) {
                    // give up for this path
                    continue;
                }
            }

            // attempt to create the file with provided flags
            let filepath = &path.join(&self.name);
            let result = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(self.truncate)
                .open(filepath);

            match result {
                Ok(file) => {
                    return Ok(FileDescriptor {
                        file,
                        path: filepath.clone(),
                    });
                }
                Err(error) => eprintln!("failed to create file: {error}"),
            }
        }

        // we exhausted every possible given directories, giving up on creating the file.
        Err(crate::errors::SquishError::FileCreationExhausted {
            filename: self.name,
        })
    }
}

#[derive(Debug)]
pub struct FileDescriptor {
    pub file: File,
    pub path: PathBuf,
}

fn is_writeable_dir(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_dir() && !metadata.permissions().readonly();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_writeable_dir_ok() {
        // squishy project dir should be writeable
        let cwd = PathBuf::from(".");
        assert!(is_writeable_dir(&cwd));
    }

    #[test]
    fn check_writeable_dir_no_dir() {
        // make up a random local folder that *should* not exist
        // UUID generated in the past might give more confidence here.
        let nodir = PathBuf::from(".").join("e079f2f6-0140-46ad-815f-d4a986cd9ea6");
        assert!(!is_writeable_dir(&nodir));
    }

    #[test]
    #[cfg(feature = "readonly-test")]
    fn check_writeable_dir_readonly() {
        // requires to create a directory with readonly manually
        // test is not run unless requested explicitely with `cargo test --features readonly-test`
        let readonly = PathBuf::from(".").join("readonly");
        assert!(!is_writeable_dir(&readonly));
    }

    #[test]
    fn file_creation() -> Result<()> {
        // using the fact that `target` should exist by now as `cargo test` build the code before
        // it runs the tests, hopefully.
        let target = PathBuf::from(".").join("target");
        let FileDescriptor { path, .. } = FileCreator::new("rand_test_file_creation.txt", &target)
            .allow_create_dirs(false)
            .allow_truncate_file(true)
            .create()?;

        assert!(path.exists() && path.is_file());
        cleanup(path);
        Ok(())
    }

    #[test]
    fn file_creation_with_fallback_ok() -> Result<()> {
        let target = PathBuf::from(".").join("target");
        let nodir = PathBuf::from(".").join("e079f2f6-0140-46ad-815f-d4a986cd9ea6");
        let FileDescriptor { path, .. } =
            FileCreator::new("rand-test_file_creation_with_fallback.txt", &nodir)
                .with_fallback(&target)
                .allow_create_dirs(false)
                .allow_truncate_file(true)
                .create()?;

        assert!(path.exists() && path.is_file());
        cleanup(path);
        Ok(())
    }

    // consumes pathbuf on purpose
    fn cleanup(path: PathBuf) {
        // atempt to clean up file
        if let Err(error) = fs::remove_file(&path) {
            // this warning is probably going to be ignored if the test succeeded.
            eprintln!(
                "failed to remove test file at: {}\n\t{error}",
                &path.display()
            );
        }
    }
}
