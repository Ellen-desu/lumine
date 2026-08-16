#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Remainder(Option<Box<str>>);

impl Remainder {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn get(&self) -> Option<&str> {
        self.0.as_deref()
    }

    pub fn is_empty(&self) -> bool {
        self.0.as_deref().is_none_or(str::is_empty)
    }
}

impl From<Box<str>> for Remainder {
    fn from(value: Box<str>) -> Self {
        Self(Some(value))
    }
}
