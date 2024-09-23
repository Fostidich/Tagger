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
            Reset => "\\e[0m",
            Red => "\\e[31m",
            Green => "\\e[32m",
            Yellow => "\\e[33m",
            Blue => "\\e[34m",
            Magenta => "\\e[35m",
            Cyan => "\\e[36m",
        }
    }
}
