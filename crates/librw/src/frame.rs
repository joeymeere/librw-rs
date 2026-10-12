use core::{marker::PhantomData, ptr::NonNull};

use librw_sys as sys;

use crate::util::owned_handle;
use crate::{Engine, Matrix, Result, Vec3};

#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Combine {
    Replace = sys::RW_COMBINE_REPLACE,
    PreConcat = sys::RW_COMBINE_PRECONCAT,
    #[default]
    PostConcat = sys::RW_COMBINE_POSTCONCAT,
}

owned_handle!(Frame, sys::rw_Frame, sys::librw_frame_destroy);

impl<'engine> Frame<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_frame_create()) }
    }

    pub fn matrix(&self) -> &Matrix {
        unsafe { &*sys::librw_frame_matrix(self.raw.as_ptr()) }
    }

    pub fn matrix_mut(&mut self) -> &mut Matrix {
        unsafe { &mut *sys::librw_frame_matrix(self.raw.as_ptr()) }
    }

    pub fn local_to_world(&mut self) -> &Matrix {
        unsafe { &*sys::librw_frame_ltm(self.raw.as_ptr()) }
    }

    pub fn translate(&mut self, value: Vec3, combine: Combine) {
        unsafe { sys::librw_frame_translate(self.raw.as_ptr(), &value, combine as i32) }
    }

    pub fn scale(&mut self, value: Vec3, combine: Combine) {
        unsafe { sys::librw_frame_scale(self.raw.as_ptr(), &value, combine as i32) }
    }

    pub fn rotate(&mut self, axis: Vec3, radians: f32, combine: Combine) {
        unsafe { sys::librw_frame_rotate(self.raw.as_ptr(), &axis, radians, combine as i32) }
    }

    /// marks the frame dirty; ltm and the objects on it update at the next sync
    pub fn update_objects(&mut self) {
        unsafe { sys::librw_frame_update_objects(self.raw.as_ptr()) }
    }
}

/// a frame an object is attached to
pub struct FrameRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Frame>,
    pub(crate) _owner: PhantomData<&'a sys::rw_Frame>,
}

impl FrameRef<'_> {
    #[inline]
    pub fn as_raw(&self) -> *mut sys::rw_Frame {
        self.raw.as_ptr()
    }

    pub fn local_to_world(&self) -> Matrix {
        unsafe { *sys::librw_frame_ltm(self.raw.as_ptr()) }
    }
}
