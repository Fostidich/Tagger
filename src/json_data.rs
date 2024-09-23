use crate::catcher::{Catch, Error::*};
use crate::Tags;
use crate::{fs_ops::open_file, DATA_FOLDER};

use std::io::{Read, Write};

pub fn retrieve_tag_data() -> Tags {
    // Get content of file as string
    let mut buff = String::new();
    open_file(DATA_FOLDER, false)
        .read_to_string(&mut buff)
        .catch(StringConversionFailure);

    // Deserialize from string
    serde_json::from_str::<Tags>(&buff).catch(SerdeJsonConversionFailure)
}

pub fn dump_tag_data(data: Tags) {
    // Serialize from object
    open_file(DATA_FOLDER, false)
        .write_all(
            serde_json::to_string_pretty(&data)
                .catch(SerdeJsonConversionFailure)
                .as_bytes(),
        )
        .catch(IOError)
}
