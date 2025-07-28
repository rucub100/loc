use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FilesError {
    Io(std::io::Error),
    NotADirectory,
}

pub struct Files {
    dir: PathBuf,
}

impl Files {
    pub fn new() -> Result<Self, FilesError> {
        let current_dir = current_dir().map_err(FilesError::Io)?;

        Ok(Files {
            dir: current_dir,
        })
    }
}

impl Iterator for Files {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}