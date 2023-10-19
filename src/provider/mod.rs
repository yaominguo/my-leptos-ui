use std::str::FromStr;

use leptos::{component, create_rw_signal, provide_context, view, Children, IntoView, ReadSignal};

use crate::{GlobalConfig, Size};

#[component]
pub fn MyProvider(
    theme: ReadSignal<&'static str>,
    #[prop(optional)] size: &'static str,
    children: Children,
) -> impl IntoView {
    let size = Size::from_str(size).unwrap_or(Size::Medium);
    provide_context(create_rw_signal(GlobalConfig::new(theme, size)));
    view! {
        <>
            {children()}
        </>
    }
}
