use core::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
    ptr,
    ptr::NonNull,
};
use std::panic::{AssertUnwindSafe, catch_unwind};

use librw_sys as sys;

use crate::util::cstr;
use crate::{
    Atomic, AtomicRef, Engine, Error, GeometryFlags, MaterialRef, Matrix, PluginRegistrar, Result,
    Rgba, SurfaceProperties, TextureRef,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gl3Attribute {
    pub location: u32,
    pub kind: i32,
    pub normalized: bool,
    pub size: i32,
    pub stride: u32,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Gl3Mesh<'a> {
    pub index_count: u32,
    pub min_vertex: u32,
    pub vertex_count: i32,
    pub material: Option<MaterialRef<'a>>,
    pub vertex_alpha: bool,
    pub program: u32,
    pub offset: u32,
}

#[derive(Clone, Copy)]
pub struct Gl3Instance<'a> {
    raw: NonNull<sys::rw_Gl3InstanceData>,
    _lifetime: PhantomData<&'a sys::rw_Gl3InstanceData>,
}

#[derive(Clone, Copy)]
pub struct Gl3Shader<'a> {
    raw: NonNull<sys::rw_Gl3Shader>,
    _lifetime: PhantomData<&'a sys::rw_Gl3Shader>,
}

impl<'engine> Gl3Shader<'engine> {
    pub fn new(_engine: &'engine Engine, vertex: &[&CStr], fragment: &[&CStr]) -> Result<Self> {
        let list = |sources: &[&CStr]| {
            sources
                .iter()
                .map(|s| s.as_ptr())
                .chain([ptr::null::<c_char>()])
                .collect::<Vec<_>>()
        };
        let (mut vertex, mut fragment) = (list(vertex), list(fragment));
        let raw =
            unsafe { sys::librw_gl3_shader_create(vertex.as_mut_ptr(), fragment.as_mut_ptr()) };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(Error::Null("creating a GL3 shader"))?,
            _lifetime: PhantomData,
        })
    }
}

impl Gl3Shader<'_> {
    pub fn program(&self) -> u32 {
        unsafe { sys::librw_gl3_shader_program(self.raw.as_ptr()) }
    }

    pub fn bind(&mut self) {
        unsafe { sys::librw_gl3_shader_use(self.raw.as_ptr()) }
    }
}

impl<'a> Gl3Instance<'a> {
    pub fn vertex_buffer(&self) -> u32 {
        unsafe { sys::librw_gl3_instance_vbo(self.raw.as_ptr()) }
    }

    pub fn index_buffer(&self) -> u32 {
        unsafe { sys::librw_gl3_instance_ibo(self.raw.as_ptr()) }
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { sys::librw_gl3_instance_vertex_count(self.raw.as_ptr()) as usize }
    }

    pub fn index_count(&self) -> usize {
        unsafe { sys::librw_gl3_instance_index_count(self.raw.as_ptr()) as usize }
    }

    pub fn mesh_count(&self) -> usize {
        unsafe { sys::librw_gl3_instance_mesh_count(self.raw.as_ptr()) as usize }
    }

    pub fn attribute_count(&self) -> usize {
        unsafe { sys::librw_gl3_instance_attribute_count(self.raw.as_ptr()) as usize }
    }

    pub fn attribute(&self, index: usize) -> Option<Gl3Attribute> {
        let index = i32::try_from(index).ok()?;
        let (mut location, mut kind, mut normalized, mut size, mut stride, mut offset) =
            (0, 0, 0, 0, 0, 0);
        let found = unsafe {
            sys::librw_gl3_instance_attribute(
                self.raw.as_ptr(),
                index,
                &mut location,
                &mut kind,
                &mut normalized,
                &mut size,
                &mut stride,
                &mut offset,
            )
        };
        (found != 0).then_some(Gl3Attribute {
            location,
            kind,
            normalized: normalized != 0,
            size,
            stride,
            offset,
        })
    }

    pub fn setup_vertex_input(&self) {
        unsafe { sys::librw_gl3_setup_vertex_input(self.raw.as_ptr()) }
    }

    pub fn teardown_vertex_input(&self) {
        unsafe { sys::librw_gl3_teardown_vertex_input(self.raw.as_ptr()) }
    }

    pub fn draw_mesh(&self, index: usize) {
        if let Ok(index) = i32::try_from(index) {
            unsafe { sys::librw_gl3_draw_mesh(self.raw.as_ptr(), index) }
        }
    }

    pub fn mesh(&self, index: usize) -> Option<Gl3Mesh<'a>> {
        let index = i32::try_from(index).ok()?;
        let (mut index_count, mut min_vertex, mut vertex_count) = (0, 0, 0);
        let mut material = core::ptr::null_mut();
        let (mut vertex_alpha, mut program, mut offset) = (0, 0, 0);
        let found = unsafe {
            sys::librw_gl3_instance_mesh(
                self.raw.as_ptr(),
                index,
                &mut index_count,
                &mut min_vertex,
                &mut vertex_count,
                &mut material,
                &mut vertex_alpha,
                &mut program,
                &mut offset,
            )
        };
        (found != 0).then_some(Gl3Mesh {
            index_count,
            min_vertex,
            vertex_count,
            material: NonNull::new(material).map(|raw| MaterialRef {
                raw,
                _owner: PhantomData,
            }),
            vertex_alpha: vertex_alpha != 0,
            program,
            offset,
        })
    }

    pub fn set_world_matrix(&self, matrix: &Matrix) {
        unsafe { sys::librw_gl3_set_world_matrix(matrix) }
    }

    pub fn set_material(&self, flags: GeometryFlags, color: Rgba, surface: SurfaceProperties) {
        unsafe { sys::librw_gl3_set_material(flags.0, &color, &surface) }
    }

    pub fn set_texture(&self, stage: u32, texture: Option<&TextureRef<'_>>) {
        // librw MAXNUMSTAGES
        if stage < 8 {
            unsafe {
                sys::librw_gl3_set_texture(
                    stage as i32,
                    texture.map_or(ptr::null_mut(), |t| t.as_raw()),
                )
            }
        }
    }

    pub fn upload_skin_matrices(&self, atomic: &AtomicRef<'_>) {
        if atomic.geometry().is_some_and(|g| g.skin().is_some()) {
            unsafe { sys::librw_gl3_upload_skin_matrices(atomic.as_raw()) }
        }
    }
}

type RenderCallback = dyn for<'a> FnMut(&AtomicRef<'a>, Gl3Instance<'a>);

struct CallbackState {
    callback: Box<RenderCallback>,
}

pub struct Gl3Pipeline<'engine> {
    raw: NonNull<sys::rw_Gl3Pipeline>,
    state: NonNull<CallbackState>,
    _engine: PhantomData<&'engine Engine>,
}

impl<'engine> Gl3Pipeline<'engine> {
    pub fn new(
        engine: &'engine Engine,
        callback: impl for<'a> FnMut(&AtomicRef<'a>, Gl3Instance<'a>) + 'static,
    ) -> Result<Self> {
        Self::create_like(engine, ptr::null_mut(), callback)
    }

    pub fn new_skin(
        engine: &'engine Engine,
        callback: impl for<'a> FnMut(&AtomicRef<'a>, Gl3Instance<'a>) + 'static,
    ) -> Result<Self> {
        let base = unsafe { sys::librw_gl3_skin_pipeline() };
        if base.is_null() {
            return Err(Error::PluginNotRegistered("skin"));
        }
        Self::create_like(engine, base, callback)
    }

    fn create_like(
        _engine: &'engine Engine,
        base: *mut sys::rw_Gl3Pipeline,
        callback: impl for<'a> FnMut(&AtomicRef<'a>, Gl3Instance<'a>) + 'static,
    ) -> Result<Self> {
        let state = NonNull::from(Box::leak(Box::new(CallbackState {
            callback: Box::new(callback),
        })));
        let raw = unsafe {
            sys::librw_gl3_pipeline_create_like(
                base,
                Some(render_trampoline),
                state.as_ptr().cast(),
            )
        };
        match NonNull::new(raw) {
            Some(raw) => Ok(Self {
                raw,
                state,
                _engine: PhantomData,
            }),
            None => {
                unsafe { drop(Box::from_raw(state.as_ptr())) };
                Err(Error::Null("creating a GL3 pipeline"))
            }
        }
    }

    pub fn render(&mut self, atomic: &mut Atomic<'engine>) {
        unsafe {
            let previous = sys::librw_atomic_gl3_pipeline(atomic.raw.as_ptr());
            sys::librw_atomic_set_gl3_pipeline(atomic.raw.as_ptr(), self.raw.as_ptr());
            sys::librw_atomic_render(atomic.raw.as_ptr());
            sys::librw_atomic_set_gl3_pipeline(atomic.raw.as_ptr(), previous);
        }
    }
}

impl<'engine> Atomic<'engine> {
    pub fn set_gl3_pipeline(&mut self, pipeline: &'engine Gl3Pipeline<'engine>) {
        unsafe { sys::librw_atomic_set_gl3_pipeline(self.raw.as_ptr(), pipeline.raw.as_ptr()) }
    }
}

impl Drop for Gl3Pipeline<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::librw_gl3_pipeline_destroy(self.raw.as_ptr());
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

unsafe extern "C" fn render_trampoline(
    atomic: *mut sys::rw_Atomic,
    instance: *mut sys::rw_Gl3InstanceData,
    user_data: *mut c_void,
) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &mut *user_data.cast::<CallbackState>() };
        let instance = Gl3Instance {
            raw: NonNull::new(instance).expect("librw passed a null GL3 instance"),
            _lifetime: PhantomData,
        };
        let atomic = AtomicRef {
            raw: NonNull::new(atomic).expect("librw passed a null atomic"),
            _owner: PhantomData,
        };
        (state.callback)(&atomic, instance);
    }));
    if result.is_err() {
        std::process::abort();
    }
}

pub fn native_raster_offset() -> i32 {
    unsafe { sys::librw_gl3_native_raster_offset() }
}

/// # Safety
/// data must point to the value expected by the registered uniform
pub unsafe fn set_uniform(id: i32, data: *mut c_void) {
    unsafe { sys::librw_gl3_set_uniform(id, data) }
}

pub fn current_shader_program() -> Option<u32> {
    NonNull::new(unsafe { sys::librw_gl3_current_shader() })
        .map(|shader| unsafe { sys::librw_gl3_shader_program(shader.as_ptr()) })
}

pub fn current_shader(_engine: &Engine) -> Option<Gl3Shader<'_>> {
    NonNull::new(unsafe { sys::librw_gl3_current_shader() }).map(|raw| Gl3Shader {
        raw,
        _lifetime: PhantomData,
    })
}

pub fn shader_decl(_engine: &Engine) -> &CStr {
    cstr(unsafe { sys::librw_gl3_shader_decl() })
}

pub fn shader_header_vertex() -> &'static CStr {
    cstr(unsafe { sys::librw_gl3_shader_header_vs() })
}

pub fn shader_header_fragment() -> &'static CStr {
    cstr(unsafe { sys::librw_gl3_shader_header_fs() })
}

/// registered vec4[N uniform
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gl3Uniform<const N: usize> {
    id: i32,
}

impl<const N: usize> Gl3Uniform<N> {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set(&self, values: &[[f32; 4]; N]) {
        unsafe { sys::librw_gl3_set_uniform(self.id, values.as_ptr().cast_mut().cast()) }
    }
}

impl PluginRegistrar<'_> {
    pub fn register_gl3_uniform<const N: usize>(&mut self, name: &CStr) -> Result<Gl3Uniform<N>> {
        let count = i32::try_from(N).map_err(|_| Error::LengthOverflow)?;
        let id = unsafe {
            sys::librw_gl3_register_uniform(name.as_ptr(), sys::RW_GL3_UNIFORM_VEC4, count)
        };
        let (mut kind, mut registered) = (0, 0);
        if id < 0
            || unsafe { sys::librw_gl3_uniform_info(id, &mut kind, &mut registered) } == 0
            || kind != sys::RW_GL3_UNIFORM_VEC4
            || registered != count
        {
            return Err(Error::Engine("registering a GL3 uniform"));
        }
        Ok(Gl3Uniform { id })
    }
}
