pub type rw_bool32 = i32;

macro_rules! opaque {
    ($($name:ident),+ $(,)?) => {$(
        #[repr(C)]
        pub struct $name { _private: [u8; 0] }
    )+};
}

opaque!(
    rw_Stream,
    rw_Frame,
    rw_Image,
    rw_Raster,
    rw_Texture,
    rw_Material,
    rw_Geometry,
    rw_Atomic,
    rw_Light,
    rw_Camera,
    rw_Clump,
    rw_World,
    rw_TexDictionary,
    rw_WorldLights,
    rw_Gl3Pipeline,
    rw_Gl3InstanceData,
    rw_Gl3Shader,
    rw_Skin,
);

pub type rw_light_callback = Option<unsafe extern "C" fn(*mut rw_Light, *mut core::ffi::c_void)>;

pub type rw_gl3_render_callback =
    Option<unsafe extern "C" fn(*mut rw_Atomic, *mut rw_Gl3InstanceData, *mut core::ffi::c_void)>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct rw_RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_RGBAf {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_V2d {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_V3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_TexCoords {
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_Sphere {
    pub center: rw_V3d,
    pub radius: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_BBox {
    pub sup: rw_V3d,
    pub inf: rw_V3d,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_Matrix {
    pub right: rw_V3d,
    pub flags: u32,
    pub up: rw_V3d,
    pub pad1: u32,
    pub at: rw_V3d,
    pub pad2: u32,
    pub pos: rw_V3d,
    pub pad3: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct rw_Triangle {
    pub v: [u16; 3],
    pub mat_id: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_SurfaceProperties {
    pub ambient: f32,
    pub specular: f32,
    pub diffuse: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_Im2DVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rw_Im3DVertex {
    pub position: rw_V3d,
    pub normal: rw_V3d,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub u: f32,
    pub v: f32,
}

// static asserts
const _: () = {
    use core::mem::{offset_of, size_of};

    assert!(size_of::<rw_RGBA>() == 4);
    assert!(offset_of!(rw_RGBA, alpha) == 3);
    assert!(size_of::<rw_RGBAf>() == 16);
    assert!(offset_of!(rw_RGBAf, alpha) == 12);
    assert!(size_of::<rw_V2d>() == 8);
    assert!(offset_of!(rw_V2d, y) == 4);
    assert!(size_of::<rw_V3d>() == 12);
    assert!(offset_of!(rw_V3d, z) == 8);
    assert!(size_of::<rw_TexCoords>() == 8);
    assert!(offset_of!(rw_TexCoords, v) == 4);
    assert!(size_of::<rw_Sphere>() == 16);
    assert!(offset_of!(rw_Sphere, radius) == 12);
    assert!(size_of::<rw_BBox>() == 24);
    assert!(offset_of!(rw_BBox, inf) == 12);
    assert!(size_of::<rw_Matrix>() == 64);
    assert!(offset_of!(rw_Matrix, pos) == 48);
    assert!(size_of::<rw_Triangle>() == 8);
    assert!(offset_of!(rw_Triangle, mat_id) == 6);
    assert!(size_of::<rw_SurfaceProperties>() == 12);
    assert!(offset_of!(rw_SurfaceProperties, diffuse) == 8);
    assert!(size_of::<rw_Im2DVertex>() == 28);
    assert!(offset_of!(rw_Im2DVertex, red) == 16);
    assert!(offset_of!(rw_Im2DVertex, u) == 20);
    assert!(size_of::<rw_Im3DVertex>() == 36);
    assert!(offset_of!(rw_Im3DVertex, red) == 24);
    assert!(offset_of!(rw_Im3DVertex, u) == 28);
};
