use crate::catcher::{Catch, Error::*};

use std::{
    fmt::{Display, Formatter, Result},
    fs::{canonicalize, create_dir_all, read_dir, File},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

pub struct FileData {
    pub filename: String,
    pub directory: bool,
    pub size: u64,
}

impl Display for FileData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.directory {
            write!(f, "      - {}/", self.filename)
        } else {
            write!(
                f,
                "{:>-5.0} - {}",
                byte_unit::Byte::from_u64(self.size)
                    .get_appropriate_unit(byte_unit::UnitType::Decimal),
                self.filename
            )
        }
    }
}

pub fn retrieve_dir_files(dir: &str) -> Vec<FileData> {
    let mut files = Vec::new();

    // Get string filename from each entry in the dir
    read_dir(dir).catch(IOError).for_each(|e| {
        let entry = e.catch(IOError);
        files.push(FileData {
            // Get file name
            filename: entry.file_name().to_string_lossy().to_string(),

            // True if file is dir or symlink to a dir
            directory: entry.file_type().catch(IOError).is_dir()
                || (entry.metadata().catch(IOError).is_symlink()
                    && entry.path().read_link().catch(IOError).is_dir()),

            // Get file size
            size: entry.metadata().catch(IOError).size(),
        })
    });

    files
}

pub fn get_abs_path(dir: &str) -> String {
    // Make the path absolute
    canonicalize(dir)
        .catch(NonCanonicalizablePath)
        .to_string_lossy()
        .to_string()
}

pub fn open_file(path: &str) -> File {
    // Expand the tilde to the home directory
    let mut path_buf = PathBuf::from(path);
    if path_buf.starts_with("~") {
        path_buf = home::home_dir()
            .catch(IOError)
            .join(path_buf.strip_prefix("~").unwrap());
    }

    // Create the parent directories if they don't exist
    create_dir_all(path_buf.parent().catch(ParentDirNotFound)).catch(IOError);

    // Open/create the file
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_buf)
        .catch(IOError)
}
