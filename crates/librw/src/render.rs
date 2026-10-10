use core::ffi::c_void;

use librw_sys as sys;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderState {
    TextureRaster = sys::RW_RENDER_TEXTURE_RASTER,
    TextureAddress = sys::RW_RENDER_TEXTURE_ADDRESS,
    TextureAddressU = sys::RW_RENDER_TEXTURE_ADDRESS_U,
    TextureAddressV = sys::RW_RENDER_TEXTURE_ADDRESS_V,
    TextureFilter = sys::RW_RENDER_TEXTURE_FILTER,
    VertexAlpha = sys::RW_RENDER_VERTEX_ALPHA,
    SourceBlend = sys::RW_RENDER_SRC_BLEND,
    DestinationBlend = sys::RW_RENDER_DEST_BLEND,
    ZTestEnable = sys::RW_RENDER_Z_TEST_ENABLE,
    ZWriteEnable = sys::RW_RENDER_Z_WRITE_ENABLE,
    FogEnable = sys::RW_RENDER_FOG_ENABLE,
    FogColor = sys::RW_RENDER_FOG_COLOR,
    CullMode = sys::RW_RENDER_CULL_MODE,
    AlphaTestFunction = sys::RW_RENDER_ALPHA_TEST_FUNC,
    AlphaTestReference = sys::RW_RENDER_ALPHA_TEST_REF,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlphaTestFunction {
    Always = sys::RW_ALPHA_ALWAYS as u32,
    GreaterEqual = sys::RW_ALPHA_GREATER_EQUAL as u32,
    Less = sys::RW_ALPHA_LESS as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CullMode {
    None = sys::RW_CULL_NONE as u32,
    Back = sys::RW_CULL_BACK as u32,
    Front = sys::RW_CULL_FRONT as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFunction {
    Zero = sys::RW_BLEND_ZERO as u32,
    One = sys::RW_BLEND_ONE as u32,
    SourceAlpha = sys::RW_BLEND_SRC_ALPHA as u32,
    InverseSourceAlpha = sys::RW_BLEND_INV_SRC_ALPHA as u32,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    None = sys::RW_PRIMITIVE_NONE,
    LineList = sys::RW_PRIMITIVE_LINE_LIST,
    PolyLine = sys::RW_PRIMITIVE_POLY_LINE,
    TriangleList = sys::RW_PRIMITIVE_TRI_LIST,
    TriangleStrip = sys::RW_PRIMITIVE_TRI_STRIP,
    TriangleFan = sys::RW_PRIMITIVE_TRI_FAN,
    PointList = sys::RW_PRIMITIVE_POINT_LIST,
}

pub fn set_render_state(state: RenderState, value: u32) {
    unsafe { sys::librw_set_render_state(state as i32, value) }
}

pub fn render_state(state: RenderState) -> u32 {
    unsafe { sys::librw_get_render_state(state as i32) }
}

/// # Safety
///     ptr must satisfy the selected render states contract and
/// remain valid for as long as librw retains it
pub unsafe fn set_render_state_ptr(state: RenderState, value: *mut c_void) {
    unsafe { sys::librw_set_render_state_ptr(state as i32, value) }
}

pub fn render_state_ptr(state: RenderState) -> *mut c_void {
    unsafe { sys::librw_get_render_state_ptr(state as i32) }
}
