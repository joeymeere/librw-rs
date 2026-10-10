use core::ops::BitOr;
use core::ptr;

use librw_sys as sys;

use crate::{Error, Matrix, PrimitiveType, Result};

pub use sys::{rw_Im2DVertex as Im2DVertex, rw_Im3DVertex as Im3DVertex};

#[inline]
pub fn im2d_near_z() -> f32 {
    unsafe { sys::librw_im2d_near_z() }
}

#[inline]
pub fn im2d_far_z() -> f32 {
    unsafe { sys::librw_im2d_far_z() }
}

#[inline]
pub fn render_im2d(primitive: PrimitiveType, vertices: &mut [Im2DVertex]) -> Result<()> {
    let count = i32::try_from(vertices.len()).map_err(|_| Error::LengthOverflow)?;
    unsafe {
        sys::librw_im2d_render_primitive(primitive as i32, vertices.as_mut_ptr().cast(), count)
    };
    Ok(())
}

#[inline]
pub fn render_im2d_indexed(
    primitive: PrimitiveType,
    vertices: &mut [Im2DVertex],
    indices: &mut [u16],
) -> Result<()> {
    let vertex_count = i32::try_from(vertices.len()).map_err(|_| Error::LengthOverflow)?;
    let index_count = i32::try_from(indices.len()).map_err(|_| Error::LengthOverflow)?;
    unsafe {
        sys::librw_im2d_render_indexed_primitive(
            primitive as i32,
            vertices.as_mut_ptr().cast(),
            vertex_count,
            indices.as_mut_ptr().cast(),
            index_count,
        )
    };
    Ok(())
}

pub struct Im3D;

impl Im3D {
    pub fn transform(
        vertices: &mut [Im3DVertex],
        world: Option<&Matrix>,
        flags: Im3DFlags,
    ) -> Result<Self> {
        let count = i32::try_from(vertices.len()).map_err(|_| Error::LengthOverflow)?;
        unsafe {
            sys::librw_im3d_transform(
                vertices.as_mut_ptr().cast(),
                count,
                world.map_or(ptr::null(), |matrix| matrix),
                flags.0,
            )
        };
        Ok(Self)
    }

    pub fn render(&mut self, primitive: PrimitiveType) {
        unsafe { sys::librw_im3d_render_primitive(primitive as i32) }
    }

    pub fn render_indexed(&mut self, primitive: PrimitiveType, indices: &mut [u16]) -> Result<()> {
        let count = i32::try_from(indices.len()).map_err(|_| Error::LengthOverflow)?;
        unsafe {
            sys::librw_im3d_render_indexed_primitive(
                primitive as i32,
                indices.as_mut_ptr().cast(),
                count,
            )
        };
        Ok(())
    }
}

impl Drop for Im3D {
    fn drop(&mut self) {
        unsafe { sys::librw_im3d_end() }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Im3DFlags(pub u32);

impl Im3DFlags {
    pub const VERTEX_UV: Self = Self(sys::RW_IM3D_VERTEX_UV);
    pub const ALL_OPAQUE: Self = Self(sys::RW_IM3D_ALL_OPAQUE);
    pub const NO_CLIP: Self = Self(sys::RW_IM3D_NO_CLIP);
    pub const VERTEX_XYZ: Self = Self(sys::RW_IM3D_VERTEX_XYZ);
    pub const VERTEX_RGBA: Self = Self(sys::RW_IM3D_VERTEX_RGBA);
}

impl BitOr for Im3DFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
