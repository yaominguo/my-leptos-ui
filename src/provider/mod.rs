use crate::GlobalConfig;
use leptos::{component, create_rw_signal, provide_context, view, Children, IntoView};
use std::str::FromStr;

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

#[component]
pub fn MyProvider(#[prop(optional)] size: &'static str, children: Children) -> impl IntoView {
    let size = Size::from_str(size).unwrap_or(Size::Medium);
    provide_context(create_rw_signal(GlobalConfig::new(size)));
    view! {
        <>
            {children()}
        </>
    }
}
