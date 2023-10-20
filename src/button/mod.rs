mod demo;
mod style;

use crate::{GlobalConfig, Size};
pub use demo::ButtonDemo;
use leptos::{component, expect_context, view, Children, IntoView, RwSignal, SignalGetUntracked};
use std::str::FromStr;
use style::get_class;

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

#[component]
pub fn MyButton(
    children: Children,
    #[prop(optional)] size: &'static str,
    #[prop(default = "default")] mode: &'static str,
    #[prop(default = false)] plain: bool,
    #[prop(default = false)] rounded: bool,
    #[prop(default = false)] ghost: bool,
    #[prop(default = false)] text: bool,
    #[prop(default = false)] dashed: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let config = expect_context::<RwSignal<GlobalConfig>>();
    let mut button_class = get_class(
        Size::from_str(size).unwrap_or(config.get_untracked().size),
        ButtonType::from_str(mode).unwrap_or(ButtonType::Default),
        plain,
        rounded,
        ghost,
        text,
        dashed,
        disabled,
    );
    if !class.is_empty() {
        button_class.push(' ');
        button_class.push_str(class);
    }
    view! {
        <button class=button_class disabled=disabled>
            {children()}
        </button>
    }
}
