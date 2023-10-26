use crate::{MyButton, MySpace};
use leptos::*;
use leptos_icons::AiIcon::AiCheckCircleOutlined;
use leptos_icons::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (disabled, set_disabled) = create_signal(true);
    let (size, set_size) = create_signal("mini");
    let handle_click = move |_| set_disabled.set(!disabled.get());
    let handle_size = move |_| set_size.set("large");
    view! {
        <MySpace class="px-2" vertical=true >
            <h1 class="text-lg">Button Demo</h1>
            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2" align="center">
                <MyButton>
                    default
                </MyButton>
                <MyButton mode="primary" on:click=handle_size size=size>
                    click to change
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
            </MySpace>
            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2">
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
            </MySpace>

            <MySpace class="mb-2">
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
                <MyButton mode="warning" rounded=true on:click=handle_click>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    switch disabled
                </MyButton>
                <MyButton mode="error" ghost=true dashed=true>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    error
                </MyButton>
            </MySpace>

            <MySpace class="mb-2">
                <MyButton text=true disabled=disabled>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    default
                </MyButton>
                <MyButton mode="primary" disabled=disabled>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    primary
                </MyButton>
                <MyButton mode="info" ghost=true disabled=disabled>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    info
                </MyButton>
                <MyButton mode="warning" rounded=true disabled=disabled>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    warning
                </MyButton>
                <MyButton mode="error" ghost=true dashed=true disabled=disabled>
                    <Icon icon=Icon::from(AiCheckCircleOutlined) class="mr-1" />
                    error
                </MyButton>
            </MySpace>
        </MySpace>
    }
}
