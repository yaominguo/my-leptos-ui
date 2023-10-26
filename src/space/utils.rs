use crate::Size;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn get_name(&self) -> &'static str {
        match self {
            AlignItems::Start => "items-start",
            AlignItems::End => "items-end",
            AlignItems::Center => "items-center",
            AlignItems::Baseline => "items-baseline",
            AlignItems::Stretch => "items-stretch",
        }
    }
}

impl From<&'static str> for AlignItems {
    fn from(value: &'static str) -> Self {
        match value {
            "start" => AlignItems::Start,
            "end" => AlignItems::End,
            "center" => AlignItems::Center,
            "baseline" => AlignItems::Baseline,
            "stretch" => AlignItems::Stretch,
            _ => AlignItems::Start,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Around,
    Between,
    Evenly,
    Stretch,
}

impl JustifyContent {
    pub fn get_name(&self) -> &'static str {
        match self {
            JustifyContent::Start => "justify-start",
            JustifyContent::End => "justify-end",
            JustifyContent::Center => "justify-center",
            JustifyContent::Around => "justify-around",
            JustifyContent::Between => "justify-between",
            JustifyContent::Evenly => "justify-evenly",
            JustifyContent::Stretch => "justify-stretch",
        }
    }
}

impl From<&'static str> for JustifyContent {
    fn from(value: &'static str) -> Self {
        match value {
            "start" => JustifyContent::Start,
            "end" => JustifyContent::End,
            "center" => JustifyContent::Center,
            "around" => JustifyContent::Around,
            "between" => JustifyContent::Between,
            "evenly" => JustifyContent::Evenly,
            "stretch" => JustifyContent::Stretch,
            _ => JustifyContent::Start,
        }
    }
}

pub fn get_class(
    size: Size,
    align: AlignItems,
    justify: JustifyContent,
    inline: bool,
    vertical: bool,
    wrap: bool,
) -> String {
    let mut classes = vec![];

    if inline {
        classes.push("inline-flex");
    } else {
        classes.push("flex");
    }

    if vertical {
        classes.push("flex-col");
    }

    if wrap {
        classes.push("flex-wrap");
    }

    classes.push(justify.get_name());

    classes.push(align.get_name());

    match size {
        Size::Mini => {
            classes.push("gap-y-1");
            classes.push("gap-x-1");
        }
        Size::Small => {
            classes.push("gap-y-1");
            classes.push("gap-x-2");
        }
        Size::Medium => {
            classes.push("gap-y-2");
            classes.push("gap-x-3");
        }
        Size::Large => {
            classes.push("gap-y-3");
            classes.push("gap-x-4");
        }
    }

    classes.join(" ")
}
