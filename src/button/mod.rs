mod demo;
mod utils;

use crate::{button::utils::ButtonType, GlobalConfig, Prop, Size};
pub use demo::ButtonDemo;
use leptos::{
    component, create_signal, expect_context, logging::log, view, watch, Children, IntoView,
    RwSignal, SignalGet, SignalGetUntracked, SignalSet,
};
use std::str::FromStr;
use utils::get_class;

#[component]
pub fn MyButton(
    children: Children,
    #[prop(optional, into)] class: Prop<&'static str>,
    #[prop(optional, into)] style: Prop<&'static str>,
    #[prop(optional, into)] size: Prop<&'static str>,
    #[prop(default = Prop::new("default"), into)] mode: Prop<&'static str>,
    #[prop(optional, into)] plain: Prop<bool>,
    #[prop(optional, into)] rounded: Prop<bool>,
    #[prop(optional, into)] ghost: Prop<bool>,
    #[prop(optional, into)] text: Prop<bool>,
    #[prop(optional, into)] dashed: Prop<bool>,
    #[prop(optional, into)] disabled: Prop<bool>,
) -> impl IntoView {
    let config = expect_context::<RwSignal<GlobalConfig>>();
    let (button_class, set_button_class) = create_signal("".to_string());
    let _ = watch(
        move || {
            (
                class.0.get(),
                style.0.get(),
                mode.0.get(),
                size.0.get(),
                plain.0.get(),
                rounded.0.get(),
                ghost.0.get(),
                text.0.get(),
                dashed.0.get(),
                disabled.0.get(),
            )
        },
        move |new_value, past_value, _| {
            // don't get_class repeatedly if parameters are same like before
            if let Some(v) = past_value {
                if new_value == v {
                    return;
                }
            }
            let size = Size::from_str(size.into()).unwrap_or(config.get_untracked().size);
            let button_type = ButtonType::from_str(mode.into()).unwrap_or(ButtonType::Default);
            let mut classes = get_class(
                size,
                button_type,
                plain.into(),
                rounded.into(),
                ghost.into(),
                text.into(),
                dashed.into(),
                disabled.into(),
            );
            log!("get : {}", &classes);
            let class: &str = class.into();
            if !class.is_empty() {
                classes.push(' ');
                classes.push_str(class);
            }
            set_button_class.set(classes);
        },
        true,
    );

    view! {
        <button class=button_class disabled=disabled.0 style=style.0>
            {children()}
        </button>
    }
}
