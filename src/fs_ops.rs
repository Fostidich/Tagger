use crate::catcher::{Catch, Error::*};

use home::home_dir;
use std::{
    fs::{canonicalize, create_dir_all, read_dir, File},
    path::{Path, PathBuf},
};

pub fn retrieve_dir_filenames(dir: &str) -> Vec<String> {
    let path = Path::new(dir);
    let mut filenames = Vec::new();

    // Get string filename from each entry in the dir
    read_dir(path).catch(IOError).for_each(|entry| {
        filenames.push(
            entry
                .catch(IOError)
                .path()
                .file_name()
                .catch(BaseNameNotFound)
                .to_str()
                .catch(StringConversionFailure)
                .to_string(),
        )
    });

    filenames
}

pub fn get_abs_path(dir: &str) -> String {
    // Make the path absolute
    canonicalize(dir)
        .catch(NonCanonicalizablePath)
        .to_str()
        .catch(StringConversionFailure)
        .to_string()
}

pub fn open_file(path: &str) -> File {
    // Expand the tilde to the home directory
    let mut path = PathBuf::from(path);
    if path.starts_with("~") {
        path = home_dir()
            .catch(IOError)
            .join(path.strip_prefix("~").unwrap());
    }

    // Create the parent directories if they don't exist
    create_dir_all(path.parent().catch(ParentDirNotFound)).catch(IOError);

    // Open/create the file
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .catch(IOError)
}
