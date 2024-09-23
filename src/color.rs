use serde::{Deserialize, Serialize};
use Color::*;

#[derive(strum_macros::EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Color {
    pub fn name(&self) -> &str {
        match self {
            Reset => "reset",
            Red => "red",
            Green => "green",
            Yellow => "yellow",
            Blue => "blue",
            Magenta => "magenta",
            Cyan => "cyan",
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Reset => "\x1b[0m",
            Red => "\x1b[31m",
            Green => "\x1b[32m",
            Yellow => "\x1b[33m",
            Blue => "\x1b[34m",
            Magenta => "\x1b[35m",
            Cyan => "\x1b[36m",
        }
    }
}
