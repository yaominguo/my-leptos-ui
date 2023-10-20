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
use my_leptos_ui::{MyButton, MyProvider};

fn main() {
    mount_to_body(|| {
        view! {<App />}
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <MyProvider>
            <h1>Buttons</h1>
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
            <MyButton mode="primary" ghost=true>
                primary
            </MyButton>
            <MyButton mode="info" ghost=true dashed=true>
                info
            </MyButton>
            <MyButton mode="warning" rounded=true>
                warning
            </MyButton>
            <MyButton mode="error" disabled=true>
                error
            </MyButton>
        </MyProvider>
    }
}

```
