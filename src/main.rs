mod catcher;
mod color;
mod fs_ops;
mod json_data;

use catcher::{Catch, Error::*};
use color::Color;
use fs_ops::{get_abs_path, retrieve_dir_filenames};
use json_data::{dump_tag_data, retrieve_tag_data};
use std::{collections::HashMap, env::args, fs::canonicalize, path::Path};
use strum::IntoEnumIterator;

type Tags = HashMap<String, HashMap<String, Color>>;

const DATA_FOLDER: &str = "~/.config/tag/tag-data.json";

fn main() {
    let args: Vec<String> = args().collect();
    let argc = args.len();

    // TODO: check config file presence
    // TODO: make a clean

    if argc < 2 {
        NoArgument.abort();
    }

    if argc == 2 {
        if args[1] == "list" {
            print_list(".");
        } else if args[1] == "clean" {
            unimplemented!();
        } else {
            UnknownArgument.abort();
        }
        return;
    }

    if argc == 3 {
        if args[1] == "list" {
            print_list(&args[2]);
        } else if args[1] == "order" {
            unimplemented!();
        } else {
            tag_file(&args[1], &args[2]);
        }
        return;
    }

    if argc > 3 {
        TooManyArguments.abort();
    }
}

fn tag_file(filename: &str, color: &str) {
    // Check if color exists
    let new_color = Color::iter()
        .find(|c| color.eq(c.name()))
        .catch(UnknownValue);

    // Check that filename exists
    let path = Path::new(filename);
    if !path.exists() {
        FileNotFound.abort();
    }

    // Get the absolute path
    let abs_path = canonicalize(path).catch(NonCanonicalizablePath);

    // Get the basename
    let basename = abs_path
        .file_name()
        .catch(BaseNameNotFound)
        .to_str()
        .catch(StringConversionFailure);

    // Get the parent dir
    let parent_dir = abs_path
        .parent()
        .catch(ParentDirNotFound)
        .to_str()
        .catch(StringConversionFailure);

    // Get folder tags
    let mut json_data = retrieve_tag_data();

    // Add color to the entry
    json_data
        .entry(parent_dir.to_string())
        .or_insert_with(HashMap::new)
        .insert(basename.to_string(), new_color);

    // Dump back json data
    dump_tag_data(json_data);
}

fn print_list(dir: &str) {
    // Get filenames of the provided dir
    let dir_filenames = retrieve_dir_filenames(dir);

    // Get folders tags
    let json_data = retrieve_tag_data();

    // Get the absolute path
    let abs_path = get_abs_path(dir);

    // Check if there is json tag data for the provided folder
    match json_data.get(&abs_path) {
        Some(dir_tags) => {
            for item in dir_filenames {
                // Check if filename has a tag and print
                match dir_tags.get(&item) {
                    Some(color) => println!("{}{}{}", color.value(), item, Color::Reset.value()),
                    None => println!("{}", item),
                }
            }
        }
        None => {
            for item in dir_filenames {
                println!("{}", item)
            }
        }
    }

    // TODO: show other file metadata
    // TODO: use alphabetical order
    // TODO: manage dot files
}
