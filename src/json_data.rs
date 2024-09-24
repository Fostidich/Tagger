use crate::catcher::{Catch, Error::*};
use crate::Tags;
use crate::{fs_ops::open_file, DATA_FOLDER};

use std::io::{Read, Write};

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
