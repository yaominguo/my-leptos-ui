mod demo;
mod utils;

use crate::{input::utils::get_template_value, GlobalConfig, Prop, Size};
pub use demo::InputDemo;
use leptos::{
    component, create_node_ref, create_signal,
    ev::{Event, MouseEvent},
    event_target_value, expect_context,
    html::{Div, Input},
    view, watch, Children, IntoView, NodeRef, RwSignal, Show, SignalGet, SignalGetUntracked,
    SignalSet, View,
};
use leptos_icons::AiIcon::AiCloseCircleFilled;
use leptos_icons::Icon;
use std::str::FromStr;
use utils::get_class;

#[component]
pub fn MyInput(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] value: Prop<String>,
    #[prop(optional, into)] class: Prop<&'static str>,
    #[prop(optional, into)] style: Prop<&'static str>,
    #[prop(optional, into)] size: Prop<&'static str>,
    #[prop(optional, into)] password: Prop<bool>,
    #[prop(optional, into)] textarea: Prop<bool>,
    #[prop(optional, into)] disabled: Prop<bool>,
    #[prop(optional, into)] rounded: Prop<bool>,
    #[prop(optional, into)] clearable: Prop<bool>,
    #[prop(default = Prop::new("请输入"), into)] placeholder: Prop<&'static str>,
    #[prop(default = Prop::new(3), into)] rows: Prop<i32>,
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
        move |new_value, past_value, _| {
            // don't get_class repeatedly if parameters are same like before
            if let Some(v) = past_value {
                if new_value == v {
                    return;
                }
            }
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

    let (show_clear, set_show_clear) = create_signal(false);
    let handle_input = move |e| {
        if clearable.into() && !event_target_value(&e).is_empty() {
            set_show_clear.set(true);
        } else {
            set_show_clear.set(false);
        }
    };
    let input_element: NodeRef<Input> = create_node_ref();
    let div_element: NodeRef<Div> = create_node_ref();
    let handle_clear = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        if let Some(input) = input_element.get() {
            input.set_value("");
            let ev = Event::new("change").unwrap();
            ev.init_event_with_bubbles("change", true);
            if input.dispatch_event(&ev).is_ok() {
                set_show_clear.set(false);
            }
        }
    };
    let handle_focus = move |_| {
        if let Some(div) = div_element.get() {
            let ev = Event::new("focus").unwrap();
            let _ = div.dispatch_event(&ev);
        }
    };
    let handle_blur = move |_| {
        if let Some(div) = div_element.get() {
            let ev = Event::new("blur").unwrap();
            let _ = div.dispatch_event(&ev);
        }
    };

    view! {
      <div class=input_class style=style.0 node_ref=div_element>
        {prefix}
        <Show
            when=move || textarea.0.get()
            fallback=move || view! {
                <input
                    node_ref=input_element
                    prop:value=value.0
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                    class=_input_class
                    type=_type
                    disabled=disabled.0
                    placeholder=placeholder.0
                />}
        >
            <textarea
                prop:value=value.0
                on:input=handle_input
                on:focus=handle_focus
                on:blur=handle_blur
                class=_input_class
                rows=rows.0
                type=_type
                disabled=disabled.0
                placeholder=placeholder.0
            />
        </Show>
        <Show when=move || show_clear.get()>
            <button on:click=handle_clear>
                <Icon
                    class="text-gray-300 hover:text-gray-400 transition-all mr-1 visi"
                    icon=Icon::from(AiCloseCircleFilled)
                />
            </button>
        </Show>
        {suffix}
      </div>
    }
}
