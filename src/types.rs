use leptos::{ReadSignal, Signal, SignalGet};

#[derive(Debug, Clone, Copy)]
pub struct Prop<T: 'static>(pub Signal<T>);
impl<T: 'static + Clone> Prop<T> {
    pub fn new(value: T) -> Self {
        Self(Signal::derive(move || value.clone()))
    }
}
impl From<Prop<&'static str>> for &'static str {
    fn from(value: Prop<&'static str>) -> Self {
        value.0.get()
    }
}
impl From<Prop<String>> for String {
    fn from(value: Prop<String>) -> Self {
        value.0.get()
    }
}
impl From<Prop<bool>> for bool {
    fn from(value: Prop<bool>) -> Self {
        value.0.get()
    }
}
impl From<Prop<i32>> for i32 {
    fn from(value: Prop<i32>) -> Self {
        value.0.get()
    }
}
impl<T: 'static + Clone> From<T> for Prop<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
impl<T: 'static> From<ReadSignal<T>> for Prop<T> {
    fn from(value: ReadSignal<T>) -> Self {
        Self(value.into())
    }
}
impl<T: 'static + Clone + Default> Default for Prop<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
