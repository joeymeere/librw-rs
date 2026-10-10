use core::{marker::PhantomData, ptr, ptr::NonNull};

use librw_sys as sys;

use crate::util::owned_handle;
use crate::{BBox, Clump, Engine, Error, Result, Rgba};

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

    pub fn clear(&mut self, color: Rgba, mode: u32) {
        unsafe { sys::librw_camera_clear(self.raw.as_ptr(), &color, mode) }
    }

    pub fn begin_update(&mut self) {
        unsafe { sys::librw_camera_begin_update(self.raw.as_ptr()) }
    }

    pub fn end_update(&mut self) {
        unsafe { sys::librw_camera_end_update(self.raw.as_ptr()) }
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
