use core::{marker::PhantomData, ptr::NonNull, slice};
use std::{ffi::CString, path::Path};

use librw_sys as sys;

use crate::util::{nonneg, nonneg_product, owned_handle, slice_from_raw_mut};
use crate::{Engine, Error, Platform, Result, Rgba};

owned_handle!(Image, sys::rw_Image, sys::librw_image_destroy);
owned_handle!(Raster, sys::rw_Raster, sys::librw_raster_destroy);

impl<'engine> Image<'engine> {
    pub fn new(
        _engine: &'engine Engine,
        width: u32,
        height: u32,
        depth: ImageDepth,
    ) -> Result<Self> {
        let width = i32::try_from(width).map_err(|_| Error::LengthOverflow)?;
        let height = i32::try_from(height).map_err(|_| Error::LengthOverflow)?;
        let mut result =
            unsafe { Self::from_raw(sys::librw_image_create(width, height, depth as i32))? };
        unsafe { sys::librw_image_allocate(result.raw.as_ptr()) };
        if result.pixels().is_none() && width > 0 && height > 0 {
            return Err(Error::Null("allocating image pixels"));
        }
        Ok(result)
    }

    pub fn read(_engine: &'engine Engine, path: impl AsRef<Path>) -> Result<Self> {
        let path = CString::new(path.as_ref().to_str().ok_or(Error::NonUtf8Path)?)?;
        unsafe { Self::from_raw(sys::librw_image_read(path.as_ptr())) }
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::librw_image_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::librw_image_height(self.raw.as_ptr()) }
    }

    pub fn depth(&self) -> i32 {
        unsafe { sys::librw_image_depth(self.raw.as_ptr()) }
    }

    pub fn stride(&self) -> i32 {
        unsafe { sys::librw_image_stride(self.raw.as_ptr()) }
    }

    pub fn pixels(&mut self) -> Option<&mut [u8]> {
        let len = nonneg_product(self.stride(), self.height());
        let raw = unsafe { sys::librw_image_pixels(self.raw.as_ptr()) };
        slice_from_raw_mut(raw, len)
    }

    pub fn palette(&mut self) -> Option<&mut [Rgba]> {
        let count = match self.depth() {
            4 => 16,
            8 => 256,
            _ => return None,
        };
        let raw = unsafe { sys::librw_image_palette(self.raw.as_ptr()).cast::<Rgba>() };
        slice_from_raw_mut(raw, count)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageDepth {
    Indexed4 = 4,
    Indexed8 = 8,
    Rgb16 = 16,
    Rgb24 = 24,
    Rgba32 = 32,
}

impl<'engine> Raster<'engine> {
    pub fn new(
        _engine: &'engine Engine,
        width: u32,
        height: u32,
        depth: u32,
        format: i32,
        platform: Platform,
    ) -> Result<Self> {
        let width = i32::try_from(width).map_err(|_| Error::LengthOverflow)?;
        let height = i32::try_from(height).map_err(|_| Error::LengthOverflow)?;
        let depth = i32::try_from(depth).map_err(|_| Error::LengthOverflow)?;

        unsafe {
            Self::from_raw(sys::librw_raster_create(
                width,
                height,
                depth,
                format,
                platform.as_raw(),
            ))
        }
    }

    pub fn from_image(
        _engine: &'engine Engine,
        image: &mut Image<'engine>,
        platform: Platform,
    ) -> Result<Self> {
        unsafe {
            Self::from_raw(sys::librw_raster_from_image(
                image.raw.as_ptr(),
                platform.as_raw(),
            ))
        }
    }

    pub fn to_image(&mut self) -> Result<Image<'engine>> {
        unsafe { Image::from_raw(sys::librw_raster_to_image(self.raw.as_ptr())) }
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::librw_raster_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::librw_raster_height(self.raw.as_ptr()) }
    }

    pub fn depth(&self) -> i32 {
        unsafe { sys::librw_raster_depth(self.raw.as_ptr()) }
    }

    pub fn stride(&self) -> i32 {
        unsafe { sys::librw_raster_stride(self.raw.as_ptr()) }
    }

    pub fn num_levels(&mut self) -> usize {
        nonneg(unsafe { sys::librw_raster_num_levels(self.raw.as_ptr()) })
    }

    pub fn lock(&mut self, level: usize, mode: RasterLockMode) -> Result<RasterLock<'_, 'engine>> {
        let levels = self.num_levels();
        if level >= levels {
            return Err(Error::IndexOutOfBounds {
                index: level,
                len: levels,
            });
        }
        let level = i32::try_from(level).map_err(|_| Error::LengthOverflow)?;
        let raw = unsafe { sys::librw_raster_lock(self.raw.as_ptr(), level, mode as i32) };
        let len = nonneg_product(self.stride(), self.height());
        if raw.is_null() {
            Err(Error::Null("locking a raster"))
        } else {
            Ok(RasterLock {
                raster: self,
                raw,
                len,
                level,
            })
        }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_texture(&self) -> u32 {
        unsafe { sys::librw_gl3_raster_texture(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_framebuffer(&self) -> u32 {
        unsafe { sys::librw_gl3_raster_fbo(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_format(&self) -> i32 {
        unsafe { sys::librw_gl3_raster_format(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_bytes_per_pixel(&self) -> i32 {
        unsafe { sys::librw_gl3_raster_bytes_per_pixel(self.raw.as_ptr()) }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterLockMode {
    Write = sys::RW_RASTER_LOCK_WRITE,
    Read = sys::RW_RASTER_LOCK_READ,
    WriteNoFetch = sys::RW_RASTER_LOCK_WRITE | sys::RW_RASTER_LOCK_NO_FETCH,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RasterFlags(pub i32);

impl RasterFlags {
    pub const TEXTURE: Self = Self(sys::RW_RASTER_TEXTURE);
    pub const CAMERA_TEXTURE: Self = Self(sys::RW_RASTER_CAMERA_TEXTURE);
    pub const Z_BUFFER: Self = Self(sys::RW_RASTER_ZBUFFER);
    pub const C8888: Self = Self(sys::RW_RASTER_FORMAT_C8888);
    pub const MIPMAP: Self = Self(sys::RW_RASTER_FORMAT_MIPMAP);
    pub const AUTO_MIPMAP: Self = Self(sys::RW_RASTER_FORMAT_AUTO_MIPMAP);
    pub const DONT_ALLOCATE: Self = Self(sys::RW_RASTER_DONT_ALLOCATE);
}

impl core::ops::BitOr for RasterFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub struct RasterLock<'a, 'engine> {
    raster: &'a mut Raster<'engine>,
    raw: *mut u8,
    len: usize,
    level: i32,
}

// TOOD @joey sketchy sketchy sketchy
impl RasterLock<'_, '_> {
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.raw, self.len) }
    }

    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.raw, self.len) }
    }
}

impl Drop for RasterLock<'_, '_> {
    fn drop(&mut self) {
        unsafe { sys::librw_raster_unlock(self.raster.raw.as_ptr(), self.level) }
    }
}

pub struct RasterRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Raster>,
    pub(crate) _owner: PhantomData<&'a sys::rw_Raster>,
}

impl RasterRef<'_> {
    #[inline]
    pub fn as_raw(&self) -> *mut sys::rw_Raster {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::librw_raster_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::librw_raster_height(self.raw.as_ptr()) }
    }

    pub fn depth(&self) -> i32 {
        unsafe { sys::librw_raster_depth(self.raw.as_ptr()) }
    }

    pub fn stride(&self) -> i32 {
        unsafe { sys::librw_raster_stride(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_texture(&self) -> u32 {
        unsafe { sys::librw_gl3_raster_texture(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_framebuffer(&self) -> u32 {
        unsafe { sys::librw_gl3_raster_fbo(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_format(&self) -> i32 {
        unsafe { sys::librw_gl3_raster_format(self.raw.as_ptr()) }
    }

    #[cfg(feature = "gl3")]
    pub fn gl3_bytes_per_pixel(&self) -> i32 {
        unsafe { sys::librw_gl3_raster_bytes_per_pixel(self.raw.as_ptr()) }
    }
}
