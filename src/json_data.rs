use crate::catcher::{Catch, Error::*};
use crate::color::Color;
use crate::Tags;
use crate::{fs_ops::open_file, DATA_FOLDER};

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn retrieve_tag_data() -> Tags {
    // Get content of file as string
    let mut buff = String::new();
    open_file(DATA_FOLDER)
        .read_to_string(&mut buff)
        .catch(StringConversionFailure);

    // If file didn't exist or is empty fill it with something
    if buff.is_empty() {
        buff = "{}".to_string();
    }

    // Deserialize from string
    serde_json::from_str::<Tags>(&buff).catch(SerdeJsonConversionFailure)
}

pub fn dump_tag_data(data: Tags) {
    // Serialize from object
    let buff = serde_json::to_string_pretty(&data).catch(SerdeJsonConversionFailure);

    // Write to file truncating old file tail
    let mut file = open_file(DATA_FOLDER);
    file.write_all(&buff.as_bytes()).catch(IOError);
    file.set_len(buff.len() as u64).catch(IOError);
}

pub fn clean_tag_data() {
    // Retrieve folder tags
    let mut json_data = retrieve_tag_data();

    // Remove parent dirs that don't exist anymore or that are empty
    json_data.retain(|k, v| !v.is_empty() && Path::new(k).exists());

    // Remove basenames that don't exist anymore or that are reset colored
    for folder in json_data.iter_mut() {
        folder
            .1
            .retain(|k, v| PathBuf::from(folder.0).join(k).exists() && *v != Color::Reset);
    }

    // Dump back json data
    dump_tag_data(json_data);
}
