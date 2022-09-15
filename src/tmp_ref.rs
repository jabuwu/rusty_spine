use std::ops::{Deref, DerefMut};

pub struct TmpRef<'a, P, T> {
    data: T,
    _parent: &'a P,
}

impl<'a, P, T> TmpRef<'a, P, T> {
    pub fn new(parent: &'a P, data: T) -> Self {
        Self {
            data,
            _parent: parent,
        }
    }
}

impl<'a, P, T> Deref for TmpRef<'a, P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, P, T: std::fmt::Debug> std::fmt::Debug for TmpRef<'a, P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("*")?;
        std::fmt::Debug::fmt(&**self, f)
    }
}

pub struct TmpRefMut<'a, P, T> {
    data: T,
    _parent: &'a P,
}

impl<'a, P, T> TmpRefMut<'a, P, T> {
    pub fn new(parent: &'a P, data: T) -> Self {
        Self {
            data,
            _parent: parent,
        }
    }
}

impl<'a, P, T> Deref for TmpRefMut<'a, P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, P, T> DerefMut for TmpRefMut<'a, P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, P, T: std::fmt::Debug> std::fmt::Debug for TmpRefMut<'a, P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("*")?;
        std::fmt::Debug::fmt(&**self, f)
    }
}
