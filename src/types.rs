use leptos::{ReadSignal, Signal, SignalGet};

#[derive(Debug, Clone, Copy)]
pub struct PropStr(pub Signal<&'static str>);
impl PropStr {
    pub fn new(value: &'static str) -> Self {
        Self(Signal::derive(move || value))
    }
}
impl From<PropStr> for &'static str {
    fn from(value: PropStr) -> Self {
        value.0.get()
    }
}
impl From<&'static str> for PropStr {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}
impl From<ReadSignal<&'static str>> for PropStr {
    fn from(value: ReadSignal<&'static str>) -> Self {
        Self(value.into())
    }
}
impl Default for PropStr {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PropBool(pub Signal<bool>);
impl PropBool {
    pub fn new(value: bool) -> Self {
        Self(Signal::derive(move || value))
    }
}
impl From<PropBool> for bool {
    fn from(value: PropBool) -> Self {
        value.0.get()
    }
}
impl From<bool> for PropBool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}
impl From<ReadSignal<bool>> for PropBool {
    fn from(value: ReadSignal<bool>) -> Self {
        Self(value.into())
    }
}
impl Default for PropBool {
    fn default() -> Self {
        Self::new(false)
    }
}
