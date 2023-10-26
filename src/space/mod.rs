mod utils;

use crate::{space::utils::get_class, GlobalConfig, Prop, Size};
use leptos::{
    component, create_signal, expect_context, view, watch, Children, IntoView, RwSignal, SignalGet,
    SignalSet,
};

#[component]
pub fn MySpace(
    children: Children,
    #[prop(optional, into)] class: Prop<&'static str>,
    #[prop(optional, into)] style: Prop<&'static str>,
    #[prop(optional, into)] size: Prop<&'static str>,
    #[prop(optional, into)] align: Prop<&'static str>,
    #[prop(optional, into)] justify: Prop<&'static str>,
    #[prop(optional, into)] inline: Prop<bool>,
    #[prop(optional, into)] vertical: Prop<bool>,
    #[prop(default=Prop::new(true), into)] wrap: Prop<bool>,
) -> impl IntoView {
    let config = expect_context::<RwSignal<GlobalConfig>>();
    let (space_class, set_space_class) = create_signal("".to_string());
    let _ = watch(
        move || {
            (
                class.0.get(),
                style.0.get(),
                size.0.get(),
                align.0.get(),
                justify.0.get(),
                inline.0.get(),
                vertical.0.get(),
                wrap.0.get(),
            )
        },
        move |new_value, past_value, _| {
            // don't get_class repeatedly if parameters are same like before
            if let Some(v) = past_value {
                if new_value == v {
                    return;
                }
            }
            let mut classes = get_class(
                Size::from(size.into()).unwrap_or(config.get().size),
                align.0.get().into(),
                justify.0.get().into(),
                inline.into(),
                vertical.into(),
                wrap.into(),
            );
            let class: &str = class.into();
            if !class.is_empty() {
                classes.push(' ');
                classes.push_str(class);
            }
            set_space_class.set(classes);
        },
        true,
    );
    view! {
        <div class=space_class style=style.0>
            {children()}
        </div>
    }
}
