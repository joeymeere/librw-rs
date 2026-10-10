use core::ffi::c_int;

pub const RW_COMBINE_REPLACE: c_int = 0;
pub const RW_COMBINE_PRECONCAT: c_int = 1;
pub const RW_COMBINE_POSTCONCAT: c_int = 2;

pub const RW_ID_TEXTURE: u32 = 0x06;
pub const RW_ID_MATERIAL: u32 = 0x07;
pub const RW_ID_GEOMETRY: u32 = 0x0F;
pub const RW_ID_CLUMP: u32 = 0x10;
pub const RW_ID_TEXTURE_NATIVE: u32 = 0x15;
pub const RW_ID_TEX_DICTIONARY: u32 = 0x16;
pub const RW_ID_SKIN: u32 = 0x116;

pub const RW_MATFX_NOTHING: u32 = 0;
pub const RW_MATFX_BUMP_MAP: u32 = 1;
pub const RW_MATFX_ENV_MAP: u32 = 2;
pub const RW_MATFX_BUMP_ENV_MAP: u32 = 3;
pub const RW_MATFX_DUAL: u32 = 4;
pub const RW_MATFX_UV_TRANSFORM: u32 = 5;
pub const RW_MATFX_DUAL_UV_TRANSFORM: u32 = 6;

pub const RW_GEOMETRY_TRISTRIP: u32 = 0x01;
pub const RW_GEOMETRY_POSITIONS: u32 = 0x02;
pub const RW_GEOMETRY_TEXTURED: u32 = 0x04;
pub const RW_GEOMETRY_PRELIT: u32 = 0x08;
pub const RW_GEOMETRY_NORMALS: u32 = 0x10;
pub const RW_GEOMETRY_LIGHT: u32 = 0x20;
pub const RW_GEOMETRY_MODULATE: u32 = 0x40;
pub const RW_GEOMETRY_TEXTURED2: u32 = 0x80;
pub const RW_GEOMETRY_NATIVE: u32 = 0x0100_0000;

pub const RW_GEOMETRY_LOCK_POLYGONS: u32 = 0x0001;
pub const RW_GEOMETRY_LOCK_VERTICES: u32 = 0x0002;
pub const RW_GEOMETRY_LOCK_NORMALS: u32 = 0x0004;
pub const RW_GEOMETRY_LOCK_PRELIGHT: u32 = 0x0008;
pub const RW_GEOMETRY_LOCK_TEXCOORDS_ALL: u32 = 0x0ff0;
pub const RW_GEOMETRY_LOCK_ALL: u32 = 0x0fff;

pub const RW_ATOMIC_COLLISION_TEST: u32 = 0x01;
pub const RW_ATOMIC_RENDER: u32 = 0x04;

pub const RW_RASTER_NORMAL: c_int = 0x00;
pub const RW_RASTER_ZBUFFER: c_int = 0x01;
pub const RW_RASTER_CAMERA: c_int = 0x02;
pub const RW_RASTER_TEXTURE: c_int = 0x04;
pub const RW_RASTER_CAMERA_TEXTURE: c_int = 0x05;
pub const RW_RASTER_DONT_ALLOCATE: c_int = 0x80;
pub const RW_RASTER_LOCK_WRITE: c_int = 1;
pub const RW_RASTER_LOCK_READ: c_int = 2;
pub const RW_RASTER_LOCK_NO_FETCH: c_int = 4;
pub const RW_RASTER_FLIP_WAIT_VSYNC: u32 = 1;
pub const RW_RASTER_FORMAT_C8888: c_int = 0x0500;
pub const RW_RASTER_FORMAT_AUTO_MIPMAP: c_int = 0x1000;
pub const RW_RASTER_FORMAT_MIPMAP: c_int = 0x8000;

pub const RW_TEXTURE_FILTER_NEAREST: c_int = 1;
pub const RW_TEXTURE_FILTER_LINEAR: c_int = 2;
pub const RW_TEXTURE_FILTER_MIP_NEAREST: c_int = 3;
pub const RW_TEXTURE_FILTER_MIP_LINEAR: c_int = 4;
pub const RW_TEXTURE_FILTER_LINEAR_MIP_NEAREST: c_int = 5;
pub const RW_TEXTURE_FILTER_LINEAR_MIP_LINEAR: c_int = 6;

pub const RW_LIGHT_DIRECTIONAL: c_int = 1;
pub const RW_LIGHT_AMBIENT: c_int = 2;
pub const RW_LIGHT_POINT: c_int = 0x80;
pub const RW_LIGHT_SPOT: c_int = 0x81;
pub const RW_LIGHT_SOFT_SPOT: c_int = 0x82;
pub const RW_LIGHT_ATOMICS: u32 = 1;
pub const RW_LIGHT_WORLD: u32 = 2;

pub const RW_PLATFORM_GL3: c_int = 12;
pub const RW_MEMDUR_EVENT: u32 = 0x0003_0000;

pub const RW_PRIMITIVE_NONE: c_int = 0;
pub const RW_PRIMITIVE_LINE_LIST: c_int = 1;
pub const RW_PRIMITIVE_POLY_LINE: c_int = 2;
pub const RW_PRIMITIVE_TRI_LIST: c_int = 3;
pub const RW_PRIMITIVE_TRI_STRIP: c_int = 4;
pub const RW_PRIMITIVE_TRI_FAN: c_int = 5;
pub const RW_PRIMITIVE_POINT_LIST: c_int = 6;

pub const RW_RENDER_TEXTURE_RASTER: c_int = 0;
pub const RW_RENDER_TEXTURE_ADDRESS: c_int = 1;
pub const RW_RENDER_TEXTURE_ADDRESS_U: c_int = 2;
pub const RW_RENDER_TEXTURE_ADDRESS_V: c_int = 3;
pub const RW_RENDER_TEXTURE_FILTER: c_int = 4;
pub const RW_RENDER_VERTEX_ALPHA: c_int = 5;
pub const RW_RENDER_SRC_BLEND: c_int = 6;
pub const RW_RENDER_DEST_BLEND: c_int = 7;
pub const RW_RENDER_Z_TEST_ENABLE: c_int = 8;
pub const RW_RENDER_Z_WRITE_ENABLE: c_int = 9;
pub const RW_RENDER_FOG_ENABLE: c_int = 10;
pub const RW_RENDER_FOG_COLOR: c_int = 11;
pub const RW_RENDER_CULL_MODE: c_int = 12;
pub const RW_RENDER_ALPHA_TEST_FUNC: c_int = 22;
pub const RW_RENDER_ALPHA_TEST_REF: c_int = 23;

pub const RW_ALPHA_ALWAYS: c_int = 0;
pub const RW_ALPHA_GREATER_EQUAL: c_int = 1;
pub const RW_ALPHA_LESS: c_int = 2;
pub const RW_CULL_NONE: c_int = 1;
pub const RW_CULL_BACK: c_int = 2;
pub const RW_CULL_FRONT: c_int = 3;
pub const RW_BLEND_ZERO: c_int = 1;
pub const RW_BLEND_ONE: c_int = 2;
pub const RW_BLEND_SRC_ALPHA: c_int = 5;
pub const RW_BLEND_INV_SRC_ALPHA: c_int = 6;

pub const RW_IM3D_VERTEX_UV: u32 = 1;
pub const RW_IM3D_ALL_OPAQUE: u32 = 2;
pub const RW_IM3D_NO_CLIP: u32 = 4;
pub const RW_IM3D_VERTEX_XYZ: u32 = 8;
pub const RW_IM3D_VERTEX_RGBA: u32 = 16;

pub const RW_CAMERA_PERSPECTIVE: c_int = 1;
pub const RW_CAMERA_PARALLEL: c_int = 2;
