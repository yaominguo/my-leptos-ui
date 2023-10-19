use leptos::ReadSignal;

use crate::Size;

#[derive(Clone, Debug)]
pub struct GlobalConfig {
    pub theme: ReadSignal<&'static str>,
    pub size: Size,
}

impl GlobalConfig {
    pub fn new(theme: ReadSignal<&'static str>, size: Size) -> Self {
        Self { theme, size }
    }
}
