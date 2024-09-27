mod catcher;
mod color;
mod fs_ops;
mod json_data;

use catcher::{Catch, Error::*};
use color::Color;
use fs_ops::{get_abs_path, retrieve_dir_files};
use json_data::{clean_tag_data, dump_tag_data, retrieve_tag_data};
use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
    env::args,
    fs::canonicalize,
    path::Path,
};
use strum::IntoEnumIterator;

type Tags = HashMap<String, HashMap<String, Color>>;

const DATA_FOLDER: &str = "~/.config/tag/tag-data.json";

fn main() {
    let args: Vec<String> = args().collect();
    let argc = args.len();

    clean_tag_data();

    if argc < 2 {
        NoArgument.abort();
    }

    if argc == 2 {
        if args[1] == "ls" {
            print_list(".", false);
        } else if args[1] == "la" {
            print_list(".", true);
        } else {
            UnknownArgument.abort();
        }
        return;
    }

    if argc == 3 {
        if args[1] == "ls" {
            print_list(&args[2], false);
        } else if args[1] == "la" {
            print_list(&args[2], true);
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

fn print_list(dir: &str, hidden_files: bool) {
    // Get filenames of the provided dir
    let mut dir_files = retrieve_dir_files(dir);

    // Remove hidden files if not requested
    if !hidden_files {
        dir_files.retain(|item| !item.filename.starts_with("."));
    }

    // Reorder files list
    dir_files.sort_by(|a, b| match (a.directory, b.directory) {
        (true, false) => Less,
        (false, true) => Greater,
        _ => a.filename.to_lowercase().cmp(&b.filename.to_lowercase()),
    });

    // Get folders tags
    let json_data = retrieve_tag_data();

    // Get the absolute path
    let abs_path = get_abs_path(dir);

    // Check if there is json tag data for the provided folder
    match json_data.get(&abs_path) {
        Some(dir_tags) => {
            for item in dir_files {
                // Check if filename has a tag and print
                match dir_tags.get(&item.filename) {
                    Some(color) => println!("{}{}{}", color.value(), item, Color::Reset.value()),
                    None => {
                        if item.directory {
                            println!("{}{}{}", Color::Cyan.value(), item, Color::Reset.value())
                        } else {
                            println!("{}", item)
                        }
                    }
                }
            }
        }
        None => {
            for item in dir_files {
                if item.directory {
                    println!("{}{}{}", Color::Cyan.value(), item, Color::Reset.value())
                } else {
                    println!("{}", item)
                }
            }
        }
    }
}
