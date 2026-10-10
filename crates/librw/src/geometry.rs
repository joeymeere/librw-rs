use core::{marker::PhantomData, ptr, ptr::NonNull, slice};
use std::ops::BitOr;

use librw_sys as sys;

use crate::util::{nonneg, owned_handle, slice_from_raw, slice_from_raw_mut};
use crate::{
    Engine, Error, Result, Rgba, Sphere, SurfaceProperties, TexCoords, Texture, TextureRef,
    Triangle, Vec3,
};

owned_handle!(Material, sys::rw_Material, sys::librw_material_destroy);
owned_handle!(Geometry, sys::rw_Geometry, sys::librw_geometry_destroy);

impl<'engine> Material<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_material_create()) }
    }

    pub fn set_texture(&mut self, texture: Option<&Texture<'engine>>) {
        unsafe {
            sys::librw_material_set_texture(
                self.raw.as_ptr(),
                texture.map_or(ptr::null_mut(), |v| v.raw.as_ptr()),
            )
        }
    }

    pub fn texture(&self) -> Option<TextureRef<'_>> {
        NonNull::new(unsafe { sys::librw_material_texture(self.raw.as_ptr()) }).map(|raw| {
            TextureRef {
                raw,
                _owner: PhantomData,
            }
        })
    }

    pub fn color(&self) -> Rgba {
        unsafe { *sys::librw_material_color(self.raw.as_ptr()) }
    }

    pub fn set_color(&mut self, color: Rgba) {
        unsafe { *sys::librw_material_color(self.raw.as_ptr()) = color }
    }

    pub fn surface_properties(&self) -> SurfaceProperties {
        unsafe { *sys::librw_material_surface_properties(self.raw.as_ptr()) }
    }

    pub fn set_surface_properties(&mut self, value: SurfaceProperties) {
        unsafe { *sys::librw_material_surface_properties(self.raw.as_ptr()) = value }
    }
}

// TODO @joey prob just use proper bitflags
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GeometryFlags(pub u32);

impl GeometryFlags {
    pub const TRISTRIP: Self = Self(sys::RW_GEOMETRY_TRISTRIP);
    pub const POSITIONS: Self = Self(sys::RW_GEOMETRY_POSITIONS);
    pub const TEXTURED: Self = Self(sys::RW_GEOMETRY_TEXTURED);
    pub const PRELIT: Self = Self(sys::RW_GEOMETRY_PRELIT);
    pub const NORMALS: Self = Self(sys::RW_GEOMETRY_NORMALS);
    pub const LIGHT: Self = Self(sys::RW_GEOMETRY_LIGHT);
    pub const MODULATE: Self = Self(sys::RW_GEOMETRY_MODULATE);
    pub const TEXTURED2: Self = Self(sys::RW_GEOMETRY_TEXTURED2);
    pub const NATIVE: Self = Self(sys::RW_GEOMETRY_NATIVE);
}

impl BitOr for GeometryFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl<'engine> Geometry<'engine> {
    pub fn new(
        _engine: &'engine Engine,
        vertices: u32,
        triangles: u32,
        flags: GeometryFlags,
    ) -> Result<Self> {
        let vertices = i32::try_from(vertices).map_err(|_| Error::LengthOverflow)?;
        let triangles = i32::try_from(triangles).map_err(|_| Error::LengthOverflow)?;
        unsafe { Self::from_raw(sys::librw_geometry_create(vertices, triangles, flags.0)) }
    }

    pub fn vertex_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_vertices(self.raw.as_ptr()) })
    }

    pub fn triangle_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_triangles(self.raw.as_ptr()) })
    }

    pub fn morph_target_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_morph_targets(self.raw.as_ptr()) })
    }

    pub fn flags(&self) -> GeometryFlags {
        GeometryFlags(unsafe { sys::librw_geometry_flags(self.raw.as_ptr()) })
    }

    pub fn lock(&mut self, flags: u32) {
        unsafe { sys::librw_geometry_lock(self.raw.as_ptr(), flags as i32) }
    }

    pub fn unlock(&mut self) {
        unsafe { sys::librw_geometry_unlock(self.raw.as_ptr()) }
    }

    pub fn vertices(&mut self, target: usize) -> Result<&mut [Vec3]> {
        self.check_morph_target(target)?;
        let len = self.vertex_count();
        let raw = unsafe { sys::librw_geometry_vertices(self.raw.as_ptr(), target as i32) };
        slice_from_raw_mut(raw, len).ok_or(Error::Null("accessing geometry vertices"))
    }

    // TODO @joey garbage API design. rarely ever a good reason to return Result<Option<T>> unless the caller fucked your wife
    pub fn normals(&mut self, target: usize) -> Result<Option<&mut [Vec3]>> {
        self.check_morph_target(target)?;
        let len = self.vertex_count();
        let raw = unsafe { sys::librw_geometry_normals(self.raw.as_ptr(), target as i32) };
        Ok((!raw.is_null()).then(|| unsafe { slice::from_raw_parts_mut(raw, len) }))
    }

    pub fn triangles(&mut self) -> Option<&mut [Triangle]> {
        let len = self.triangle_count();
        let raw = unsafe { sys::librw_geometry_triangles(self.raw.as_ptr()) };
        slice_from_raw_mut(raw, len)
    }

    pub fn colors(&mut self) -> Option<&mut [Rgba]> {
        let len = self.vertex_count();
        let raw = unsafe { sys::librw_geometry_colors(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { slice::from_raw_parts_mut(raw, len) })
    }

    pub fn tex_coords(&mut self, set: usize) -> Option<&mut [TexCoords]> {
        let len = self.vertex_count();
        let raw = unsafe { sys::librw_geometry_tex_coords(self.raw.as_ptr(), set as i32) };
        (!raw.is_null()).then(|| unsafe { slice::from_raw_parts_mut(raw, len) })
    }

    pub fn add_material(&mut self, material: &Material<'engine>) -> Result<usize> {
        let index =
            unsafe { sys::librw_geometry_add_material(self.raw.as_ptr(), material.raw.as_ptr()) };
        if index < 0 {
            Err(Error::Null("adding a material"))
        } else {
            Ok(index as usize)
        }
    }

    pub fn material_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_material_count(self.raw.as_ptr()) })
    }

    pub fn calculate_bounding_sphere(&mut self) {
        unsafe { sys::librw_geometry_calculate_bounding_sphere(self.raw.as_ptr()) }
    }

    pub fn bounding_sphere(&mut self, target: usize) -> Result<&Sphere> {
        self.check_morph_target(target)?;
        let target = i32::try_from(target).map_err(|_| Error::LengthOverflow)?;
        let raw = unsafe { sys::librw_geometry_bounding_sphere(self.raw.as_ptr(), target) };
        NonNull::new(raw)
            .map(|raw| unsafe { raw.as_ref() })
            .ok_or(Error::Null("accessing geometry bounding sphere"))
    }

    #[inline]
    fn check_morph_target(&self, index: usize) -> Result<()> {
        let len = self.morph_target_count();
        if index < len {
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds { index, len })
        }
    }
}

pub struct GeometryRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Geometry>,
    pub(crate) _owner: PhantomData<&'a sys::rw_Geometry>,
}

impl GeometryRef<'_> {
    #[inline]
    pub fn as_raw(&self) -> *mut sys::rw_Geometry {
        self.raw.as_ptr()
    }

    pub fn vertex_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_vertices(self.raw.as_ptr()) })
    }

    pub fn triangle_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_triangles(self.raw.as_ptr()) })
    }

    pub fn morph_target_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_num_morph_targets(self.raw.as_ptr()) })
    }

    pub fn flags(&self) -> GeometryFlags {
        GeometryFlags(unsafe { sys::librw_geometry_flags(self.raw.as_ptr()) })
    }

    pub fn vertices(&self, target: usize) -> Result<&[Vec3]> {
        let targets = self.morph_target_count();
        if target >= targets {
            return Err(Error::IndexOutOfBounds {
                index: target,
                len: targets,
            });
        }
        let raw = unsafe { sys::librw_geometry_vertices(self.raw.as_ptr(), target as i32) };
        slice_from_raw(raw, self.vertex_count()).ok_or(Error::Null("accessing geometry vertices"))
    }

    pub fn normals(&self, target: usize) -> Result<Option<&[Vec3]>> {
        let targets = self.morph_target_count();
        if target >= targets {
            return Err(Error::IndexOutOfBounds {
                index: target,
                len: targets,
            });
        }
        let raw = unsafe { sys::librw_geometry_normals(self.raw.as_ptr(), target as i32) };
        Ok(slice_from_raw(raw, self.vertex_count()))
    }

    pub fn triangles(&self) -> Option<&[Triangle]> {
        let raw = unsafe { sys::librw_geometry_triangles(self.raw.as_ptr()) };
        slice_from_raw(raw, self.triangle_count())
    }

    pub fn colors(&self) -> Option<&[Rgba]> {
        let raw = unsafe { sys::librw_geometry_colors(self.raw.as_ptr()) };
        slice_from_raw(raw, self.vertex_count())
    }

    pub fn tex_coords(&self, set: usize) -> Option<&[TexCoords]> {
        let raw = unsafe { sys::librw_geometry_tex_coords(self.raw.as_ptr(), set as i32) };
        slice_from_raw(raw, self.vertex_count())
    }

    pub fn material_count(&self) -> usize {
        nonneg(unsafe { sys::librw_geometry_material_count(self.raw.as_ptr()) })
    }

    pub fn material(&self, index: usize) -> Option<MaterialRef<'_>> {
        NonNull::new(unsafe { sys::librw_geometry_material(self.raw.as_ptr(), index as i32) }).map(
            |raw| MaterialRef {
                raw,
                _owner: PhantomData,
            },
        )
    }
}

pub struct MaterialRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Material>,
    pub(crate) _owner: PhantomData<&'a sys::rw_Material>,
}

impl MaterialRef<'_> {
    #[inline]
    pub fn as_raw(&self) -> *mut sys::rw_Material {
        self.raw.as_ptr()
    }

    pub fn color(&self) -> Rgba {
        unsafe { *sys::librw_material_color(self.raw.as_ptr()) }
    }

    pub fn surface_properties(&self) -> SurfaceProperties {
        unsafe { *sys::librw_material_surface_properties(self.raw.as_ptr()) }
    }

    pub fn texture(&self) -> Option<TextureRef<'_>> {
        NonNull::new(unsafe { sys::librw_material_texture(self.raw.as_ptr()) }).map(|raw| {
            TextureRef {
                raw,
                _owner: PhantomData,
            }
        })
    }
}
