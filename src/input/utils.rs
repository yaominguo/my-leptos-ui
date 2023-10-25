use crate::Size;
use leptos::View;

pub fn get_class(size: Size, disabled: bool, rounded: bool) -> String {
    let mut classes = vec![
        "relative",
        "flex",
        "items-center",
        "transition-all",
        "border",
        "hover:border-green-700",
        "focus-within:border-green-700",
    ];
    match size {
        Size::Mini => {
            classes.push("px-1");
            classes.push("text-sm");
        }
        Size::Small => {
            classes.push("px-1.5");
            classes.push("py-1");
        }
        Size::Medium => {
            classes.push("px-2");
            classes.push("py-1.5");
        }
        Size::Large => {
            classes.push("px-2.5");
            classes.push("py-2");
            classes.push("text-lg");
        }
        _ => (),
    }

    if rounded {
        classes.push("rounded-full");
    } else {
        classes.push("rounded-sm");
    }

    if disabled {
        classes.retain(|x| !x.starts_with("hover:"));
        classes.push("cursor-not-allowed opacity-70");
    }

    classes.join(" ")
}

pub fn get_template_value(view: View) -> String {
    view.into_html_element()
        .unwrap()
        .get_attribute("template")
        .unwrap_or("".to_string())
}
