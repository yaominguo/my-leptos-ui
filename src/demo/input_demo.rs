use crate::{MyInput, MySpace};
use leptos::logging::log;
use leptos::{ev::Event, *};
use leptos_icons::AiIcon::{AiCheckCircleOutlined, AiSearchOutlined};
use leptos_icons::*;
use my_leptos_ui::MyButton;

#[component]
pub fn InputDemo() -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let (len, set_len) = create_signal(2);
    let on_change = move |e: Event| {
        log!("change~~~");
        set_text.set(event_target_value(&e))
    };
    let on_focus = move |_| log!("focus~~~");
    let on_blur = move |_| log!("blur~~~");
    let on_clear = move |_: Event| log!("clear~~~");
    view! {
        <MySpace class="px-2" vertical=true align="stretch">
            <h1>Input Demo</h1>
            <MySpace>
                <MyButton on:click=move|_|set_len.set(10)>set maxlength to 10</MyButton>
                <MyButton on:click=move|_|set_text.set("hello".to_string())>set input value to hello</MyButton>
            </MySpace>
            <MyInput  size="small" clearable=true value=text on:change=on_change on:focus=on_focus on:blur=on_blur on:clear=on_clear>
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput  maxlength=len show_word_limit=true>
                <span template="prefix">
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="text-gray-300" />
                </span>
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput  size="large" maxlength=len show_word_limit=true>
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput  rounded=true disabled=true />
            <MyInput  textarea=true maxlength=100 show_word_limit=true />
        </MySpace>
    }
}
