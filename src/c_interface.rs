//! Helper structs for interacting with C pointers in safe rust.

#![allow(clippy::all)]

use std::{
    ffi::{CStr, CString},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// Create a type from its underlying [`spine-c`](`crate::c`) pointer type.
pub trait NewFromPtr<C> {
    unsafe fn new_from_ptr(c_ptr: *mut C) -> Self;
}

/// A reference type to temporarily borrow two types at once, ensuring a parent's lifetime remains
/// valid throughout the lifetime of the child.
///
/// A lot of relationships are implicitly represented throughout the codebase, since their actual
/// relationship exists within transpiled C code. This struct allows creating a temporary link the
/// borrow checker can use to ensure correctness.
pub struct CTmpRef<'a, P, T> {
    pub(crate) data: T,
    pub(crate) parent: &'a P,
}

impl<'a, P, T> CTmpRef<'a, P, T> {
    pub const fn new(parent: &'a P, data: T) -> Self {
        Self { data, parent }
    }

    pub fn unwrap_parent_child(&mut self) -> (&P, &T) {
        (self.parent, &self.data)
    }

    pub const fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<'a, P, T> Deref for CTmpRef<'a, P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, P, T: std::fmt::Debug> std::fmt::Debug for CTmpRef<'a, P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CTmpRef(")?;
        std::fmt::Debug::fmt(&**self, f)?;
        f.write_str(")")
    }
}

pub(crate) enum CTmpMutParent<'a, P> {
    Weak(*mut P),
    Strong(&'a mut P),
}

impl<'a, P> Deref for CTmpMutParent<'a, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Weak(pointer) => unsafe { &**pointer },
            Self::Strong(reference) => *reference,
        }
    }
}

/// A mutable version of [`CTmpRef`].
pub struct CTmpMut<'a, P, T> {
    pub(crate) data: T,
    pub(crate) parent: CTmpMutParent<'a, P>,
}

impl<'a, P, T> CTmpMut<'a, P, T> {
    #[must_use]
    pub fn new(parent: &'a mut P, data: T) -> Self {
        Self {
            data,
            parent: CTmpMutParent::Strong(parent),
        }
    }

    #[must_use]
    pub const fn new_weak(parent: *mut P, data: T) -> Self {
        Self {
            data,
            parent: CTmpMutParent::Weak(parent),
        }
    }

    #[must_use]
    pub fn unwrap_parent_child(&mut self) -> (&mut P, &mut T) {
        let parent = match &mut self.parent {
            CTmpMutParent::Strong(reference) => reference,
            CTmpMutParent::Weak(pointer) => unsafe { &mut **pointer },
        };
        (parent, &mut self.data)
    }

    #[must_use]
    pub const fn as_ref(&self) -> &T {
        &self.data
    }

    #[must_use]
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<'a, P, T> Deref for CTmpMut<'a, P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, P, T> DerefMut for CTmpMut<'a, P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, P, T: std::fmt::Debug> std::fmt::Debug for CTmpMut<'a, P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CTmpMut(")?;
        std::fmt::Debug::fmt(&**self, f)?;
        f.write_str(")")
    }
}

/// An iterator through a C array, maintaining a reference to a parent to ensure the data does not
/// become invalid while iterating.
pub struct CTmpPtrIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpPtrIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    #[must_use]
    pub(crate) fn new(parent: &'a P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: PhantomData::default(),
        }
    }
}

impl<'a, P, T, C> Iterator for CTmpPtrIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = CTmpRef<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let item = unsafe { T::new_from_ptr(*self.items.offset(self.index as isize)) };
            self.index += 1;
            Some(CTmpRef::new(self._parent, item))
        } else {
            None
        }
    }
}

/// A mutable version of [`CTmpPtrIterator`].
pub struct CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a mut P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    #[must_use]
    pub(crate) fn new(parent: &'a mut P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: PhantomData::default(),
        }
    }
}

impl<'a, P, T, C> Iterator for CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = CTmpMut<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let item = unsafe { T::new_from_ptr(*self.items.offset(self.index as isize)) };
            self.index += 1;
            Some(CTmpMut::new_weak(self._parent, item))
        } else {
            None
        }
    }
}

/// Similar to [`CTmpPtrIterator`], with some support for optional (null) items.
pub struct CTmpPtrNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpPtrNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    #[must_use]
    pub(crate) fn new(parent: &'a P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: PhantomData::default(),
        }
    }
}

impl<'a, P, T, C> Iterator for CTmpPtrNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = Option<CTmpRef<'a, P, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let ptr = unsafe { *self.items.offset(self.index as isize) };
            if !ptr.is_null() {
                let item = unsafe { T::new_from_ptr(ptr) };
                self.index += 1;
                Some(Some(CTmpRef::new(self._parent, item)))
            } else {
                self.index += 1;
                Some(None)
            }
        } else {
            None
        }
    }
}

impl<'a, P, T, C> Iterator for CTmpMutNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = Option<CTmpMut<'a, P, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let ptr = unsafe { *self.items.offset(self.index as isize) };
            if !ptr.is_null() {
                let item = unsafe { T::new_from_ptr(ptr) };
                self.index += 1;
                Some(Some(CTmpMut::new_weak(self._parent, item)))
            } else {
                self.index += 1;
                Some(None)
            }
        } else {
            None
        }
    }
}

/// Similar to [`CTmpMutIterator`], with some support for optional (null) items.
pub struct CTmpMutNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a mut P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpMutNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    #[must_use]
    pub(crate) fn new(parent: &'a mut P, items: *mut *mut C, count: usize) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            _marker: PhantomData::default(),
        }
    }
}

/// A wrapper over a raw pointer with the [`Send`] and [`Sync`] traits.
#[derive(Clone, Copy)]
pub struct SyncPtr<T>(pub *mut T);
unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

impl<T> std::fmt::Debug for SyncPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.0))
    }
}

impl<T> Hash for SyncPtr<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for SyncPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for SyncPtr<T> {}

impl<T> PartialOrd for SyncPtr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for SyncPtr<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

macro_rules! c_ptr {
    ($member:ident, $c_type:ty) => {
        /// Get a pointer to the underlying [`spine-c`](`crate::c`) type.
        #[inline]
        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub const fn c_ptr(&self) -> *mut $c_type {
            self.$member.0
        }

        #[inline]
        #[must_use]
        #[allow(dead_code, clippy::missing_const_for_fn)]
        pub(crate) unsafe fn c_ptr_ref(&self) -> &$c_type {
            &*self.$member.0
        }

        #[inline]
        #[must_use]
        #[allow(dead_code, clippy::mut_from_ref, clippy::missing_const_for_fn)]
        pub(crate) unsafe fn c_ptr_mut(&self) -> &mut $c_type {
            &mut *self.$member.0
        }
    };
}

macro_rules! c_accessor {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty) => {
        c_accessor_for!(c_ptr_ref, $(#[$($attrss)*])* $rust, $c, $type);
    };
}

macro_rules! c_accessor_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_mut_for!(
            c_ptr_ref,
            c_ptr_mut,
            $(#[$($attrss1)*])*
            $rust,
            $(#[$($attrss2)*])*
            $rust_set,
            $c,
            $type
        );
    };
}

macro_rules! c_accessor_for {
    ($for:ident, $(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty) => {
        $(#[$($attrss)*])*
        #[inline]
        #[must_use]
        pub fn $rust(&self) -> $type {
            #[allow(unused_unsafe)]
            unsafe {
                self.$for().$c as $type
            }
        }
    };
}

macro_rules! c_accessor_mut_for {
    ($for:ident, $for_mut:ident, $(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_for!(
            $for,
            $(#[$($attrss1)*])*
            $rust,
            $c,
            $type
        );
        $(#[$($attrss2)*])*
        #[inline]
        pub fn $rust_set(&mut self, value: $type) {
            unsafe {
                self.$for_mut().$c = value;
            }
        }
    };
}

macro_rules! c_accessor_string {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident) => {
        $(#[$($attrss)*])*
        #[inline]
        #[must_use]
        pub fn $rust(&self) -> &str {
            unsafe {
                if !self.c_ptr_ref().$c.is_null() {
                    crate::c_interface::from_c_str(std::ffi::CStr::from_ptr(self.c_ptr_ref().$c))
                } else {
                    ""
                }
            }
        }
    };
}

macro_rules! c_accessor_string_mut {
    ($(#[$($attrss:tt)*])* $rust:ident, $rust_set:ident, $c:ident) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> &str {
            unsafe {
                if !self.c_ptr_ref().$c.is_null() {
                    crate::c_interface::from_c_str(std::ffi::CStr::from_ptr(self.c_ptr_ref().$c))
                } else {
                    ""
                }
            }
        }

        /// # Errors
        ///
        /// Returns [`std::ffi::NulError`] if an interior nul byte is found.
        $(#[$($attrss)*])*
        pub fn $rust_set(&mut self, value: String) -> Result<(), std::ffi::NulError> {
            let c_str = std::ffi::CString::new(value)?;
            unsafe {
                self.c_ptr_mut().$c = c_str.into_raw();
            }
            Ok(())
        }
    };
}

macro_rules! c_accessor_string_optional {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident) => {
        $(#[$($attrss)*])*
        #[inline]
        #[must_use]
        pub fn $rust(&self) -> Option<&str> {
            unsafe {
                if !self.c_ptr_ref().$c.is_null() {
                    Some(crate::c_interface::from_c_str(std::ffi::CStr::from_ptr(self.c_ptr_ref().$c)))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! c_accessor_bool {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> bool {
            unsafe { self.c_ptr_ref().$c != 0 }
        }
    };
}

macro_rules! c_accessor_bool_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_set:ident, $c:ident) => {
        c_accessor_bool!($(#[$($attrss1)*])* $rust, $c);
        $(#[$($attrss2)*])*
        pub fn $rust_set(&mut self, value: bool) {
            unsafe {
                self.c_ptr_mut().$c = if value { 1 } else { 0 };
            }
        }
    };
}

macro_rules! c_accessor_color {
    ($(#[$($attrss1:tt)*])* $rust:ident, $c:ident) => {
        $(#[$($attrss1)*])*
        #[must_use]
        pub fn $rust(&self) -> crate::color::Color {
            unsafe {
                *((&self.c_ptr_ref().$c as *const crate::c::spColor).cast::<crate::color::Color>())
            }
        }
    };
}

macro_rules! c_accessor_color_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_mut:ident, $c:ident) => {
        c_accessor_color!(
            $(#[$($attrss1)*])*
            $rust,
            $c
        );
        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_mut(&mut self) -> &mut crate::color::Color {
            unsafe {
                &mut *(&mut self.c_ptr_mut().color as *mut crate::c::spColor)
                    .cast::<crate::color::Color>()
            }
        }
    };
}

macro_rules! c_accessor_color_optional {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> Option<crate::color::Color> {
            unsafe {
                let ptr = *(&self.c_ptr_ref().$c);
                if !ptr.is_null() {
                    Some(*(ptr).cast::<crate::color::Color>())
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! c_accessor_enum {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> $type {
            unsafe { self.c_ptr_ref().$c.into() }
        }
    };
}

/*macro_rules! c_accessor_enum_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_enum!(
            $(#[$($attrss1)*])*
            $rust,
            $c,
            $type
        );
        $(#[$($attrss2)*])*
        pub fn $rust_set(&self, value: $type) {
            unsafe {
                (*self.c_ptr()).$c = value as u32;
            }
        }
    };
}*/

macro_rules! c_accessor_renderer_object {
    () => {
        #[must_use]
        pub fn renderer_object(&self) -> crate::renderer_object::RendererObject {
            crate::renderer_object::RendererObject::new(unsafe {
                &mut self.c_ptr_mut().rendererObject
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty, $c_type:ident) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> crate::c_interface::CTmpRef<Self, $type> {
            crate::c_interface::CTmpRef::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        c_accessor_tmp_ptr!(
            $(#[$($attrss1)*])*
            $rust,
            $c,
            $type,
            $c_type
        );
        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_mut(&mut self) -> crate::c_interface::CTmpMut<Self, $type> {
            crate::c_interface::CTmpMut::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr_optional{
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty, $c_type:ident) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            let ptr = unsafe { self.c_ptr_ref().$c };
            if !ptr.is_null() {
                Some(crate::c_interface::CTmpRef::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_tmp_ptr_optional_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        c_accessor_tmp_ptr_optional!(
            $(#[$($attrss1)*])*
            $rust,
            $c,
            $type,
            $c_type
        );
        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_mut(&mut self) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            let ptr = unsafe { self.c_ptr_ref().$c };
            if !ptr.is_null() {
                Some(crate::c_interface::CTmpMut::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_passthrough {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty) => {
        $(#[$($attrss)*])*
        #[must_use]
        pub fn $rust(&self) -> $type {
            unsafe { self.c_ptr_ref().$c }
        }
    };
}

macro_rules! c_accessor_array {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_index:ident, $parent_type:ty, $type:ty, $c_type:ty, $c:ident, $count_fn:ident) => {
        $(#[$($attrss1)*])*
        #[must_use]
        pub fn $rust(&self) -> crate::c_interface::CTmpPtrIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_index(
            &self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() {
                Some(crate::c_interface::CTmpRef::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_ref().$c.add(index),
                    )
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_array_mut {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_mut:ident, $(#[$($attrss3:tt)*])* $rust_index:ident, $(#[$($attrss4:tt)*])* $rust_index_mut:ident, $parent_type:ty, $type:ty, $c_type:ty, $c:ident, $count_fn:ident) => {
        c_accessor_array!(
            $(#[$($attrss1)*])*
            $rust,
            $(#[$($attrss3)*])*
            $rust_index,
            $parent_type,
            $type,
            $c_type,
            $c,
            $count_fn
        );

        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        $(#[$($attrss4)*])*
        #[must_use]
        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() {
                Some(crate::c_interface::CTmpMut::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_mut().$c.add(index),
                    )
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_array_nullable {
    ($(#[$($attrss1:tt)*])* $rust:ident, $(#[$($attrss2:tt)*])* $rust_mut:ident, $(#[$($attrss3:tt)*])* $rust_index:ident, $(#[$($attrss4:tt)*])* $rust_index_mut:ident, $parent_type:ty, $type:ty, $c_type:ty, $c:ident, $count_fn:ident) => {
        $(#[$($attrss1)*])*
        #[must_use]
        pub fn $rust(
            &self,
        ) -> crate::c_interface::CTmpPtrNullableIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrNullableIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        $(#[$($attrss2)*])*
        #[must_use]
        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutNullableIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutNullableIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        $(#[$($attrss3)*])*
        #[must_use]
        pub fn $rust_index(
            &self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() as usize {
                let ptr = unsafe { *self.c_ptr_ref().$c.add(index) };
                if !ptr.is_null() {
                    Some(crate::c_interface::CTmpRef::new(self, unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }

        $(#[$($attrss4)*])*
        #[must_use]
        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() as usize {
                let ptr = unsafe { *self.c_ptr_ref().$c.add(index) };
                if !ptr.is_null() {
                    Some(crate::c_interface::CTmpMut::new(self, unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_slice_for {
    ($for:ident, $(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty, $len:ident) => {
        $(#[$($attrss)*])*
        #[inline]
        #[must_use]
        pub fn $rust(&self) -> $type {
            #[allow(unused_unsafe)]
            unsafe {
                std::slice::from_raw_parts(self.$for().$c, self.$for().$len as usize)
                    .try_into()
                    .unwrap()
            }
        }
    };
}

macro_rules! c_accessor_fixed_slice_optional {
    ($(#[$($attrss:tt)*])* $rust:ident, $c:ident, $type:ty, $len:literal) => {
        $(#[$($attrss)*])*
        #[inline]
        #[must_use]
        pub fn $rust(&self) -> Option<$type> {
            #[allow(unused_unsafe)]
            unsafe {
                let ptr = self.c_ptr_ref().$c;
                if !ptr.is_null() {
                    Some(std::slice::from_raw_parts(ptr, $len).try_into().unwrap())
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! c_attachment_accessors {
    () => {
        #[inline]
        #[must_use]
        pub fn name(&self) -> &str {
            unsafe {
                crate::c_interface::from_c_str(std::ffi::CStr::from_ptr(self.attachment().name))
            }
        }

        #[inline]
        #[must_use]
        pub fn attachment_type(&self) -> crate::attachment::AttachmentType {
            self.attachment().type_0.into()
        }
    };
}

macro_rules! c_vertex_attachment_accessors {
    () => {
        /// # Safety
        ///
        /// The slot passed in must be the same slot this attachment originated from.
        #[inline]
        pub unsafe fn compute_world_vertices(
            &self,
            slot: &crate::slot::Slot,
            start: i32,
            count: i32,
            world_vertices: &mut [f32],
            offset: i32,
            stride: i32,
        ) {
            crate::c::spVertexAttachment_computeWorldVertices(
                self.vertex_attachment() as *const crate::c::spVertexAttachment
                    as *mut crate::c::spVertexAttachment,
                slot.c_ptr(),
                start,
                count,
                world_vertices.as_mut_ptr(),
                offset,
                stride,
            );
        }

        c_accessor_for!(
            vertex_attachment,
            world_vertices_length,
            worldVerticesLength,
            i32
        );
        c_accessor_for!(vertex_attachment, id, id, i32);

        c_accessor_slice_for!(vertex_attachment, bones, bones, &[i32], bonesCount);
        c_accessor_slice_for!(
            vertex_attachment,
            /// Gets the raw float array slice representing the vertices of the attachment. If using
            /// the `mint` feature, the [`Self::vertices2`] function may be more convenient to use.
            vertices,
            vertices,
            &[f32],
            verticesCount
        );

        // TODO: accessor for timelineAttachment
    };
}

#[cfg(feature = "mint")]
macro_rules! c_vertex_attachment_accessors_mint {
    () => {
        /// Gets the vertices of the attachment as a Vector2 slice.
        #[must_use]
        pub fn vertices2(&self) -> &[mint::Vector2<f32>] {
            unsafe {
                std::slice::from_raw_parts(
                    self.vertex_attachment()
                        .vertices
                        .cast::<mint::Vector2<f32>>(),
                    self.vertex_attachment().verticesCount as usize / 2,
                )
                .try_into()
                .unwrap()
            }
        }
    };
}

macro_rules! c_handle_decl {
    ($(#[$($attrss:tt)*])* $name:ident, $type:ident, $parent:ident, $c_type:ident, $c_parent:ident) => {
        $(#[$($attrss)*])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            c_item: crate::c_interface::SyncPtr<$c_type>,
            c_parent: crate::c_interface::SyncPtr<$c_parent>,
        }

        impl $name {
            #[must_use]
            pub(crate) const fn new(c_item: *const $c_type, c_parent: *const $c_parent) -> Self {
                Self {
                    c_item: SyncPtr(c_item as *mut $c_type),
                    c_parent: SyncPtr(c_parent as *mut $c_parent),
                }
            }

            /// Safely acquired the item, verifying its existence using its parent.
            #[must_use]
            pub fn get<'a>(
                &self,
                parent: &'a $parent,
            ) -> Option<crate::c_interface::CTmpRef<'a, $parent, $type>> {
                if parent.c_ptr() == self.c_parent.0 {
                    Some(crate::c_interface::CTmpRef::new(parent, unsafe {
                        <$type>::new_from_ptr(self.c_item.0)
                    }))
                } else {
                    None
                }
            }

            /// Safely acquired the item, verifying its existence using its parent.
            #[must_use]
            pub fn get_mut<'a>(
                &self,
                parent: &'a mut $parent,
            ) -> Option<crate::c_interface::CTmpMut<'a, $parent, $type>> {
                if parent.c_ptr() == self.c_parent.0 {
                    Some(crate::c_interface::CTmpMut::new(parent, unsafe {
                        <$type>::new_from_ptr(self.c_item.0)
                    }))
                } else {
                    None
                }
            }

            /// # Safety
            ///
            /// Acquire the item without any checks. This is a direct pointer access which is fast
            /// but will segfault if the data has been disposed of already.
            #[must_use]
            pub unsafe fn get_unchecked(&self) -> $type {
                <$type>::new_from_ptr(self.c_item.0)
            }
        }
    };
}

macro_rules! c_handle_indexed_decl {
    ($(#[$($attrss:tt)*])* $name:ident, $type:ty, $parent:ty, $c_type:ident, $c_parent:ident) => {
        $(#[$($attrss)*])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            index: i32,
            c_item: crate::c_interface::SyncPtr<$c_type>,
            c_parent: crate::c_interface::SyncPtr<$c_parent>,
        }

        impl $name {
            #[must_use]
            pub(crate) const fn new(
                index: i32,
                c_item: *const $c_type,
                c_parent: *const $c_parent,
            ) -> Self {
                Self {
                    index,
                    c_item: SyncPtr(c_item as *mut $c_type),
                    c_parent: SyncPtr(c_parent as *mut $c_parent),
                }
            }

            /// Safely acquired the item, verifying its existence using its parent.
            #[must_use]
            pub fn get<'a>(
                &self,
                parent: &'a $parent,
            ) -> Option<crate::c_interface::CTmpRef<'a, $parent, $type>> {
                if parent.c_ptr() == self.c_parent.0 {
                    if <$type>::handle_valid(self) {
                        Some(crate::c_interface::CTmpRef::new(parent, unsafe {
                            <$type>::new_from_ptr(self.c_item.0)
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            /// Safely acquired the item, verifying its existence using its parent.
            #[must_use]
            pub fn get_mut<'a>(
                &self,
                parent: &'a mut $parent,
            ) -> Option<crate::c_interface::CTmpMut<'a, $parent, $type>> {
                if parent.c_ptr() == self.c_parent.0 {
                    if <$type>::handle_valid(self) {
                        Some(crate::c_interface::CTmpMut::new(parent, unsafe {
                            <$type>::new_from_ptr(self.c_item.0)
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            /// # Safety
            ///
            /// Acquire the item without any checks. This is a direct pointer access which is fast
            /// but will segfault if the data has been disposed of already.
            #[must_use]
            pub unsafe fn get_unchecked(&self) -> $type {
                <$type>::new_from_ptr(self.c_item.0)
            }
        }
    };
}

/// Used to isolate this `unwrap()` in one place in the codebase. It is necessary to avoid proceeding
/// with corrupt data, but the panic (ideally) never happens.
pub(crate) fn to_c_str(rust_string: &str) -> CString {
    CString::new(rust_string).unwrap()
}

/// Used to isolate this `unwrap()` in one place in the codebase. It is necessary to avoid proceeding
/// with corrupt data, but the panic (ideally) never happens.
pub(crate) fn from_c_str(c_string: &CStr) -> &str {
    c_string.to_str().unwrap()
}
