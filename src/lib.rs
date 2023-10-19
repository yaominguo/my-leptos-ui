mod button;
mod config;
mod provider;

use std::str::FromStr;

pub use button::{ButtonDemo, MyButton};
pub use config::*;
pub use provider::MyProvider;

#[derive(Debug, PartialEq, Clone)]
pub enum Size {
    Mini,
    Small,
    Medium,
    Large,
}

impl FromStr for Size {
    type Err = String;

    fn from_str(input: &str) -> Result<Size, Self::Err> {
        match input {
            "mini" => Ok(Size::Mini),
            "small" => Ok(Size::Small),
            "medium" => Ok(Size::Medium),
            "large" => Ok(Size::Large),
            _ => Err("Unrecognized parameter for 'size'".to_owned()),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ButtonType {
    Default,
    Primary,
    Info,
    Warning,
    Error,
}

impl FromStr for ButtonType {
    type Err = String;

    fn from_str(input: &str) -> Result<ButtonType, Self::Err> {
        match input {
            "default" => Ok(ButtonType::Default),
            "primary" => Ok(ButtonType::Primary),
            "info" => Ok(ButtonType::Info),
            "warning" => Ok(ButtonType::Warning),
            "error" => Ok(ButtonType::Error),
            _ => Err("Unrecognized parameter for 'button type'".to_owned()),
        }
    }
}
