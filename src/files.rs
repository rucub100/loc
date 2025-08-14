use std::collections::VecDeque;
use std::env::current_dir;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;

use crate::ignore::Ignore;

#[derive(Debug)]
pub enum FilesError {
    Io(std::io::Error),
}

pub struct Files {
    root_dir: PathBuf,
    dirs: VecDeque<PathBuf>,
    dir_iter: ReadDir,
    ignore: Option<Ignore>,
}

impl Files {
    pub fn new(path: Option<PathBuf>, ignore: Option<Ignore>) -> Result<Self, FilesError> {
        let root_dir = path.unwrap_or(current_dir().map_err(FilesError::Io)?);
        let dir_iter = root_dir.read_dir().map_err(FilesError::Io)?;

        Ok(Files {
            root_dir,
            dirs: VecDeque::new(),
            dir_iter,
            ignore,
        })
    }

    fn next_entry(&mut self) -> Option<Result<DirEntry, FilesError>> {
        let mut next_entry = self.dir_iter.next();

        while next_entry.is_none() {
            let next_dir = self.dirs.pop_front()?;
            assert!(next_dir.is_dir());
            let next_dir = next_dir.read_dir();
            let next_dir = match next_dir {
                Ok(next_dir) => next_dir,
                Err(err) => return Some(Err(FilesError::Io(err))),
            };
            self.dir_iter = next_dir;
            next_entry = self.dir_iter.next();
        }

        match next_entry.unwrap() {
            Ok(entry) => Some(Ok(entry)),
            Err(err) => Some(Err(FilesError::Io(err))),
        }
    }

    fn next_file(&mut self, entry: DirEntry) -> Option<PathBuf> {
        let path = entry.path();

        if path.is_dir() {
            self.add_dir(path);
            return None;
        }

        if path.is_file() {
            return Some(path);
        }

        None
    }

    fn add_dir(&mut self, path: PathBuf) {
        // TODO: use .gitignore if available
        if let Some(ref ignore) = self.ignore
            && let Some(file_name) = path.file_name()
            && let Some(dir_name) = file_name.to_str()
            && ignore.should_ignore_dir(dir_name)
        {
            return;
        }

        self.dirs.push_back(path);
    }
}

impl Iterator for Files {
    type Item = Result<PathBuf, FilesError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_file: Option<PathBuf> = None;

        while next_file.is_none() {
            let next_entry = self.next_entry()?;
            next_file = match next_entry {
                Err(err) => return Some(Err(err)),
                Ok(entry) => self.next_file(entry),
            };
        }

        let next_file = next_file.unwrap();
        assert!(next_file.is_file());
        Some(Ok(next_file))
    }
}
