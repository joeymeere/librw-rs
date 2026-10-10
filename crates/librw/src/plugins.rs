use core::sync::atomic::{AtomicBool, Ordering};
use core::{ffi::c_void, marker::PhantomData, ptr::NonNull, slice};

use librw_sys as sys;

use crate::{Atomic, Engine, Error, Geometry, Material, MaterialRef, Matrix, Result, TextureRef};

static SKIN_REGISTERED: AtomicBool = AtomicBool::new(false);
static MATFX_REGISTERED: AtomicBool = AtomicBool::new(false);
static NATIVE_DATA_REGISTERED: AtomicBool = AtomicBool::new(false);
static HANIM_REGISTERED: AtomicBool = AtomicBool::new(false);

pub const ID_SKIN: u32 = sys::RW_ID_SKIN;
pub const MEMDUR_EVENT: u32 = sys::RW_MEMDUR_EVENT;

/// Access librw's plugin registration phase
///
/// Values of this type are only supplied by `Engine::new_with_plugins` and
/// `Engine::new_gl3_with_plugins`, after `Engine::init` and before `Engine::open`
pub struct PluginRegistrar<'a> {
    pub(crate) _phase: PhantomData<&'a mut ()>,
}

impl PluginRegistrar<'_> {
    pub fn register_native_data(&mut self) {
        if !NATIVE_DATA_REGISTERED.swap(true, Ordering::AcqRel) {
            unsafe { sys::librw_register_native_data_plugin() }
        }
    }

    pub fn register_skin(&mut self) {
        if !SKIN_REGISTERED.swap(true, Ordering::AcqRel) {
            unsafe { sys::librw_register_skin_plugin() }
        }
    }

    pub fn register_hanim(&mut self) {
        if !HANIM_REGISTERED.swap(true, Ordering::AcqRel) {
            unsafe { sys::librw_register_hanim_plugin() }
        }
    }

    pub fn register_matfx(&mut self) {
        if !MATFX_REGISTERED.swap(true, Ordering::AcqRel) {
            unsafe { sys::librw_register_matfx_plugin() }
        }
    }

    pub fn register_raster(&mut self, size: usize, id: u32) -> Result<PluginOffset> {
        register_plugin("raster", size, id, sys::librw_raster_register_plugin)
    }

    pub fn register_texture(&mut self, size: usize, id: u32) -> Result<PluginOffset> {
        register_plugin("texture", size, id, sys::librw_texture_register_plugin)
    }
}

fn register_plugin(
    kind: &'static str,
    size: usize,
    id: u32,
    register: unsafe extern "C" fn(i32, u32) -> i32,
) -> Result<PluginOffset> {
    let size = i32::try_from(size).map_err(|_| Error::LengthOverflow)?;
    let offset = unsafe { register(size, id) };
    if offset < 0 {
        Err(Error::PluginRegistration { kind, id })
    } else {
        Ok(PluginOffset(offset))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluginOffset(i32);

impl PluginOffset {
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HAnimFlags(pub i32);

impl HAnimFlags {
    pub const SUB_HIERARCHY: Self = Self(0x1);
    pub const NO_MATRICES: Self = Self(0x2);
    pub const UPDATE_MODELING_MATRICES: Self = Self(0x1000);
    pub const UPDATE_LTMS: Self = Self(0x2000);
    pub const LOCAL_SPACE_MATRICES: Self = Self(0x4000);
}

impl core::ops::BitOr for HAnimFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub struct HAnimHierarchy<'engine> {
    raw: NonNull<c_void>,
    _engine: PhantomData<&'engine Engine>,
}

impl<'engine> HAnimHierarchy<'engine> {
    pub fn new(
        _engine: &'engine Engine,
        node_flags: &[i32],
        node_ids: &[i32],
        flags: HAnimFlags,
        max_key_size: usize,
    ) -> Result<Self> {
        if node_flags.len() != node_ids.len() {
            return Err(Error::MismatchedHAnimNodes);
        }
        let count = i32::try_from(node_flags.len()).map_err(|_| Error::LengthOverflow)?;
        let max_key_size = i32::try_from(max_key_size).map_err(|_| Error::LengthOverflow)?;
        let raw = unsafe {
            sys::librw_hanim_hierarchy_create(
                count,
                node_flags.as_ptr().cast_mut(),
                node_ids.as_ptr().cast_mut(),
                flags.0,
                max_key_size,
            )
        };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(Error::Null("creating an HAnim hierarchy"))?,
            _engine: PhantomData,
        })
    }

    pub fn node_count(&self) -> usize {
        unsafe { sys::librw_hanim_hierarchy_node_count(self.raw.as_ptr()) as usize }
    }

    pub fn matrices(&self) -> Option<&[Matrix]> {
        let raw = unsafe { sys::librw_hanim_hierarchy_matrices(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { slice::from_raw_parts(raw, self.node_count()) })
    }

    pub fn matrices_mut(&mut self) -> Option<&mut [Matrix]> {
        let raw = unsafe { sys::librw_hanim_hierarchy_matrices(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { slice::from_raw_parts_mut(raw, self.node_count()) })
    }

    pub fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for HAnimHierarchy<'_> {
    fn drop(&mut self) {
        unsafe { sys::librw_hanim_hierarchy_destroy(self.raw.as_ptr()) }
    }
}

#[derive(Clone, Copy)]
pub struct SkinRef<'a> {
    raw: NonNull<c_void>,
    _owner: PhantomData<&'a sys::rw_Geometry>,
}

impl SkinRef<'_> {
    pub fn as_raw(self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Geometry<'_> {
    pub fn skin(&self) -> Option<SkinRef<'_>> {
        if !SKIN_REGISTERED.load(Ordering::Acquire) {
            return None;
        }
        NonNull::new(unsafe { sys::librw_skin_get(self.raw.as_ptr()) }).map(|raw| SkinRef {
            raw,
            _owner: PhantomData,
        })
    }

    /// # Safety
    /// `skin` must be a valid librw `Skin` alloc. Geometry assumes ownership and will destroy it through the registered skin plugin
    pub unsafe fn set_skin_raw(&mut self, skin: *mut c_void) {
        unsafe { sys::librw_skin_set(self.raw.as_ptr(), skin) }
    }
}

impl Atomic<'_> {
    /// # Safety
    /// hierarchy must remain alive while the atomic can render with it
    pub unsafe fn set_skin_hierarchy(&mut self, hierarchy: Option<&HAnimHierarchy<'_>>) {
        unsafe {
            sys::librw_skin_set_hierarchy(
                self.raw.as_ptr(),
                hierarchy.map_or(core::ptr::null_mut(), HAnimHierarchy::as_raw),
            )
        }
    }

    pub fn set_skin_pipeline(&mut self, kind: i32) {
        unsafe { sys::librw_skin_set_pipeline(self.raw.as_ptr(), kind) }
    }
}

pub fn skin_geometry_offset() -> Option<PluginOffset> {
    SKIN_REGISTERED
        .load(Ordering::Acquire)
        .then(|| PluginOffset(unsafe { sys::librw_skin_geometry_offset() }))
}

pub fn skin_atomic_offset() -> Option<PluginOffset> {
    SKIN_REGISTERED
        .load(Ordering::Acquire)
        .then(|| PluginOffset(unsafe { sys::librw_skin_atomic_offset() }))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MatFxEffects(pub u32);

impl MatFxEffects {
    pub const NONE: Self = Self(sys::RW_MATFX_NOTHING);
    pub const BUMP_MAP: Self = Self(sys::RW_MATFX_BUMP_MAP);
    pub const ENV_MAP: Self = Self(sys::RW_MATFX_ENV_MAP);
    pub const BUMP_ENV_MAP: Self = Self(sys::RW_MATFX_BUMP_ENV_MAP);
    pub const DUAL: Self = Self(sys::RW_MATFX_DUAL);
    pub const UV_TRANSFORM: Self = Self(sys::RW_MATFX_UV_TRANSFORM);
    pub const DUAL_UV_TRANSFORM: Self = Self(sys::RW_MATFX_DUAL_UV_TRANSFORM);
}

pub struct MatFx<'a> {
    material: NonNull<sys::rw_Material>,
    _owner: PhantomData<&'a sys::rw_Material>,
}

impl Material<'_> {
    pub fn matfx(&self) -> Option<MatFx<'_>> {
        MATFX_REGISTERED.load(Ordering::Acquire).then_some(MatFx {
            material: self.raw,
            _owner: PhantomData,
        })
    }
}

impl MaterialRef<'_> {
    pub fn matfx(&self) -> Option<MatFx<'_>> {
        MATFX_REGISTERED.load(Ordering::Acquire).then_some(MatFx {
            material: self.raw,
            _owner: PhantomData,
        })
    }
}

impl MatFx<'_> {
    pub fn effects(&self) -> MatFxEffects {
        MatFxEffects(unsafe { sys::librw_matfx_effects(self.material.as_ptr()) })
    }

    pub fn env_texture(&self) -> Option<TextureRef<'_>> {
        NonNull::new(unsafe { sys::librw_matfx_env_texture(self.material.as_ptr()) }).map(|raw| {
            TextureRef {
                raw,
                _owner: PhantomData,
            }
        })
    }

    pub fn env_coefficient(&self) -> f32 {
        unsafe { sys::librw_matfx_env_coefficient(self.material.as_ptr()) }
    }

    pub fn dual_texture(&self) -> Option<TextureRef<'_>> {
        NonNull::new(unsafe { sys::librw_matfx_dual_texture(self.material.as_ptr()) }).map(|raw| {
            TextureRef {
                raw,
                _owner: PhantomData,
            }
        })
    }

    pub fn dual_blend(&self) -> (i32, i32) {
        unsafe {
            (
                sys::librw_matfx_dual_src_blend(self.material.as_ptr()),
                sys::librw_matfx_dual_dest_blend(self.material.as_ptr()),
            )
        }
    }
}

#[inline]
pub(crate) fn reset_registration_state() {
    SKIN_REGISTERED.store(false, Ordering::Release);
    MATFX_REGISTERED.store(false, Ordering::Release);
    NATIVE_DATA_REGISTERED.store(false, Ordering::Release);
    HANIM_REGISTERED.store(false, Ordering::Release);
}
