use std::ffi::{CStr, CString};
use std::{path::Path, ptr::null_mut};

use crate::c::{spAtlasFilter, spAtlasFormat, spAtlasRegion, spAtlasWrap};
use crate::sync_ptr::SyncPtr;
use crate::texture_region::TextureRegion;
use crate::{
    c::{c_int, spAtlas, spAtlasPage, spAtlas_create, spAtlas_dispose},
    error::Error,
};

#[derive(Debug)]
pub struct Atlas {
    c_atlas: SyncPtr<spAtlas>,
    pages: Vec<AtlasPage>,
    regions: Vec<AtlasRegion>,
}

impl Atlas {
    pub fn new<P: AsRef<Path>>(data: &[u8], dir: P) -> Result<Atlas, Error> {
        let c_data = CString::new(data)?;
        let c_dir = CString::new(dir.as_ref().to_str().unwrap())?;
        let c_atlas = unsafe {
            spAtlas_create(
                c_data.as_ptr(),
                data.len() as c_int,
                c_dir.as_ptr(),
                null_mut(),
            )
        };
        Ok(Self::new_from_ptr(c_atlas))
    }

    pub(crate) fn new_from_ptr(c_atlas: *mut spAtlas) -> Atlas {
        let mut pages = vec![];
        let mut c_atlas_page = unsafe { (*c_atlas).pages };
        while !c_atlas_page.is_null() {
            pages.push(AtlasPage::new_from_ptr(c_atlas_page));
            c_atlas_page = unsafe { (*c_atlas_page).next };
        }
        let mut regions = vec![];
        let mut c_atlas_region = unsafe { (*c_atlas).regions };
        while !c_atlas_region.is_null() {
            regions.push(AtlasRegion::new_from_ptr(c_atlas_region));
            c_atlas_region = unsafe { (*c_atlas_region).next };
        }
        Atlas {
            c_atlas: SyncPtr(c_atlas),
            pages,
            regions,
        }
    }

    pub fn pages(&self) -> &Vec<AtlasPage> {
        &self.pages
    }

    pub fn regions(&self) -> &Vec<AtlasRegion> {
        &self.regions
    }

    c_ptr!(c_atlas, spAtlas);
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe {
            spAtlas_dispose(self.c_atlas.0);
        }
    }
}

#[derive(Debug)]
pub struct AtlasPage {
    c_atlas_page: SyncPtr<spAtlasPage>,
}

impl AtlasPage {
    pub(crate) fn new_from_ptr(c_atlas_page: *mut spAtlasPage) -> Self {
        Self {
            c_atlas_page: SyncPtr(c_atlas_page),
        }
    }

    c_ptr!(c_atlas_page, spAtlasPage);
    c_accessor_string!(name, name);
    c_accessor_enum!(format, format, AtlasFormat);
    c_accessor_enum!(min_filter, minFilter, AtlasFilter);
    c_accessor_enum!(mag_filter, magFilter, AtlasFilter);
    c_accessor_enum!(u_wrap, uWrap, AtlasWrap);
    c_accessor_enum!(v_wrap, vWrap, AtlasWrap);
    c_accessor!(width, width, i32);
    c_accessor!(height, height, i32);
    c_accessor_bool!(pma, pma);
    c_accessor_void_ptr!(renderer_object, rendererObject);
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
}

impl From<spAtlasWrap> for AtlasWrap {
    fn from(wrap: spAtlasWrap) -> Self {
        match wrap {
            1 => Self::ClampToEdge,
            2 => Self::Repeat,
            _ => Self::MirroredRepeat,
        }
    }
}

#[derive(Debug)]
pub struct AtlasRegion {
    c_atlas_region: SyncPtr<spAtlasRegion>,
    texture_region: TextureRegion,
    page: AtlasPage,
}

impl AtlasRegion {
    pub(crate) fn new_from_ptr(c_atlas_region: *mut spAtlasRegion) -> Self {
        Self {
            c_atlas_region: SyncPtr(c_atlas_region),
            texture_region: unsafe { TextureRegion::new_from_ptr(&mut (*c_atlas_region).super_0) },
            page: unsafe { AtlasPage::new_from_ptr((*c_atlas_region).page) },
        }
    }

    pub fn texture_region(&self) -> &TextureRegion {
        &self.texture_region
    }

    pub fn page(&self) -> &AtlasPage {
        &self.page
    }

    pub fn splits(&self) -> *mut c_int {
        self.c_ptr_ref().splits
    }

    pub fn pads(&self) -> *mut c_int {
        self.c_ptr_ref().pads
    }

    pub fn key_values(&self) -> Vec<KeyValue> {
        let mut vec = vec![];
        unsafe {
            let array = &mut *self.c_ptr_ref().keyValues;
            for i in 0..array.size {
                let item = &*array.items.offset(i as isize);
                let name = String::from(CStr::from_ptr(item.name).to_str().unwrap());
                let values = item.values.clone();
                vec.push(KeyValue { name, values });
            }
        }
        vec
    }

    c_ptr!(c_atlas_region, spAtlasRegion);
    c_accessor_string!(name, name);
    c_accessor!(x, x, i32);
    c_accessor!(y, y, i32);
    c_accessor!(index, index, i32);
}

#[derive(Debug)]
pub struct KeyValue {
    pub name: String,
    pub values: [f32; 5],
}
