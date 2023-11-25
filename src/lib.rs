use std::{ops::Deref, sync::OnceLock};

pub struct DerefedOnceLock<T> {
    inner: OnceLock<T>,
}

impl<T> DerefedOnceLock<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn init(&self, value: T) {
        if self.inner.set(value).is_err() {
            panic!("Lazy is already initialized");
        }
    }

    pub fn is_init(&self) -> bool {
        self.inner.get().is_some()
    }
}

impl<T> Deref for DerefedOnceLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.get().unwrap()
    }
}
