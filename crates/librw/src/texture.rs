use core::{marker::PhantomData, ptr, ptr::NonNull};
use std::borrow::Cow;
use std::ffi::{CStr, CString};

use librw_sys as sys;

use crate::util::{cstr, nonneg, owned_handle};
use crate::{Engine, Raster, RasterRef, Result, Stream};

owned_handle!(Texture, sys::rw_Texture, sys::librw_texture_destroy);
owned_handle!(
    TextureDictionary,
    sys::rw_TexDictionary,
    sys::librw_tex_dictionary_destroy
);

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFilter {
    Nearest = sys::RW_TEXTURE_FILTER_NEAREST,
    Linear = sys::RW_TEXTURE_FILTER_LINEAR,
    MipNearest = sys::RW_TEXTURE_FILTER_MIP_NEAREST,
    MipLinear = sys::RW_TEXTURE_FILTER_MIP_LINEAR,
    LinearMipNearest = sys::RW_TEXTURE_FILTER_LINEAR_MIP_NEAREST,
    LinearMipLinear = sys::RW_TEXTURE_FILTER_LINEAR_MIP_LINEAR,
}

impl<'engine> Texture<'engine> {
    pub fn new(_engine: &'engine Engine, raster: Raster<'engine>) -> Result<Self> {
        let raw_raster = raster.into_raw();
        match unsafe { Self::from_raw(sys::librw_texture_create(raw_raster)) } {
            Ok(value) => Ok(value),
            Err(error) => {
                unsafe { sys::librw_raster_destroy(raw_raster) };
                Err(error)
            }
        }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        cstr(unsafe { sys::librw_texture_name(self.raw.as_ptr()) })
    }

    #[inline]
    pub fn mask(&self) -> &CStr {
        cstr(unsafe { sys::librw_texture_mask(self.raw.as_ptr()) })
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        self.name().to_string_lossy()
    }

    pub fn mask_lossy(&self) -> Cow<'_, str> {
        self.mask().to_string_lossy()
    }

    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        let value = CString::new(name)?;
        unsafe { sys::librw_texture_set_name(self.raw.as_ptr(), value.as_ptr()) };
        Ok(())
    }

    #[inline]
    pub fn set_mask(&mut self, mask: &str) -> Result<()> {
        let value = CString::new(mask)?;
        unsafe { sys::librw_texture_set_mask(self.raw.as_ptr(), value.as_ptr()) };
        Ok(())
    }

    #[inline]
    pub fn filter(&self) -> i32 {
        unsafe { sys::librw_texture_filter(self.raw.as_ptr()) }
    }

    #[inline]
    pub fn set_filter(&mut self, filter: TextureFilter) {
        unsafe { sys::librw_texture_set_filter(self.raw.as_ptr(), filter as i32) }
    }
}

pub struct TextureRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Texture>,
    pub(crate) _owner: PhantomData<&'a sys::rw_Texture>,
}
impl TextureRef<'_> {
    #[inline]
    pub fn name(&self) -> &CStr {
        cstr(unsafe { sys::librw_texture_name(self.raw.as_ptr()) })
    }

    #[inline]
    pub fn mask(&self) -> &CStr {
        cstr(unsafe { sys::librw_texture_mask(self.raw.as_ptr()) })
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        self.name().to_string_lossy()
    }

    pub fn mask_lossy(&self) -> Cow<'_, str> {
        self.mask().to_string_lossy()
    }

    #[inline]
    pub fn as_raw(&self) -> *mut sys::rw_Texture {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn raster(&self) -> Option<RasterRef<'_>> {
        NonNull::new(unsafe { sys::librw_texture_raster(self.raw.as_ptr()) }).map(|raw| RasterRef {
            raw,
            _owner: PhantomData,
        })
    }
}

impl<'engine> TextureDictionary<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_tex_dictionary_create()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn set_current(&mut self) {
        unsafe { sys::librw_tex_dictionary_set_current(self.raw.as_ptr()) }
    }

    #[inline]
    pub fn clear_current() {
        unsafe { sys::librw_tex_dictionary_set_current(ptr::null_mut()) }
    }

    pub fn read(_engine: &'engine Engine, stream: &mut impl Stream) -> Result<Self> {
        stream.find_chunk(sys::RW_ID_TEX_DICTIONARY)?;
        unsafe { Self::from_raw(sys::librw_tex_dictionary_read(stream.as_raw_stream())) }
    }

    pub fn read_payload(_engine: &'engine Engine, stream: &mut impl Stream) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_tex_dictionary_read(stream.as_raw_stream())) }
    }

    pub fn write(&mut self, stream: &mut impl Stream) {
        unsafe { sys::librw_tex_dictionary_write(self.raw.as_ptr(), stream.as_raw_stream()) }
    }

    pub fn add(&mut self, texture: Texture<'engine>) {
        unsafe { sys::librw_tex_dictionary_add(self.raw.as_ptr(), texture.into_raw()) }
    }

    pub fn len(&self) -> usize {
        nonneg(unsafe { sys::librw_tex_dictionary_count(self.raw.as_ptr()) })
    }

    pub fn texture(&mut self, index: usize) -> Option<TextureRef<'_>> {
        NonNull::new(unsafe { sys::librw_tex_dictionary_texture(self.raw.as_ptr(), index as i32) })
            .map(|raw| TextureRef {
                raw,
                _owner: PhantomData,
            })
    }

    pub fn find(&mut self, name: &str) -> Result<Option<TextureRef<'_>>> {
        let name = CString::new(name)?;
        Ok(NonNull::new(unsafe {
            sys::librw_tex_dictionary_find(self.raw.as_ptr(), name.as_ptr())
        })
        .map(|raw| TextureRef {
            raw,
            _owner: PhantomData,
        }))
    }
}
