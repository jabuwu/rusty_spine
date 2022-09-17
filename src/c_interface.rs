use std::marker::PhantomData;

use crate::tmp_ref::{TmpRef, TmpRefMut};

pub trait NewFromPtr<C> {
    unsafe fn new_from_ptr(c_ptr: *const C) -> Self;
}

pub struct CArrayIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CArrayIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    pub(crate) fn new(parent: &'a P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: Default::default(),
        }
    }
}

impl<'a, P, T, C> Iterator for CArrayIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = TmpRef<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let item = unsafe { T::new_from_ptr(*self.items.offset(1)) };
            self.index += 1;
            Some(TmpRef::new(self._parent, item))
        } else {
            None
        }
    }
}

pub struct CArrayMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CArrayMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    pub(crate) fn new(parent: &'a P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: Default::default(),
        }
    }
}

impl<'a, P, T, C> Iterator for CArrayMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = TmpRefMut<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let item = unsafe { T::new_from_ptr(*self.items.offset(1)) };
            self.index += 1;
            Some(TmpRefMut::new(self._parent, item))
        } else {
            None
        }
    }
}
