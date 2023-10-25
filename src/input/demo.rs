use crate::MyInput;
use leptos::logging::log;
use leptos::{ev::Event, *};
use leptos_icons::AiIcon::{AiCheckCircleOutlined, AiSearchOutlined};
use leptos_icons::*;

#[component]
pub fn InputDemo() -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let on_change = move |e: Event| {
        log!("hello~~~ {:?}", e.target().unwrap().value_of());
        set_text.set(event_target_value(&e))
    };
    let on_focus = move |_| log!("focus~~~");
    let on_blur = move |_| log!("blur~~~");
    view! {
        <input type="text" on:change=move |e| {
            log!("hello~~~ {:?}", e.target().unwrap().value_of());
            set_text.set(event_target_value(&e))
        }/>
        <button on:click=move|_|log!("{:?}", text.get())>button</button>
        <button on:click=move|_|set_text.set("hello".to_string())>button2</button>
        <div class="px-4">
            <h1>Input Demo</h1>
            <MyInput class="mb-4" size="mini">
                <span template="suffix">
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput class="mb-4" size="small" clearable=true value=text on:change=on_change on:focus=on_focus on:blur=on_blur>
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput class="mb-4">
                <span template="prefix">
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="text-gray-300" />
                </span>
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput class="mb-4" size="large">
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput class="mb-4" rounded=true disabled=true />
            <MyInput class="mb-4" textarea=true clearable=true />
        </div>
    }
}
