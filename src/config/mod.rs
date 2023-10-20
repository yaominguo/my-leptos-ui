use crate::Size;

#[derive(Clone, Debug)]
pub struct GlobalConfig {
    pub size: Size,
}

impl GlobalConfig {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}
