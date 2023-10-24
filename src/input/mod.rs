mod demo;
mod utils;

use crate::{input::utils::get_template_value, GlobalConfig, PropBool, PropStr, Size};
pub use demo::InputDemo;
use leptos::{
    component, create_signal, expect_context, view, watch, Children, IntoView, RwSignal, Show,
    SignalGet, SignalGetUntracked, SignalSet, View,
};
use std::str::FromStr;
use utils::get_class;

#[component]
pub fn MyInput(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: PropStr,
    #[prop(optional, into)] style: PropStr,
    #[prop(optional, into)] size: PropStr,
    #[prop(optional, into)] password: PropBool,
    #[prop(optional, into)] textarea: PropBool,
    #[prop(optional, into)] disabled: PropBool,
    #[prop(optional, into)] rounded: PropBool,
    #[prop(default = PropStr::new("请输入"), into)] placeholder: PropStr,
) -> impl IntoView {
    let config = expect_context::<RwSignal<GlobalConfig>>();
    let (input_class, set_input_class) = create_signal("".to_string());
    let _ = watch(
        move || {
            (
                class.0.get(),
                style.0.get(),
                size.0.get(),
                password.0.get(),
                textarea.0.get(),
                disabled.0.get(),
                rounded.0.get(),
                placeholder.0.get(),
            )
        },
        move |_, _, _| {
            let size = Size::from_str(size.into()).unwrap_or(config.get_untracked().size);
            let mut classes = get_class(size, disabled.into(), rounded.into());
            let class: &str = class.into();
            if !class.is_empty() {
                classes.push(' ');
                classes.push_str(class);
            }
            set_input_class.set(classes);
        },
        true,
    );

    let mut prefix: Option<View> = None;
    let mut suffix: Option<View> = None;
    if let Some(fragments) = children {
        fragments().nodes.into_iter().for_each(|child| {
            let name = get_template_value(child.clone());
            if name == "prefix" {
                prefix = Some(child);
            } else if name == "suffix" {
                suffix = Some(child);
            }
        });
    }

    let _type = move || match password.into() {
        true => "password",
        false => "text",
    };

    let _input_class = "outline-none flex-1 px-2 disabled:cursor-not-allowed";

    view! {
      <div class=input_class style=style.0>
        {prefix}
        <Show when=move || textarea.0.get() fallback=move || view! {<input class=_input_class type=_type disabled=disabled.0 placeholder=placeholder.0 />}>
            <textarea class=_input_class type=_type disabled=disabled.0 placeholder=placeholder.0 />
        </Show>
        {suffix}
      </div>
    }
}
