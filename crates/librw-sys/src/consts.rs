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

pub const RW_LIGHT_DIRECTIONAL: c_int = 1;
pub const RW_LIGHT_AMBIENT: c_int = 2;
pub const RW_LIGHT_POINT: c_int = 0x80;
pub const RW_LIGHT_SPOT: c_int = 0x81;
pub const RW_LIGHT_SOFT_SPOT: c_int = 0x82;
