use crate::MyButton;
use leptos::*;
use leptos_icons::AiIcon::AiCheckCircleOutlined;
use leptos_icons::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <h1>Button Demo</h1>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton>
                default
            </MyButton>
            <MyButton mode="primary">
                primary
            </MyButton>
            <MyButton mode="info">
                info
            </MyButton>
            <MyButton mode="warning">
                warning
            </MyButton>
            <MyButton mode="error">
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton>
                default
            </MyButton>
            <MyButton mode="primary" size="mini">
                primary
            </MyButton>
            <MyButton mode="info" size="small">
                info
            </MyButton>
            <MyButton mode="warning" size="medium">
                warning
            </MyButton>
            <MyButton mode="error" size="large">
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton plain=true>
                default
            </MyButton>
            <MyButton mode="primary" plain=true>
                primary
            </MyButton>
            <MyButton mode="info" plain=true>
                info
            </MyButton>
            <MyButton mode="warning" plain=true>
                warning
            </MyButton>
            <MyButton mode="error" plain=true>
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton ghost=true>
                default
            </MyButton>
            <MyButton mode="primary" ghost=true>
                primary
            </MyButton>
            <MyButton mode="info" ghost=true>
                info
            </MyButton>
            <MyButton mode="warning" ghost=true>
                warning
            </MyButton>
            <MyButton mode="error" ghost=true>
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton ghost=true dashed=true>
                default
            </MyButton>
            <MyButton mode="primary" ghost=true dashed=true>
                primary
            </MyButton>
            <MyButton mode="info" ghost=true dashed=true>
                info
            </MyButton>
            <MyButton mode="warning" ghost=true dashed=true>
                warning
            </MyButton>
            <MyButton mode="error" ghost=true dashed=true>
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton rounded=true>
                default
            </MyButton>
            <MyButton mode="primary" rounded=true>
                primary
            </MyButton>
            <MyButton mode="info" rounded=true>
                info
            </MyButton>
            <MyButton mode="warning" rounded=true>
                warning
            </MyButton>
            <MyButton mode="error" rounded=true>
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/3">
            <MyButton text=true>
                default
            </MyButton>
            <MyButton mode="primary" text=true>
                primary
            </MyButton>
            <MyButton mode="info" text=true>
                info
            </MyButton>
            <MyButton mode="warning" text=true>
                warning
            </MyButton>
            <MyButton mode="error" text=true>
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/2">
            <MyButton text=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                default
            </MyButton>
            <MyButton mode="primary">
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                primary
            </MyButton>
            <MyButton mode="info" ghost=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                info
            </MyButton>
            <MyButton mode="warning" rounded=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                warning
            </MyButton>
            <MyButton mode="error" ghost=true dashed=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                error
            </MyButton>
        </div>

        <div class="flex justify-around items-center mt-8 mb-8 w-1/2">
            <MyButton text=true disabled=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                default
            </MyButton>
            <MyButton mode="primary" disabled=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                primary
            </MyButton>
            <MyButton mode="info" ghost=true disabled=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                info
            </MyButton>
            <MyButton mode="warning" rounded=true disabled=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                warning
            </MyButton>
            <MyButton mode="error" ghost=true dashed=true disabled=true>
                <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                error
            </MyButton>
        </div>
    }
}
