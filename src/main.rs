use std::{collections::HashMap, env, process::exit};

const DATA_FOLDER: &str = "~/.config/tag/tag-data.json";

enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Color {
    fn name(&self) -> &str {
        match self {
            Color::Reset => "reset",
            Color::Red => "red",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Blue => "blue",
            Color::Magenta => "magenta",
            Color::Cyan => "cyan",
        }
    }

    fn value(&self) -> &str {
        match self {
            Color::Reset => "\\e[0m",
            Color::Red => "\\e[31m",
            Color::Green => "\\e[32m",
            Color::Yellow => "\\e[33m",
            Color::Blue => "\\e[34m",
            Color::Magenta => "\\e[35m",
            Color::Cyan => "\\e[36m",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    if len < 2 {
        eprintln!("No argument provided");
        exit(1);
    }

    if len == 2 {
        if args[1] == "list" {
            print_list(".");
        } else {
            eprintln!("Unknown argument");
            exit(1);
        }
        return;
    }

    if len == 3 {
        if args[1] == "list" {
            print_list(&args[2]);
        } else {
            tag_file(&args[1], &args[2]);
        }
        return;
    }

    if len > 3 {
        eprintln!("Too many arguments");
        exit(1);
    }
}

fn tag_file(filename: &str, color: &str) {
    todo!()
}

fn print_list(dir: &str) {
    // Get filenames of the provided dir
    let dir_filenames = retrieve_dir_filenames(dir);

    // Get tags of the provided dir
    let json_data = retrieve_json_data();
    let dir_tags: &HashMap<String, Color>;
    let empty_map: HashMap<String, Color> = HashMap::new();

    match json_data.get(dir) {
        Some(tags) => dir_tags = tags,
        None => dir_tags = &empty_map,
    }

    // Check if filename has a tag
    for item in dir_filenames {
        match dir_tags.get(&item) {
            Some(color) => println!("{}{}{}", color.value(), item, Color::Reset.value()),
            None => println!("{}", item),
        }
    }

    unimplemented!("use alphabetical order");
}

fn retrieve_json_data() -> HashMap<String, HashMap<String, Color>> {
    todo!();
}

fn dump_json_data(data: HashMap<String, HashMap<String, Color>>) {
    todo!();
}

fn retrieve_dir_filenames(dir: &str) -> Vec<String> {
    todo!();
}
