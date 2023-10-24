use crate::MyInput;
use leptos::*;
use leptos_icons::AiIcon::{AiCheckCircleOutlined, AiSearchOutlined};
use leptos_icons::*;

#[component]
pub fn InputDemo() -> impl IntoView {
    view! {
        <div class="px-4">
            <h1>Input Demo</h1>
            <MyInput class="mb-4" size="mini">
                <span template="suffix" >
                    <Icon icon=Icon::from(AiSearchOutlined) class="text-gray-300" />
                </span>
            </MyInput>
            <MyInput class="mb-4" size="small">
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
            <MyInput class="mb-4" textarea=true></MyInput>
        </div>
    }
}
