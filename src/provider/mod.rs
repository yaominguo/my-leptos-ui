use crate::{GlobalConfig, Size};
use leptos::{component, create_rw_signal, provide_context, view, Children, IntoView};

#[component]
pub fn MyProvider(#[prop(optional)] size: &'static str, children: Children) -> impl IntoView {
    provide_context(create_rw_signal(GlobalConfig::new(
        Size::from(size).unwrap_or(Size::Medium),
    )));
    view! {
        <>
            {children()}
        </>
    }
}
