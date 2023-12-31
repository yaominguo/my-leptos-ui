mod demo;

use demo::*;
use leptos::*;
use my_leptos_ui::*;

fn main() {
    mount_to_body(|| {
        view! {<App />}
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <MyProvider>
            // <ButtonDemo />
            <InputDemo />
        </MyProvider>
    }
}
