//! Helper structs for interacting with C pointers in safe rust.

use std::marker::PhantomData;

use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::error::Error;

pub trait NewFromPtr<C> {
    unsafe fn new_from_ptr(c_ptr: *const C) -> Self;
}

#[derive(Debug)]
pub struct CRefValidator<P> {
    id: Arc<AtomicU32>,
    _marker: PhantomData<P>,
}

impl<P> CRefValidator<P> {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Self {
            id: Arc::new(AtomicU32::new(id)),
            _marker: Default::default(),
        }
    }

    pub fn check(&self, id: u32) -> bool {
        self.id.load(Ordering::SeqCst) == id
    }

    pub fn invalidate(&mut self) {
        self.id.store(0, Ordering::SeqCst);
    }

    pub fn create_ref<T>(&self, value: T) -> CRef<P, T> {
        CRef {
            id: self.id.clone(),
            value,
            _marker: Default::default(),
        }
    }

    pub fn create_mut<T>(&self, value: T) -> CMut<P, T> {
        CMut {
            id: self.id.clone(),
            value,
            _marker: Default::default(),
        }
    }
}

impl<P> Clone for CRefValidator<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _marker: Default::default(),
        }
    }
}

pub trait CRefValidate {
    fn validate(&self, id: u32) -> bool;
}

pub struct CRef<P, T> {
    id: Arc<AtomicU32>,
    value: T,
    _marker: PhantomData<P>,
}

impl<P, T> CRef<P, T> {
    pub(crate) fn new_bad(value: T) -> Self {
        Self {
            id: Arc::new(AtomicU32::new(0)),
            value,
            _marker: Default::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.id.load(Ordering::SeqCst) != 0
    }
}

impl<P: CRefValidate, T> CRef<P, T> {
    pub fn get(&self, parent: &P) -> Result<&T, Error> {
        if self.is_valid() && parent.validate(self.id.load(Ordering::SeqCst)) {
            Ok(&self.value)
        } else {
            Err(Error::NotFound)
        }
    }

    pub unsafe fn get_unchecked(&self) -> Result<&T, Error> {
        if self.is_valid() {
            Ok(&self.value)
        } else {
            Err(Error::NotFound)
        }
    }
}

impl<P, T: std::fmt::Debug> std::fmt::Debug for CRef<P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            f.write_str("CRef(")?;
            std::fmt::Debug::fmt(&self.value, f)?;
            f.write_str(")")
        } else {
            f.write_str("CRef(invalid)")
        }
    }
}

pub struct CMut<P, T> {
    id: Arc<AtomicU32>,
    value: T,
    _marker: PhantomData<P>,
}

impl<P, T> CMut<P, T> {
    pub(crate) fn new_bad(value: T) -> Self {
        Self {
            id: Arc::new(AtomicU32::new(0)),
            value,
            _marker: Default::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.id.load(Ordering::SeqCst) != 0
    }
}

impl<P: CRefValidate, T> CMut<P, T> {
    pub fn get(&self, parent: &P) -> Result<&T, Error> {
        if self.is_valid() && parent.validate(self.id.load(Ordering::SeqCst)) {
            Ok(&self.value)
        } else {
            Err(Error::NotFound)
        }
    }

    pub unsafe fn get_unchecked(&self) -> Result<&T, Error> {
        if self.is_valid() {
            Ok(&self.value)
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn get_mut(&mut self, parent: &mut P) -> Result<&mut T, Error> {
        if self.is_valid() && parent.validate(self.id.load(Ordering::SeqCst)) {
            Ok(&mut self.value)
        } else {
            Err(Error::NotFound)
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self) -> Result<&mut T, Error> {
        if self.is_valid() {
            Ok(&mut self.value)
        } else {
            Err(Error::NotFound)
        }
    }
}

impl<P, T: std::fmt::Debug> std::fmt::Debug for CMut<P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            f.write_str("CMut(")?;
            std::fmt::Debug::fmt(&self.value, f)?;
            f.write_str(")")
        } else {
            f.write_str("CMut(invalid)")
        }
    }
}

pub struct CTmpRef<'a, P, T> {
    data: T,
    validator: Option<CRefValidator<P>>,
    _parent: &'a P,
}

impl<'a, P, T> CTmpRef<'a, P, T> {
    pub fn new(parent: &'a P, data: T, validator: Option<CRefValidator<P>>) -> Self {
        Self {
            data,
            validator,
            _parent: parent,
        }
    }

    pub fn keep(self) -> CRef<P, T> {
        if let Some(validator) = self.validator {
            validator.create_ref(self.data)
        } else {
            CRef::new_bad(self.data)
        }
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
    data: T,
    validator: Option<CRefValidator<P>>,
    _parent: &'a P,
}

impl<'a, P, T> CTmpMut<'a, P, T> {
    pub fn new(parent: &'a P, data: T, validator: Option<CRefValidator<P>>) -> Self {
        Self {
            data,
            validator,
            _parent: parent,
        }
    }

    pub fn keep(self) -> CMut<P, T> {
        if let Some(validator) = self.validator {
            validator.create_mut(self.data)
        } else {
            CMut::new_bad(self.data)
        }
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
    validator: Option<CRefValidator<P>>,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpPtrIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    pub(crate) fn new(
        parent: &'a P,
        items: *mut *mut C,
        count: usize,
        validator: Option<CRefValidator<P>>,
    ) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            validator,
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
            Some(CTmpRef::new(self._parent, item, self.validator.clone()))
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
    validator: Option<CRefValidator<P>>,
    _marker: PhantomData<T>,
}

impl<'a, P, T, C> CTmpMutIterator<'a, P, T, C>
where
    T: NewFromPtr<C>,
{
    pub(crate) fn new(
        parent: &'a P,
        items: *mut *mut C,
        count: usize,
        validator: Option<CRefValidator<P>>,
    ) -> Self {
        Self {
            _parent: parent,
            items,
            index: 0,
            count,
            validator,
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
            Some(CTmpMut::new(self._parent, item, self.validator.clone()))
        } else {
            None
        }
    }
}

macro_rules! c_ptr {
    ($member:ident, $c_type:ty) => {
        #[inline]
        pub fn c_ptr(&self) -> *mut $c_type {
            self.$member.0
        }

        #[inline]
        pub fn c_ptr_ref(&self) -> &$c_type {
            unsafe { &*self.$member.0 }
        }

        #[inline]
        pub fn c_ptr_mut(&self) -> &mut $c_type {
            unsafe { &mut *self.$member.0 }
        }
    };
}

macro_rules! c_accessor {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        c_accessor_for!(c_ptr_ref, c_ptr_mut, $rust, $rust_set, $c, $type);
    };
}

macro_rules! c_accessor_for {
    ($for:ident, $for_mut:ident, $rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        #[inline]
        pub fn $rust(&self) -> $type {
            self.$for().$c
        }
        #[inline]
        pub fn $rust_set(&mut self, value: $type) {
            self.$for_mut().$c = value;
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
    ($rust:ident, $rust_set:ident, $c:ident) => {
        pub fn $rust(&self) -> bool {
            self.c_ptr_ref().$c != 0
        }
        pub fn $rust_set(&mut self, value: bool) {
            self.c_ptr_mut().$c = if value { 1 } else { 0 };
        }
    };
}

macro_rules! c_accessor_color {
    ($rust:ident, $rust_mut:ident, $c:ident) => {
        pub fn $rust(&self) -> &crate::color::Color {
            unsafe {
                &*(&self.c_ptr_ref().$c as *const crate::c::spColor as *const crate::color::Color)
            }
        }
        pub fn $rust_mut(&mut self) -> &mut crate::color::Color {
            unsafe {
                &mut *(&self.c_ptr_mut().$c as *const crate::c::spColor as *mut crate::color::Color)
            }
        }
    };
}

macro_rules! c_accessor_enum_no_set {
    ($rust:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c.into()
        }
    };
}

macro_rules! c_accessor_enum {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c.into()
        }
        pub fn $rust_set(&mut self, value: $type) {
            self.c_ptr_mut().$c = value as u32;
        }
    };
}

macro_rules! c_accessor_renderer_object {
    () => {
        pub fn renderer_object(&self) -> crate::renderer_object::RendererObject {
            crate::renderer_object::RendererObject::new(&mut self.c_ptr_mut().rendererObject)
        }
    };
}

macro_rules! c_accessor_tmp_ptr {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpRef<Self, $type> {
            crate::c_interface::CTmpRef::new(
                self,
                unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        self.c_ptr_ref().$c,
                    )
                },
                None,
            )
        }
        pub fn $rust_mut(&mut self) -> crate::c_interface::CTmpMut<Self, $type> {
            crate::c_interface::CTmpMut::new(
                self,
                unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        self.c_ptr_ref().$c,
                    )
                },
                None,
            )
        }
    };
}

macro_rules! c_accessor_tmp_ptr_optional {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            let ptr = self.c_ptr_ref().$c;
            if !ptr.is_null() {
                Some(crate::c_interface::CTmpRef::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                    },
                    None,
                ))
            } else {
                None
            }
        }
        pub fn $rust_mut(&mut self) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            let ptr = self.c_ptr_ref().$c;
            if !ptr.is_null() {
                Some(crate::c_interface::CTmpMut::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                    },
                    None,
                ))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_super {
    ($rust:ident, $rust_mut:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpRef<Self, $type> {
            crate::c_interface::CTmpRef::new(
                self,
                unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        &mut self.c_ptr_mut().super_0,
                    )
                },
                None,
            )
        }
        pub fn $rust_mut(&mut self) -> crate::c_interface::CTmpMut<Self, $type> {
            crate::c_interface::CTmpMut::new(
                self,
                unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        &mut self.c_ptr_mut().super_0,
                    )
                },
                None,
            )
        }
    };
}

macro_rules! c_accessor_passthrough {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $type_mut:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c
        }

        pub fn $rust_mut(&mut self) -> $type_mut {
            self.c_ptr_mut().$c as $type_mut
        }
    };
}

macro_rules! c_accessor_array {
    ($rust:ident, $rust_mut:ident, $rust_index:ident, $rust_index_mut:ident, $parent_type:ident, $type:ident, $c_type:ident, $c:ident, $count_fn:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpPtrIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
                None,
            )
        }

        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
                None,
            )
        }

        pub fn $rust_index(
            &self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::c_interface::CTmpRef::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                            *self.c_ptr_ref().$c.offset(index as isize),
                        )
                    },
                    None,
                ))
            } else {
                None
            }
        }

        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::c_interface::CTmpMut::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                            *self.c_ptr_mut().$c.offset(index as isize),
                        )
                    },
                    None,
                ))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_array_validated {
    ($rust:ident, $rust_mut:ident, $rust_index:ident, $rust_index_mut:ident, $parent_type:ident, $type:ident, $c_type:ident, $c:ident, $count_fn:ident, $validator:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CTmpPtrIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpPtrIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
                Some(self.$validator.clone()),
            )
        }

        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CTmpMutIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CTmpMutIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
                Some(self.$validator.clone()),
            )
        }

        pub fn $rust_index(
            &self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpRef<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::c_interface::CTmpRef::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                            *self.c_ptr_ref().$c.offset(index as isize),
                        )
                    },
                    Some(self.$validator.clone()),
                ))
            } else {
                None
            }
        }

        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::c_interface::CTmpMut<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::c_interface::CTmpMut::new(
                    self,
                    unsafe {
                        <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                            *self.c_ptr_mut().$c.offset(index as isize),
                        )
                    },
                    Some(self.$validator.clone()),
                ))
            } else {
                None
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
        // TODO: fill accessors
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
            vertex_attachment_mut,
            world_vertices_length,
            world_vertex_length_mut,
            worldVerticesLength,
            i32
        );
        c_accessor_for!(
            vertex_attachment,
            vertex_attachment_mut,
            id,
            id_mut,
            id,
            i32
        );
    };
}
