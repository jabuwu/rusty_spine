//! Helper structs for interacting with C pointers in safe rust.

use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait NewFromPtr<C> {
    unsafe fn new_from_ptr(c_ptr: *const C) -> Self;
}

pub struct CTmpRef<'a, P, T> {
    pub(crate) data: T,
    pub(crate) parent: &'a P,
}

impl<'a, P, T> CTmpRef<'a, P, T> {
    pub fn new(parent: &'a P, data: T) -> Self {
        Self { data, parent }
    }

    pub fn unwrap_parent_child(&mut self) -> (&P, &T) {
        (self.parent, &self.data)
    }

    pub fn as_ref(&self) -> &T {
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

pub struct CTmpMut<'a, P, T> {
    pub(crate) data: T,
    pub(crate) parent: &'a mut P,
}

impl<'a, P, T> CTmpMut<'a, P, T> {
    pub fn new(parent: &'a mut P, data: T) -> Self {
        Self { data, parent }
    }

    pub fn unwrap_parent_child(&mut self) -> (&mut P, &mut T) {
        (self.parent, &mut self.data)
    }

    pub fn as_ref(&self) -> &T {
        &self.data
    }

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

pub struct CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpMutIterator<'a, P, T, C>
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

impl<'a, P, T, C> Iterator for CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    type Item = CTmpMut<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let item = unsafe { T::new_from_ptr(*self.items.offset(self.index as isize)) };
            self.index += 1;
            Some(CTmpMut::new(
                unsafe { &mut *(self._parent as *const P as *mut P) },
                item,
            ))
        } else {
            None
        }
    }
}

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
                Some(Some(CTmpMut::new(
                    unsafe { &mut *(self._parent as *const P as *mut P) },
                    item,
                )))
            } else {
                self.index += 1;
                Some(None)
            }
        } else {
            None
        }
    }
}

pub struct CTmpMutNullableIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    _parent: &'a P,
    items: *mut *mut C,
    index: usize,
    count: usize,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpMutNullableIterator<'a, P, T, C>
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
        #[inline]
        pub fn c_ptr(&self) -> *mut $c_type {
            self.$member.0
        }

        #[inline]
        #[allow(dead_code)]
        pub(crate) unsafe fn c_ptr_ref(&self) -> &$c_type {
            &*self.$member.0
        }

        #[inline]
        #[allow(dead_code)]
        pub(crate) unsafe fn c_ptr_mut(&self) -> &mut $c_type {
            &mut *self.$member.0
        }
    };
}

macro_rules! c_accessor {
    ($rust:ident, $c:ident, $type:ty) => {
        c_accessor_for!(c_ptr_ref, $rust, $c, $type);
    };
}

macro_rules! c_accessor_mut {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_mut_for!(c_ptr_ref, c_ptr_mut, $rust, $rust_set, $c, $type);
    };
}

macro_rules! c_accessor_for {
    ($for:ident, $rust:ident, $c:ident, $type:ty) => {
        #[inline]
        pub fn $rust(&self) -> $type {
            #[allow(unused_unsafe)]
            unsafe {
                self.$for().$c
            }
        }
    };
}

macro_rules! c_accessor_mut_for {
    ($for:ident, $for_mut:ident, $rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_for!($for, $rust, $c, $type);
        #[inline]
        pub fn $rust_set(&mut self, value: $type) {
            unsafe {
                self.$for_mut().$c = value;
            }
        }
    };
}

macro_rules! c_accessor_string {
    ($rust:ident, $c:ident) => {
        #[inline]
        pub fn $rust(&self) -> &str {
            unsafe {
                if !self.c_ptr_ref().$c.is_null() {
                    std::ffi::CStr::from_ptr(self.c_ptr_ref().$c)
                        .to_str()
                        .unwrap()
                } else {
                    ""
                }
            }
        }
    };
}

macro_rules! c_accessor_bool {
    ($rust:ident, $c:ident) => {
        pub fn $rust(&self) -> bool {
            unsafe { self.c_ptr_ref().$c != 0 }
        }
    };
}

macro_rules! c_accessor_bool_mut {
    ($rust:ident, $rust_set:ident, $c:ident) => {
        c_accessor_bool!($rust, $c);
        pub fn $rust_set(&mut self, value: bool) {
            unsafe {
                self.c_ptr_mut().$c = if value { 1 } else { 0 };
            }
        }
    };
}

macro_rules! c_accessor_color {
    ($rust:ident, $c:ident) => {
        pub fn $rust(&self) -> crate::color::Color {
            unsafe {
                *(&self.c_ptr_ref().$c as *const crate::c::spColor as *const crate::color::Color)
            }
        }
    };
}

macro_rules! c_accessor_color_mut {
    ($rust:ident, $rust_mut:ident, $c:ident) => {
        c_accessor_color!($rust, $c);
        pub fn $rust_mut(&mut self) -> &mut crate::color::Color {
            unsafe {
                &mut *(&self.c_ptr_mut().$c as *const crate::c::spColor as *mut crate::color::Color)
            }
        }
    };
}

macro_rules! c_accessor_color_optional {
    ($rust:ident, $c:ident) => {
        pub fn $rust(&self) -> Option<crate::color::Color> {
            unsafe {
                let ptr = *(&self.c_ptr_ref().$c);
                if !ptr.is_null() {
                    Some(*(ptr as *const crate::c::spColor as *const crate::color::Color))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! c_accessor_enum {
    ($rust:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> $type {
            unsafe { self.c_ptr_ref().$c.into() }
        }
    };
}

macro_rules! c_accessor_enum_mut {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_enum!($rust, $c, $type);
        pub fn $rust_set(&self, value: $type) {
            unsafe {
                (*self.c_ptr()).$c = value as u32;
            }
        }
    };
}

/*macro_rules! c_accessor_enum_mut {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_enum!($rust, $c, $type);
        pub fn $rust_set(&mut self, value: $type) {
            self.c_ptr_mut().$c = value as u32;
        }
    };
}*/

macro_rules! c_accessor_renderer_object {
    () => {
        pub fn renderer_object(&self) -> crate::renderer_object::RendererObject {
            crate::renderer_object::RendererObject::new(unsafe {
                &mut self.c_ptr_mut().rendererObject
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpRef<Self, $type> {
            crate::c_interface::CTmpRef::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
        pub fn $rust_mut(&mut self) -> crate::c_interface::CTmpMut<Self, $type> {
            crate::c_interface::CTmpMut::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr_optional {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
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

#[cfg_attr(feature="spine38", allow(unused_macros))]
macro_rules! c_accessor_super {
    ($rust:ident, $rust_mut:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpRef<Self, $type> {
            crate::c_interface::CTmpRef::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    &mut self.c_ptr_mut().super_0,
                )
            })
        }
        pub fn $rust_mut(&mut self) -> crate::c_interface::CTmpMut<Self, $type> {
            crate::c_interface::CTmpMut::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    &mut self.c_ptr_mut().super_0,
                )
            })
        }
    };
}

macro_rules! c_accessor_passthrough {
    ($rust:ident, $c:ident, $type:ty) => {
        pub unsafe fn $rust(&self) -> $type {
            self.c_ptr_ref().$c
        }
    };
}

macro_rules! c_accessor_array {
    ($(#[$($attrss:tt)*])* $rust:ident, $rust_mut:ident, $rust_index:ident, $rust_index_mut:ident, $parent_type:ty, $type:ty, $c_type:ty, $c:ident, $count_fn:ident) => {
        $(#[$($attrss)*])*
        pub fn $rust(&self) -> crate::c_interface::CTmpPtrIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_index(
            &self,
            index: i32,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() {
                Some(crate::c_interface::CTmpRef::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_ref().$c.offset(index as isize),
                    )
                }))
            } else {
                None
            }
        }

        pub fn $rust_index_mut(
            &mut self,
            index: i32,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() {
                Some(crate::c_interface::CTmpMut::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_mut().$c.offset(index as isize),
                    )
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_array_nullable {
    ($rust:ident, $rust_mut:ident, $rust_index:ident, $rust_index_mut:ident, $parent_type:ty, $type:ty, $c_type:ty, $c:ident, $count_fn:ident) => {
        pub fn $rust(
            &self,
        ) -> crate::c_interface::CTmpPtrNullableIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrNullableIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutNullableIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutNullableIterator::new(
                self,
                unsafe { self.c_ptr_ref().$c },
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_index(
            &self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() as usize {
                let ptr = unsafe { *self.c_ptr_ref().$c.offset(index as isize) };
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

        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() as usize {
                let ptr = unsafe { *self.c_ptr_ref().$c.offset(index as isize) };
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

macro_rules! c_accessor_slice_optional {
    ($rust:ident, $c:ident, $type:ty, $len:literal) => {
        #[inline]
        pub fn $rust(&self) -> Option<$type> {
            #[allow(unused_unsafe)]
            unsafe {
                let ptr = self.c_ptr_ref().$c;
                if !ptr.is_null() {
                    Some(
                        std::slice::from_raw_parts(self.c_ptr_ref().$c, $len)
                            .try_into()
                            .unwrap(),
                    )
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
        pub fn name(&self) -> &str {
            unsafe {
                std::ffi::CStr::from_ptr(self.attachment().name)
                    .to_str()
                    .unwrap()
            }
        }

        #[inline]
        pub fn attachment_type(&self) -> crate::attachment::AttachmentType {
            self.attachment().type_0.into()
        }
    };
}

macro_rules! c_vertex_attachment_accessors {
    () => {
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

        // TODO: accessor for bones, timelineAttachment, vertices
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
            pub(crate) fn new(c_item: *const $c_type, c_parent: *const $c_parent) -> Self {
                Self {
                    c_item: SyncPtr(c_item as *mut $c_type),
                    c_parent: SyncPtr(c_parent as *mut $c_parent),
                }
            }

            /// Safely acquired the item, verifying its existence using its parent.
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

            /// Acquire the item without any checks. This is a direct pointer access which is fast
            /// but will segfault if the data has been disposed of already.
            pub unsafe fn get_unchecked<'a>(&self) -> $type {
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
            pub(crate) fn new(
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

            /// Acquire the item without any checks. This is a direct pointer access which is fast
            /// but will segfault if the data has been disposed of already.
            pub unsafe fn get_unchecked<'a>(&self) -> $type {
                <$type>::new_from_ptr(self.c_item.0)
            }
        }
    };
}
