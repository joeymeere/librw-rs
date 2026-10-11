use core::ops::BitOr;
use core::{ffi::c_void, marker::PhantomData, ptr, ptr::NonNull};
use std::panic::{AssertUnwindSafe, catch_unwind};

use librw_sys as sys;

use crate::util::owned_handle;
use crate::{Atomic, BBox, Clump, Engine, Error, Frame, Raster, Result, Rgba, Vec2};

owned_handle!(Light, sys::rw_Light, sys::librw_light_destroy);
owned_handle!(Camera, sys::rw_Camera, sys::librw_camera_destroy);

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Directional = sys::RW_LIGHT_DIRECTIONAL,
    Ambient = sys::RW_LIGHT_AMBIENT,
    Point = sys::RW_LIGHT_POINT,
    Spot = sys::RW_LIGHT_SPOT,
    SoftSpot = sys::RW_LIGHT_SOFT_SPOT,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LightFlags(pub u32);

impl LightFlags {
    pub const ATOMICS: Self = Self(sys::RW_LIGHT_ATOMICS);
    pub const WORLD: Self = Self(sys::RW_LIGHT_WORLD);
}

impl BitOr for LightFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    Perspective = sys::RW_CAMERA_PERSPECTIVE,
    Parallel = sys::RW_CAMERA_PARALLEL,
}

impl<'engine> Light<'engine> {
    pub fn new(_engine: &'engine Engine, kind: LightType) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_light_create(kind as i32)) }
    }

    pub fn set_color(&mut self, red: f32, green: f32, blue: f32) {
        unsafe { sys::librw_light_set_color(self.raw.as_ptr(), red, green, blue) }
    }

    pub fn set_radius(&mut self, radius: f32) {
        unsafe { sys::librw_light_set_radius(self.raw.as_ptr(), radius) }
    }

    pub fn set_cone_angle(&mut self, radians: f32) {
        unsafe { sys::librw_light_set_angle(self.raw.as_ptr(), radians) }
    }

    pub fn set_flags(&mut self, flags: LightFlags) {
        unsafe { sys::librw_light_set_flags(self.raw.as_ptr(), flags.0) }
    }

    pub fn flags(&self) -> LightFlags {
        LightFlags(unsafe { sys::librw_light_flags(self.raw.as_ptr()) })
    }

    /// # Safety
    /// frame must outlive the light or be detached before its dropped
    pub unsafe fn set_frame(&mut self, frame: Option<&Frame<'engine>>) {
        unsafe {
            sys::librw_light_set_frame(
                self.raw.as_ptr(),
                frame.map_or(ptr::null_mut(), Frame::as_raw),
            )
        }
    }
}

impl<'engine> Camera<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_camera_create()) }
    }

    pub fn set_near_plane(&mut self, value: f32) {
        unsafe { sys::librw_camera_set_near_plane(self.raw.as_ptr(), value) }
    }

    pub fn set_far_plane(&mut self, value: f32) {
        unsafe { sys::librw_camera_set_far_plane(self.raw.as_ptr(), value) }
    }

    pub fn set_fov(&mut self, degrees: f32, ratio: f32) {
        unsafe { sys::librw_camera_set_fov(self.raw.as_ptr(), degrees, ratio) }
    }

    pub fn set_projection(&mut self, projection: Projection) {
        unsafe { sys::librw_camera_set_projection(self.raw.as_ptr(), projection as i32) }
    }

    pub fn set_view_window(&mut self, window: Vec2) {
        unsafe { sys::librw_camera_set_view_window(self.raw.as_ptr(), &window) }
    }

    pub fn set_view_offset(&mut self, offset: Vec2) {
        unsafe { sys::librw_camera_set_view_offset(self.raw.as_ptr(), &offset) }
    }

    pub fn clear(&mut self, color: Rgba, mode: u32) {
        unsafe { sys::librw_camera_clear(self.raw.as_ptr(), &color, mode) }
    }

    pub fn begin_update(&mut self) {
        unsafe { sys::librw_camera_begin_update(self.raw.as_ptr()) }
    }

    pub fn end_update(&mut self) {
        unsafe { sys::librw_camera_end_update(self.raw.as_ptr()) }
    }

    pub fn show_raster(&mut self, flags: u32) {
        unsafe { sys::librw_camera_show_raster(self.raw.as_ptr(), flags) }
    }

    /// # Safety
    /// frame must outlive the camera or be detached before its dropped
    pub unsafe fn set_frame(&mut self, frame: Option<&Frame<'engine>>) {
        unsafe {
            sys::librw_camera_set_frame(
                self.raw.as_ptr(),
                frame.map_or(ptr::null_mut(), Frame::as_raw),
            )
        }
    }

    /// # Safety
    /// raster must outlive the camera or be detached before its dropped
    pub unsafe fn set_frame_buffer(&mut self, raster: Option<&Raster<'engine>>) {
        unsafe {
            sys::librw_camera_set_frame_buffer(
                self.raw.as_ptr(),
                raster.map_or(ptr::null_mut(), Raster::as_raw),
            )
        }
    }

    /// # Safety
    /// raster must outlive the camera or be detached before its dropped
    pub unsafe fn set_z_buffer(&mut self, raster: Option<&Raster<'engine>>) {
        unsafe {
            sys::librw_camera_set_z_buffer(
                self.raw.as_ptr(),
                raster.map_or(ptr::null_mut(), Raster::as_raw),
            )
        }
    }
}

pub struct LightRef<'a> {
    raw: NonNull<sys::rw_Light>,
    _owner: PhantomData<&'a sys::rw_WorldLights>,
}

impl LightRef<'_> {
    pub fn as_raw(&self) -> *mut sys::rw_Light {
        self.raw.as_ptr()
    }

    pub fn flags(&self) -> LightFlags {
        LightFlags(unsafe { sys::librw_light_flags(self.raw.as_ptr()) })
    }

    pub fn kind(&self) -> Option<LightType> {
        match unsafe { sys::librw_light_type(self.raw.as_ptr()) } {
            sys::RW_LIGHT_DIRECTIONAL => Some(LightType::Directional),
            sys::RW_LIGHT_AMBIENT => Some(LightType::Ambient),
            sys::RW_LIGHT_POINT => Some(LightType::Point),
            sys::RW_LIGHT_SPOT => Some(LightType::Spot),
            sys::RW_LIGHT_SOFT_SPOT => Some(LightType::SoftSpot),
            _ => None,
        }
    }

    /// # Safety
    /// `link` must be a live link from a librw world's light list
    pub unsafe fn from_world_link(link: *mut core::ffi::c_void) -> Option<Self> {
        NonNull::new(unsafe { sys::librw_light_from_world_link(link) }).map(|raw| Self {
            raw,
            _owner: PhantomData,
        })
    }
}

pub struct WorldLights<'a> {
    raw: NonNull<sys::rw_WorldLights>,
    _world: PhantomData<&'a mut sys::rw_World>,
}

impl<'a> WorldLights<'a> {
    fn new(
        world: &'a mut World<'_>,
        atomic: Option<&Atomic<'_>>,
        directional_capacity: usize,
        local_capacity: usize,
    ) -> Result<Self> {
        let directional_capacity =
            i32::try_from(directional_capacity).map_err(|_| Error::LengthOverflow)?;

        let local_capacity = i32::try_from(local_capacity).map_err(|_| Error::LengthOverflow)?;

        let raw = NonNull::new(unsafe {
            sys::librw_world_lights_new(directional_capacity, local_capacity)
        })
        .ok_or(Error::Null("allocating world light storage"))?;

        unsafe {
            sys::librw_world_enumerate_lights(
                world.raw.as_ptr(),
                atomic.map_or(ptr::null_mut(), Atomic::as_raw),
                raw.as_ptr(),
            )
        };

        Ok(Self {
            raw,
            _world: PhantomData,
        })
    }

    pub fn ambient_count(&self) -> usize {
        unsafe { sys::librw_world_lights_num_ambient(self.raw.as_ptr()) as usize }
    }

    pub fn directional_count(&self) -> usize {
        unsafe { sys::librw_world_lights_num_directional(self.raw.as_ptr()) as usize }
    }

    pub fn local_count(&self) -> usize {
        unsafe { sys::librw_world_lights_num_local(self.raw.as_ptr()) as usize }
    }

    pub fn directional(&self, index: usize) -> Option<LightRef<'_>> {
        let index = i32::try_from(index).ok()?;
        NonNull::new(unsafe { sys::librw_world_lights_directional(self.raw.as_ptr(), index) }).map(
            |raw| LightRef {
                raw,
                _owner: PhantomData,
            },
        )
    }

    pub fn local(&self, index: usize) -> Option<LightRef<'_>> {
        let index = i32::try_from(index).ok()?;
        NonNull::new(unsafe { sys::librw_world_lights_local(self.raw.as_ptr(), index) }).map(
            |raw| LightRef {
                raw,
                _owner: PhantomData,
            },
        )
    }

    #[cfg(feature = "gl3")]
    pub fn apply_gl3(&mut self) -> i32 {
        unsafe { sys::librw_gl3_set_lights(self.raw.as_ptr()) }
    }
}

impl Drop for WorldLights<'_> {
    fn drop(&mut self) {
        unsafe { sys::librw_world_lights_delete(self.raw.as_ptr()) }
    }
}

/// World owns no child objects in librw
///
/// this wrapper keeps objects added alive, removing them before destroying the native world
pub struct World<'engine> {
    raw: NonNull<sys::rw_World>,
    clumps: Vec<Clump<'engine>>,
    lights: Vec<Light<'engine>>,
    cameras: Vec<Camera<'engine>>,
    _engine: PhantomData<&'engine Engine>,
}

impl<'engine> World<'engine> {
    pub fn new(_engine: &'engine Engine, bounds: Option<BBox>) -> Result<Self> {
        let raw = unsafe { sys::librw_world_create(bounds.as_ref().map_or(ptr::null(), |v| v)) };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(Error::Null("creating a world"))?,
            clumps: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            _engine: PhantomData,
        })
    }

    pub fn add_clump(&mut self, value: Clump<'engine>) {
        unsafe { sys::librw_world_add_clump(self.raw.as_ptr(), value.raw.as_ptr()) };
        self.clumps.push(value);
    }

    pub fn add_light(&mut self, value: Light<'engine>) {
        unsafe { sys::librw_world_add_light(self.raw.as_ptr(), value.raw.as_ptr()) };
        self.lights.push(value);
    }

    pub fn add_camera(&mut self, value: Camera<'engine>) {
        unsafe { sys::librw_world_add_camera(self.raw.as_ptr(), value.raw.as_ptr()) };
        self.cameras.push(value);
    }

    pub fn render(&mut self) {
        unsafe { sys::librw_world_render(self.raw.as_ptr()) }
    }

    /// visitor for global or local lights the global or local lights
    pub fn for_each_light<F: FnMut(LightRef<'_>)>(&self, local: bool, mut visit: F) {
        unsafe extern "C" fn trampoline<F: FnMut(LightRef<'_>)>(
            light: *mut sys::rw_Light,
            user_data: *mut c_void,
        ) {
            let Some(raw) = NonNull::new(light) else {
                return;
            };

            // SAFETY:
            // user_data is the `visit` borrowed for the duration of the call
            let visit = unsafe { &mut *user_data.cast::<F>() };
            let light = LightRef {
                raw,
                _owner: PhantomData,
            };
            if catch_unwind(AssertUnwindSafe(|| visit(light))).is_err() {
                std::process::abort();
            }
        }
        unsafe {
            sys::librw_world_for_each_light(
                self.raw.as_ptr(),
                i32::from(local),
                Some(trampoline::<F>),
                (&mut visit as *mut F).cast(),
            )
        }
    }

    pub fn lights(
        &mut self,
        atomic: Option<&Atomic<'_>>,
        directional_capacity: usize,
        local_capacity: usize,
    ) -> Result<WorldLights<'_>> {
        WorldLights::new(self, atomic, directional_capacity, local_capacity)
    }

    pub fn as_raw(&self) -> *mut sys::rw_World {
        self.raw.as_ptr()
    }
}

impl Drop for World<'_> {
    fn drop(&mut self) {
        unsafe {
            for value in &mut self.cameras {
                sys::librw_world_remove_camera(self.raw.as_ptr(), value.raw.as_ptr());
            }

            for value in &mut self.lights {
                sys::librw_world_remove_light(self.raw.as_ptr(), value.raw.as_ptr());
            }

            for value in &mut self.clumps {
                sys::librw_world_remove_clump(self.raw.as_ptr(), value.raw.as_ptr());
            }

            sys::librw_world_destroy(self.raw.as_ptr());
        }
    }
}
