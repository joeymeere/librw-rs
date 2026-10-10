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
);

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
