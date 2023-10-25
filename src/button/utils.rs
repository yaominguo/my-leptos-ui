use crate::Size;

#[allow(clippy::too_many_arguments)]
pub fn get_class(
    size: Size,
    button_type: ButtonType,
    plain: bool,
    rounded: bool,
    ghost: bool,
    text: bool,
    dashed: bool,
    disabled: bool,
) -> String {
    let mut classes = vec![
        "flex",
        "items-center",
        "transition-all",
        "disabled:cursor-not-allowed",
    ];
    match size {
        Size::Mini => {
            classes.push("px-2");
            classes.push("text-sm");
        }
        Size::Small => {
            classes.push("px-4");
            classes.push("py-1");
        }
        Size::Medium => {
            classes.push("px-5");
            classes.push("py-2");
        }
        Size::Large => {
            classes.push("px-8");
            classes.push("py-3");
            classes.push("text-lg");
        }
        _ => (),
    }
    match button_type {
        ButtonType::Default => {
            if plain {
                classes.push("bg-gray-100");
                classes.push("hover:bg-gray-200 ");
                classes.push("text-black");
            } else if ghost {
                classes.push("border");
                classes.push("border-black");
                classes.push("hover:border-gray-700");
                classes.push("text-black");
                classes.push("hover:text-gray-700");
                if dashed {
                    classes.push("border-dashed");
                }
            } else if text {
                classes.push("text-black");
                classes.push("hover:text-gray-700");
            } else {
                classes.push("bg-white");
                classes.push("border");
                classes.push("border-gray-400");
                classes.push("hover:border-green-600");
                classes.push("text-black");
                classes.push("hover:text-green-600");
            }
        }
        ButtonType::Primary => {
            if plain {
                classes.push("bg-green-100");
                classes.push("hover:bg-green-200");
                classes.push("text-green-700 ");
            } else if ghost {
                classes.push("border");
                classes.push("border-green-700");
                classes.push("hover:border-green-500");
                classes.push("text-green-700");
                classes.push("hover:text-green-500");
                if dashed {
                    classes.push("border-dashed");
                }
            } else if text {
                classes.push("text-green-700");
                classes.push("hover:text-green-500");
            } else {
                classes.push("text-white");
                classes.push("bg-green-700");
                classes.push("hover:bg-green-600");
            }
        }
        ButtonType::Info => {
            if plain {
                classes.push("bg-blue-100");
                classes.push("hover:bg-blue-200");
                classes.push("text-blue-700");
            } else if ghost {
                classes.push("border");
                classes.push("border-blue-700");
                classes.push("hover:border-blue-500");
                classes.push("text-blue-700");
                classes.push("hover:text-blue-500");
                if dashed {
                    classes.push("border-dashed");
                }
            } else if text {
                classes.push("text-blue-700");
                classes.push("hover:text-blue-500");
            } else {
                classes.push("text-white");
                classes.push("bg-blue-700");
                classes.push("hover:bg-blue-600");
            }
        }
        ButtonType::Warning => {
            if plain {
                classes.push("bg-yellow-100");
                classes.push("hover:bg-yellow-200");
                classes.push("text-yellow-500");
            } else if ghost {
                classes.push("border");
                classes.push("border-yellow-500");
                classes.push("hover:border-yellow-300");
                classes.push("text-yellow-500");
                classes.push("hover:text-yellow-300");
                if dashed {
                    classes.push("border-dashed");
                }
            } else if text {
                classes.push("text-yellow-500");
                classes.push("hover:text-yellow-300");
            } else {
                classes.push("text-white");
                classes.push("bg-yellow-500");
                classes.push("hover:bg-yellow-400");
            }
        }
        ButtonType::Error => {
            if plain {
                classes.push("bg-red-100");
                classes.push("hover:bg-red-200");
                classes.push("text-red-700");
            } else if ghost {
                classes.push("border");
                classes.push("border-red-700");
                classes.push("hover:border-red-500");
                classes.push("text-red-700");
                classes.push("hover:text-red-500");
                if dashed {
                    classes.push("border-dashed");
                }
            } else if text {
                classes.push("text-red-700");
                classes.push("hover:text-red-500");
            } else {
                classes.push("text-white");
                classes.push("bg-red-700");
                classes.push("hover:bg-red-600");
            }
        }
    }

    if rounded {
        classes.push("rounded-full");
    } else {
        classes.push("rounded-sm");
    }

    if disabled {
        classes.retain(|x| !x.starts_with("hover:"));
        classes.push("opacity-70");
    }

    classes.join(" ")
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonType {
    Default,
    Primary,
    Info,
    Warning,
    Error,
}

impl From<&'static str> for ButtonType {
    fn from(value: &'static str) -> Self {
        match value {
            "default" => ButtonType::Default,
            "primary" => ButtonType::Primary,
            "info" => ButtonType::Info,
            "warning" => ButtonType::Warning,
            "error" => ButtonType::Error,
            _ => ButtonType::Default,
        }
    }
}
