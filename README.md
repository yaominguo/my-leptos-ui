# MY LEPTOS UI

UI library for leptos

### Usage

1. add dependencies into Cargo.toml

```toml
[dependencies]
leptos = { version = "0.5.1", features = ["csr"] }
my-leptos-ui = { version = "0.1.0", git = "https://github.com/yaominguo/my-leptos-ui" }
```

2. link the stylesheet in html

```html
<link rel="stylesheet" href="http://raw.githack.com/yaominguo/my-leptos-ui/main/style/output.css" />
```

3. use in leptos project

```rust
use leptos::*;
use my_leptos_ui::*;

fn main() {
    mount_to_body(|| {
        view! {<App />}
    })
}

#[component]
fn App() -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let change_text = move |_| set_text.set("Hello World!".to_string());
    view! {
        <MyProvider>
            <MySpace justify="center" align="center" style="height:100vh;">
                <MyInput value=text clearable=true size="small" />
                <MyButton on:click=change_text size="small">Change Text</MyButton>
            </MySpace>
        </MyProvider>
    }
}

```
