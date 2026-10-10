use core::{
    ffi::c_void,
    marker::PhantomData,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};
#[cfg(feature = "gl3")]
use std::ffi::CStr;
use std::{path::Path, rc::Rc};

use librw_sys as sys;

use crate::plugins::{PluginRegistrar, reset_registration_state};
use crate::{Error, FileMode, FileStream, MemoryStream, Result};

static ENGINE_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Process-global librw engine state
///
/// note: this is NOT `Send` or `Sync`
pub struct Engine {
    _single_thread: PhantomData<Rc<()>>,
}

#[cfg(feature = "gl3")]
pub struct Gl3OpenParams<'a> {
    pub window: &'a mut *mut c_void,
    pub fullscreen: bool,
    pub width: i32,
    pub height: i32,
    pub title: &'a CStr,
}

impl Engine {
    /// starts librw's NULL backend with a null platform-open param
    pub fn new() -> Result<Self> {
        if unsafe { sys::librw_get_platform() } != Platform::NULL.0 {
            return Err(Error::PlatformParamsRequired);
        }

        // SAFETY:
        // null is permitted here!
        Self::new_with_plugins(|_| Ok(())).map(|(engine, ())| engine)
    }

    pub fn new_with_plugins<T>(
        register: impl FnOnce(&mut PluginRegistrar<'_>) -> Result<T>,
    ) -> Result<(Self, T)> {
        if unsafe { sys::librw_get_platform() } != Platform::NULL.0 {
            return Err(Error::PlatformParamsRequired);
        }
        unsafe { Self::start_with(register, || sys::librw_engine_open(ptr::null_mut())) }
    }

    /// starts librw using a platform-specific `rw::EngineOpenParams` ptr
    ///
    /// # Safety
    /// `params` needs to be null or point to the type expected by librw
    /// backend, and remain valid for the duration of `Engine::open`
    pub unsafe fn with_open_params(params: *mut c_void) -> Result<Self> {
        unsafe { Self::start_with(|_| Ok(()), || sys::librw_engine_open(params)) }
            .map(|(engine, ())| engine)
    }

    #[cfg(feature = "gl3")]
    pub fn new_gl3(params: Gl3OpenParams<'_>) -> Result<Self> {
        Self::new_gl3_with_plugins(params, |_| Ok(())).map(|(engine, ())| engine)
    }

    #[cfg(feature = "gl3")]
    pub fn new_gl3_with_plugins<T>(
        params: Gl3OpenParams<'_>,
        register: impl FnOnce(&mut PluginRegistrar<'_>) -> Result<T>,
    ) -> Result<(Self, T)> {
        unsafe {
            Self::start_with(register, || {
                sys::librw_engine_open_gl3(
                    params.window,
                    i32::from(params.fullscreen),
                    params.width,
                    params.height,
                    params.title.as_ptr(),
                )
            })
        }
    }

    unsafe fn start_with<T>(
        register: impl FnOnce(&mut PluginRegistrar<'_>) -> Result<T>,
        open: impl FnOnce() -> i32,
    ) -> Result<(Self, T)> {
        if ENGINE_ACTIVE.swap(true, Ordering::AcqRel) {
            return Err(Error::EngineAlreadyActive);
        }

        if unsafe { sys::librw_engine_init() } == 0 {
            ENGINE_ACTIVE.store(false, Ordering::Release);
            return Err(Error::Engine("initialization"));
        }

        let mut plugins = PluginRegistrar {
            _phase: PhantomData,
        };
        let registered = match register(&mut plugins) {
            Ok(value) => value,
            Err(error) => {
                unsafe { sys::librw_engine_term() };
                reset_registration_state();
                ENGINE_ACTIVE.store(false, Ordering::Release);
                return Err(error);
            }
        };

        if open() == 0 {
            unsafe { sys::librw_engine_term() };
            reset_registration_state();
            ENGINE_ACTIVE.store(false, Ordering::Release);
            return Err(Error::Engine("open"));
        }

        if unsafe { sys::librw_engine_start() } == 0 {
            unsafe {
                sys::librw_engine_close();
                sys::librw_engine_term();
            }
            reset_registration_state();
            ENGINE_ACTIVE.store(false, Ordering::Release);
            return Err(Error::Engine("start"));
        }
        Ok((
            Self {
                _single_thread: PhantomData,
            },
            registered,
        ))
    }

    pub fn platform(&self) -> i32 {
        unsafe { sys::librw_get_platform() }
    }

    pub fn set_platform(&mut self, platform: Platform) {
        unsafe { sys::librw_set_platform(platform.0) }
    }

    pub fn version(&self) -> i32 {
        unsafe { sys::librw_get_version() }
    }

    pub fn set_version(&mut self, version: i32) {
        unsafe { sys::librw_set_version(version) }
    }

    pub fn memory_reader<'engine, 'data>(
        &'engine self,
        bytes: &'data mut [u8],
    ) -> Result<MemoryStream<'engine, 'data>> {
        let len = bytes.len();
        MemoryStream::new(self, bytes, len)
    }

    pub fn memory_writer<'engine, 'data>(
        &'engine self,
        bytes: &'data mut [u8],
    ) -> Result<MemoryStream<'engine, 'data>> {
        MemoryStream::new(self, bytes, 0)
    }

    pub fn open_file(&self, path: impl AsRef<Path>, mode: FileMode) -> Result<FileStream<'_>> {
        FileStream::open(self, path.as_ref(), mode)
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            sys::librw_engine_stop();
            sys::librw_engine_close();
            sys::librw_engine_term();
        }
        reset_registration_state();
        ENGINE_ACTIVE.store(false, Ordering::Release);
    }
}

// TODO @joey can etiher make this a repr(i32) enum or just repr(Transparent)
// double check this won't fuck anything
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Platform(i32);

impl Platform {
    pub const NULL: Self = Self(0);
    pub const GL: Self = Self(2);
    pub const PS2: Self = Self(4);
    pub const XBOX: Self = Self(5);
    pub const GAMECUBE: Self = Self(6);
    pub const D3D8: Self = Self(8);
    pub const D3D9: Self = Self(9);
    pub const WDGL: Self = Self(11);
    pub const GL3: Self = Self(12);

    // on the above, we prob shouldn't need this & just use Into<i32> instead
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
