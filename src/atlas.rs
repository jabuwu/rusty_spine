use std::ffi::{CStr, CString};
use std::{path::Path, ptr::null_mut};

use crate::c::{
    spAtlasFilter, spAtlasFormat, spAtlasRegion, spAtlasWrap, spAtlas_createFromFile,
    spTextureRegion,
};
use crate::c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr};
use crate::texture_region::TextureRegion;
use crate::{
    c::{c_int, spAtlas, spAtlasPage, spAtlas_create, spAtlas_dispose},
    error::SpineError,
};

use atlas::*;

#[cfg(feature = "mint")]
use mint::Vector2;

/// An atlas loaded from Spine's `.atlas` file format.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Atlas)
#[derive(Debug)]
pub struct Atlas {
    c_atlas: SyncPtr<spAtlas>,
    owns_memory: bool,
}

impl NewFromPtr<spAtlas> for Atlas {
    unsafe fn new_from_ptr(c_atlas: *mut spAtlas) -> Atlas {
        Atlas {
            c_atlas: SyncPtr(c_atlas),
            owns_memory: false,
        }
    }
}

impl Atlas {
    /// Create an Atlas from an in-memory vector.
    ///
    /// ```
    /// use rusty_spine::Atlas;
    /// fn load_atlas() -> Atlas {
    ///     let atlas_file = std::fs::read("skeleton.atlas").unwrap();
    ///     Atlas::new(&atlas_file, "").unwrap()
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the [`SpineError::NulError`] if `dir` or `data` contain an internal 0 byte. Returns
    /// [`SpineError::PathNotUtf8`] if the specified `dir` is not utf-8. This function does not
    /// error if the atlas file is invalid or malformed. The file is parsed line-by-line and invalid
    /// lines are simply ignored.
    pub fn new<P: AsRef<Path>>(data: &[u8], dir: P) -> Result<Atlas, SpineError> {
        let c_data = CString::new(data)?;
        let Some(dir_path) = dir.as_ref().to_str() else {
            return Err(SpineError::PathNotUtf8);
        };
        let c_dir = CString::new(dir_path)?;
        let c_atlas = unsafe {
            spAtlas_create(
                c_data.as_ptr(),
                data.len() as c_int,
                c_dir.as_ptr(),
                null_mut(),
            )
        };
        Ok(Self {
            c_atlas: SyncPtr(c_atlas),
            owns_memory: true,
        })
    }

    /// Create an Atlas from a file.
    /// ```
    /// use rusty_spine::Atlas;
    /// fn load_atlas() -> Result<Atlas, rusty_spine::SpineError>{
    ///     Ok(Atlas::new_from_file("skeleton.json")?)
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::FailedToReadFile`] if the file could not be read, returns
    /// [`SpineError::NulError`] if `path` contains an internal 0 byte or if the loaded atlas
    /// contains a 0 byte. Returns  [`SpineError::PathNotUtf8`] if the specified `path` is not
    /// utf-8.
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Atlas, SpineError> {
        let Some(path_str) = path.as_ref().to_str() else {
            return Err(SpineError::PathNotUtf8);
        };
        let c_path = CString::new(path_str)?;
        let c_atlas = unsafe { spAtlas_createFromFile(c_path.as_ptr(), null_mut()) };
        if !c_atlas.is_null() {
            Ok(Self {
                c_atlas: SyncPtr(c_atlas),
                owns_memory: true,
            })
        } else {
            Err(SpineError::FailedToReadFile {
                file: path_str.to_owned(),
            })
        }
    }

    #[must_use]
    pub fn pages(&self) -> AtlasPageIterator {
        AtlasPageIterator {
            _atlas: self,
            page: unsafe { self.c_ptr_mut().pages },
        }
    }

    #[must_use]
    pub fn pages_mut(&mut self) -> AtlasPageMutIterator {
        let page = unsafe { self.c_ptr_mut().pages };
        AtlasPageMutIterator { _atlas: self, page }
    }

    #[must_use]
    pub fn find_page(&self, name: &str) -> Option<CTmpRef<Self, AtlasPage>> {
        self.pages().find(|page| page.name() == name)
    }

    #[must_use]
    pub fn find_page_mut(&mut self, name: &str) -> Option<CTmpMut<Self, AtlasPage>> {
        self.pages_mut().find(|page| page.name() == name)
    }

    #[must_use]
    pub fn regions(&self) -> AtlasRegionIterator {
        AtlasRegionIterator {
            _atlas: self,
            region: unsafe { self.c_ptr_mut().regions },
        }
    }

    #[must_use]
    pub fn regions_mut(&mut self) -> AtlasRegionMutIterator {
        let region = unsafe { self.c_ptr_mut().regions };
        AtlasRegionMutIterator {
            _atlas: self,
            region,
        }
    }

    #[must_use]
    pub fn find_region(&self, name: &str) -> Option<CTmpRef<Self, AtlasRegion>> {
        self.regions().find(|region| region.name() == name)
    }

    #[must_use]
    pub fn find_region_mut(&mut self, name: &str) -> Option<CTmpMut<Self, AtlasRegion>> {
        self.regions_mut().find(|region| region.name() == name)
    }

    c_accessor_renderer_object!();
    c_ptr!(c_atlas, spAtlas);
}

impl Drop for Atlas {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spAtlas_dispose(self.c_atlas.0);
            }
        }
    }
}

pub mod atlas {
    //! Types related to atlases.
    //!
    //! To load an atlas file, see [`Atlas`].

    use crate::c_interface::from_c_str;

    use super::*;

    #[derive(Debug)]
    pub struct AtlasPage {
        c_atlas_page: SyncPtr<spAtlasPage>,
    }

    impl NewFromPtr<spAtlasPage> for AtlasPage {
        unsafe fn new_from_ptr(c_atlas_page: *mut spAtlasPage) -> Self {
            Self {
                c_atlas_page: SyncPtr(c_atlas_page),
            }
        }
    }

    impl AtlasPage {
        c_accessor_tmp_ptr!(atlas, atlas_mut, atlas, Atlas, spAtlas);
        c_accessor_string!(name, name);
        c_accessor_enum!(format, format, AtlasFormat);
        c_accessor_enum!(min_filter, minFilter, AtlasFilter);
        c_accessor_enum!(mag_filter, magFilter, AtlasFilter);
        c_accessor_enum!(u_wrap, uWrap, AtlasWrap);
        c_accessor_enum!(v_wrap, vWrap, AtlasWrap);
        c_accessor!(width, width, i32);
        c_accessor!(height, height, i32);
        c_accessor_bool!(pma, pma);
        c_accessor_renderer_object!();
        c_ptr!(c_atlas_page, spAtlasPage);
    }

    /// Functions available if using the `mint` feature.
    #[cfg(feature = "mint")]
    impl AtlasPage {
        #[must_use]
        pub fn size(&self) -> Vector2<i32> {
            Vector2 {
                x: self.width(),
                y: self.height(),
            }
        }
    }

    pub struct AtlasPageIterator<'a> {
        pub(crate) _atlas: &'a Atlas,
        pub(crate) page: *mut spAtlasPage,
    }

    impl<'a> Iterator for AtlasPageIterator<'a> {
        type Item = CTmpRef<'a, Atlas, AtlasPage>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.page.is_null() {
                let page = unsafe { AtlasPage::new_from_ptr(self.page) };
                self.page = unsafe { (*self.page).next };
                Some(CTmpRef::new(self._atlas, page))
            } else {
                None
            }
        }
    }

    pub struct AtlasPageMutIterator<'a> {
        pub(crate) _atlas: &'a mut Atlas,
        pub(crate) page: *mut spAtlasPage,
    }

    impl<'a> Iterator for AtlasPageMutIterator<'a> {
        type Item = CTmpMut<'a, Atlas, AtlasPage>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.page.is_null() {
                let page = unsafe { AtlasPage::new_from_ptr(self.page) };
                self.page = unsafe { (*self.page).next };
                Some(CTmpMut::new(
                    unsafe { &mut *(self._atlas as *mut Atlas) },
                    page,
                ))
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AtlasFormat {
        UnknownFormat = 0,
        Alpha = 1,
        Intensity = 2,
        LuminanceAlpha = 3,
        RGB565 = 4,
        RGBA4444 = 5,
        RGB888 = 6,
        RGBA8888 = 7,
    }

    impl From<spAtlasFormat> for AtlasFormat {
        fn from(format: spAtlasFormat) -> Self {
            match format {
                1 => Self::Alpha,
                2 => Self::Intensity,
                3 => Self::LuminanceAlpha,
                4 => Self::RGB565,
                5 => Self::RGBA4444,
                6 => Self::RGB888,
                7 => Self::RGBA8888,
                _ => Self::UnknownFormat,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AtlasFilter {
        UnknownFilter = 0,
        Nearest = 1,
        Linear = 2,
        Mipmap = 3,
        MipmapNearestNearest = 4,
        MipmapLinearNearest = 5,
        MipmapNearestLinear = 6,
        MipmapLinearLinear = 7,
    }

    impl From<spAtlasFilter> for AtlasFilter {
        fn from(filter: spAtlasFilter) -> Self {
            match filter {
                1 => Self::Nearest,
                2 => Self::Linear,
                3 => Self::Mipmap,
                4 => Self::MipmapNearestNearest,
                5 => Self::MipmapLinearNearest,
                6 => Self::MipmapNearestLinear,
                7 => Self::MipmapLinearLinear,
                _ => Self::UnknownFilter,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AtlasWrap {
        MirroredRepeat = 0,
        ClampToEdge = 1,
        Repeat = 2,
        Unknown = 99,
    }

    impl From<spAtlasWrap> for AtlasWrap {
        fn from(wrap: spAtlasWrap) -> Self {
            match wrap {
                0 => Self::MirroredRepeat,
                1 => Self::ClampToEdge,
                2 => Self::Repeat,
                _ => Self::Unknown,
            }
        }
    }

    #[derive(Debug)]
    pub struct AtlasRegion {
        c_atlas_region: SyncPtr<spAtlasRegion>,
    }

    impl NewFromPtr<spAtlasRegion> for AtlasRegion {
        unsafe fn new_from_ptr(c_atlas_region: *mut spAtlasRegion) -> Self {
            Self {
                c_atlas_region: SyncPtr(c_atlas_region),
            }
        }
    }

    impl AtlasRegion {
        c_accessor_super!(
            texture_region,
            texture_region_mut,
            TextureRegion,
            spTextureRegion
        );
        c_accessor_string!(name, name);
        c_accessor!(x, x, i32);
        c_accessor!(y, y, i32);
        c_accessor!(index, index, usize);
        c_accessor_fixed_slice_optional!(splits, splits, &[c_int; 4], 4);
        c_accessor_fixed_slice_optional!(pads, pads, &[c_int; 4], 4);
        c_accessor_tmp_ptr!(page, page_mut, page, AtlasPage, spAtlasPage);

        #[must_use]
        pub fn key_values(&self) -> Vec<KeyValue> {
            let mut vec = vec![];
            unsafe {
                let array = &mut *self.c_ptr_ref().keyValues;
                for i in 0..array.size {
                    let item = &*array.items.offset(i as isize);
                    let name = String::from(from_c_str(CStr::from_ptr(item.name)));
                    let values = item.values;
                    vec.push(KeyValue { name, values });
                }
            }
            vec
        }

        c_ptr!(c_atlas_region, spAtlasRegion);
    }

    #[derive(Debug)]
    pub struct KeyValue {
        pub name: String,
        pub values: [f32; 5],
    }

    pub struct AtlasRegionIterator<'a> {
        pub(crate) _atlas: &'a Atlas,
        pub(crate) region: *mut spAtlasRegion,
    }

    impl<'a> Iterator for AtlasRegionIterator<'a> {
        type Item = CTmpRef<'a, Atlas, AtlasRegion>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.region.is_null() {
                let page = unsafe { AtlasRegion::new_from_ptr(self.region) };
                self.region = unsafe { (*self.region).next };
                Some(CTmpRef::new(self._atlas, page))
            } else {
                None
            }
        }
    }

    /// Functions available if using the `mint` feature.
    #[cfg(feature = "mint")]
    impl AtlasRegion {
        #[must_use]
        pub fn position(&self) -> Vector2<i32> {
            Vector2 {
                x: self.x(),
                y: self.y(),
            }
        }
    }

    pub struct AtlasRegionMutIterator<'a> {
        pub(crate) _atlas: &'a mut Atlas,
        pub(crate) region: *mut spAtlasRegion,
    }

    impl<'a> Iterator for AtlasRegionMutIterator<'a> {
        type Item = CTmpMut<'a, Atlas, AtlasRegion>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.region.is_null() {
                let page = unsafe { AtlasRegion::new_from_ptr(self.region) };
                self.region = unsafe { (*self.region).next };
                Some(CTmpMut::new(
                    unsafe { &mut *(self._atlas as *mut Atlas) },
                    page,
                ))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::TestAsset;

    use super::Atlas;

    #[test]
    fn new_from_file() {
        for test_example_asset in TestAsset::all() {
            let atlas = Atlas::new_from_file(test_example_asset.atlas_file);
            assert!(atlas.is_ok());
        }

        let atlas = Atlas::new_from_file(format!("missing/{}", TestAsset::spineboy().atlas_file));
        assert!(atlas.is_err());
    }
}
