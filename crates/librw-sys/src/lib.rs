#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

pub mod consts;
pub mod types;

pub use consts::*;
pub use types::*;

unsafe extern "C" {
    pub fn librw_engine_init() -> rw_bool32;
    pub fn librw_engine_open(params: *mut c_void) -> rw_bool32;
    pub fn librw_engine_start() -> rw_bool32;
    pub fn librw_engine_stop();
    pub fn librw_engine_close();
    pub fn librw_engine_term();
    pub fn librw_get_platform() -> c_int;
    pub fn librw_set_platform(platform: c_int);
    pub fn librw_get_version() -> c_int;
    pub fn librw_set_version(version: c_int);

    pub fn librw_stream_memory_new(data: *mut u8, len: u32, capacity: u32) -> *mut rw_Stream;
    pub fn librw_stream_memory_delete(stream: *mut rw_Stream);
    pub fn librw_stream_memory_data(stream: *mut rw_Stream) -> *mut u8;
    pub fn librw_stream_memory_len(stream: *mut rw_Stream) -> u32;
    pub fn librw_stream_file_new(path: *const c_char, mode: *const c_char) -> *mut rw_Stream;
    pub fn librw_stream_file_delete(stream: *mut rw_Stream);
    pub fn librw_stream_read(stream: *mut rw_Stream, data: *mut c_void, len: u32) -> u32;
    pub fn librw_stream_write(stream: *mut rw_Stream, data: *const c_void, len: u32) -> u32;
    pub fn librw_stream_seek(stream: *mut rw_Stream, offset: i32, whence: c_int);
    pub fn librw_stream_tell(stream: *mut rw_Stream) -> u32;
    pub fn librw_stream_eof(stream: *mut rw_Stream) -> rw_bool32;
    pub fn librw_stream_find_chunk(
        stream: *mut rw_Stream,
        kind: u32,
        len: *mut u32,
        version: *mut u32,
    ) -> rw_bool32;

    pub fn librw_frame_create() -> *mut rw_Frame;
    pub fn librw_frame_destroy(frame: *mut rw_Frame);
    pub fn librw_frame_destroy_hierarchy(frame: *mut rw_Frame);
    pub fn librw_frame_add_child(parent: *mut rw_Frame, child: *mut rw_Frame, append: rw_bool32);
    pub fn librw_frame_remove_child(frame: *mut rw_Frame) -> *mut rw_Frame;
    pub fn librw_frame_parent(frame: *const rw_Frame) -> *mut rw_Frame;
    pub fn librw_frame_child(frame: *const rw_Frame) -> *mut rw_Frame;
    pub fn librw_frame_next(frame: *const rw_Frame) -> *mut rw_Frame;
    pub fn librw_frame_matrix(frame: *mut rw_Frame) -> *mut rw_Matrix;
    pub fn librw_frame_ltm(frame: *mut rw_Frame) -> *const rw_Matrix;
    pub fn librw_frame_translate(frame: *mut rw_Frame, value: *const rw_V3d, combine: c_int);
    pub fn librw_frame_scale(frame: *mut rw_Frame, value: *const rw_V3d, combine: c_int);
    pub fn librw_frame_rotate(
        frame: *mut rw_Frame,
        axis: *const rw_V3d,
        angle: f32,
        combine: c_int,
    );

    pub fn librw_image_create(width: i32, height: i32, depth: i32) -> *mut rw_Image;
    pub fn librw_image_destroy(image: *mut rw_Image);
    pub fn librw_image_allocate(image: *mut rw_Image);
    pub fn librw_image_read(path: *const c_char) -> *mut rw_Image;
    pub fn librw_image_width(image: *const rw_Image) -> i32;
    pub fn librw_image_height(image: *const rw_Image) -> i32;
    pub fn librw_image_depth(image: *const rw_Image) -> i32;
    pub fn librw_image_stride(image: *const rw_Image) -> i32;
    pub fn librw_image_pixels(image: *mut rw_Image) -> *mut u8;
    pub fn librw_image_palette(image: *mut rw_Image) -> *mut u8;

    pub fn librw_raster_create(
        width: i32,
        height: i32,
        depth: i32,
        format: i32,
        platform: i32,
    ) -> *mut rw_Raster;
    pub fn librw_raster_from_image(image: *mut rw_Image, platform: i32) -> *mut rw_Raster;
    pub fn librw_raster_to_image(raster: *mut rw_Raster) -> *mut rw_Image;
    pub fn librw_raster_destroy(raster: *mut rw_Raster);
    pub fn librw_raster_width(raster: *const rw_Raster) -> i32;
    pub fn librw_raster_height(raster: *const rw_Raster) -> i32;
    pub fn librw_raster_depth(raster: *const rw_Raster) -> i32;
    pub fn librw_raster_stride(raster: *const rw_Raster) -> i32;
    pub fn librw_raster_num_levels(raster: *mut rw_Raster) -> i32;
    pub fn librw_raster_lock(raster: *mut rw_Raster, level: i32, mode: i32) -> *mut u8;
    pub fn librw_raster_unlock(raster: *mut rw_Raster, level: i32);

    pub fn librw_texture_create(raster: *mut rw_Raster) -> *mut rw_Texture;
    pub fn librw_texture_destroy(texture: *mut rw_Texture);
    pub fn librw_texture_name(texture: *const rw_Texture) -> *const c_char;
    pub fn librw_texture_mask(texture: *const rw_Texture) -> *const c_char;
    pub fn librw_texture_set_name(texture: *mut rw_Texture, name: *const c_char);
    pub fn librw_texture_set_mask(texture: *mut rw_Texture, mask: *const c_char);
    pub fn librw_texture_raster(texture: *const rw_Texture) -> *mut rw_Raster;
    pub fn librw_texture_filter(texture: *const rw_Texture) -> c_int;
    pub fn librw_texture_set_filter(texture: *mut rw_Texture, filter: c_int);

    pub fn librw_material_create() -> *mut rw_Material;
    pub fn librw_material_destroy(material: *mut rw_Material);
    pub fn librw_material_set_texture(material: *mut rw_Material, texture: *mut rw_Texture);
    pub fn librw_material_texture(material: *const rw_Material) -> *mut rw_Texture;
    pub fn librw_material_color(material: *mut rw_Material) -> *mut rw_RGBA;
    pub fn librw_material_surface_properties(
        material: *mut rw_Material,
    ) -> *mut rw_SurfaceProperties;

    pub fn librw_geometry_create(vertices: i32, triangles: i32, flags: u32) -> *mut rw_Geometry;
    pub fn librw_geometry_destroy(geometry: *mut rw_Geometry);
    pub fn librw_geometry_lock(geometry: *mut rw_Geometry, flags: i32);
    pub fn librw_geometry_unlock(geometry: *mut rw_Geometry);
    pub fn librw_geometry_num_vertices(geometry: *const rw_Geometry) -> i32;
    pub fn librw_geometry_num_triangles(geometry: *const rw_Geometry) -> i32;
    pub fn librw_geometry_num_morph_targets(geometry: *const rw_Geometry) -> i32;
    pub fn librw_geometry_flags(geometry: *const rw_Geometry) -> u32;
    pub fn librw_geometry_vertices(geometry: *mut rw_Geometry, target: i32) -> *mut rw_V3d;
    pub fn librw_geometry_normals(geometry: *mut rw_Geometry, target: i32) -> *mut rw_V3d;
    pub fn librw_geometry_triangles(geometry: *mut rw_Geometry) -> *mut rw_Triangle;
    pub fn librw_geometry_colors(geometry: *mut rw_Geometry) -> *mut rw_RGBA;
    pub fn librw_geometry_tex_coords(geometry: *mut rw_Geometry, set: i32) -> *mut rw_TexCoords;
    pub fn librw_geometry_add_material(
        geometry: *mut rw_Geometry,
        material: *mut rw_Material,
    ) -> i32;
    pub fn librw_geometry_material_count(geometry: *const rw_Geometry) -> i32;
    pub fn librw_geometry_material(geometry: *const rw_Geometry, index: i32) -> *mut rw_Material;
    pub fn librw_geometry_calculate_bounding_sphere(geometry: *mut rw_Geometry);

    pub fn librw_atomic_create() -> *mut rw_Atomic;
    pub fn librw_atomic_destroy(atomic: *mut rw_Atomic);
    pub fn librw_atomic_set_geometry(
        atomic: *mut rw_Atomic,
        geometry: *mut rw_Geometry,
        flags: u32,
    );
    pub fn librw_atomic_geometry(atomic: *const rw_Atomic) -> *mut rw_Geometry;
    pub fn librw_atomic_set_frame(atomic: *mut rw_Atomic, frame: *mut rw_Frame);
    pub fn librw_atomic_frame(atomic: *const rw_Atomic) -> *mut rw_Frame;
    pub fn librw_atomic_flags(atomic: *const rw_Atomic) -> u32;
    pub fn librw_atomic_set_flags(atomic: *mut rw_Atomic, flags: u32);
    pub fn librw_atomic_render(atomic: *mut rw_Atomic);

    pub fn librw_clump_create() -> *mut rw_Clump;
    pub fn librw_clump_destroy(clump: *mut rw_Clump);
    pub fn librw_clump_read(stream: *mut rw_Stream) -> *mut rw_Clump;
    pub fn librw_clump_write(clump: *mut rw_Clump, stream: *mut rw_Stream) -> rw_bool32;
    pub fn librw_clump_set_frame(clump: *mut rw_Clump, frame: *mut rw_Frame);
    pub fn librw_clump_frame(clump: *const rw_Clump) -> *mut rw_Frame;
    pub fn librw_clump_add_atomic(clump: *mut rw_Clump, atomic: *mut rw_Atomic);
    pub fn librw_clump_remove_atomic(clump: *mut rw_Clump, atomic: *mut rw_Atomic);
    pub fn librw_clump_atomic_count(clump: *const rw_Clump) -> i32;
    pub fn librw_clump_atomic(clump: *const rw_Clump, index: i32) -> *mut rw_Atomic;
    pub fn librw_clump_render(clump: *mut rw_Clump);

    pub fn librw_tex_dictionary_create() -> *mut rw_TexDictionary;
    pub fn librw_tex_dictionary_destroy(dict: *mut rw_TexDictionary);
    pub fn librw_tex_dictionary_read(stream: *mut rw_Stream) -> *mut rw_TexDictionary;
    pub fn librw_tex_dictionary_write(dict: *mut rw_TexDictionary, stream: *mut rw_Stream);
    pub fn librw_tex_dictionary_add(dict: *mut rw_TexDictionary, texture: *mut rw_Texture);
    pub fn librw_tex_dictionary_remove(dict: *mut rw_TexDictionary, texture: *mut rw_Texture);
    pub fn librw_tex_dictionary_count(dict: *const rw_TexDictionary) -> i32;
    pub fn librw_tex_dictionary_texture(
        dict: *const rw_TexDictionary,
        index: i32,
    ) -> *mut rw_Texture;
    pub fn librw_tex_dictionary_find(
        dict: *mut rw_TexDictionary,
        name: *const c_char,
    ) -> *mut rw_Texture;
    pub fn librw_tex_dictionary_set_current(dict: *mut rw_TexDictionary);

    pub fn librw_light_create(kind: i32) -> *mut rw_Light;
    pub fn librw_light_destroy(light: *mut rw_Light);
    pub fn librw_light_set_frame(light: *mut rw_Light, frame: *mut rw_Frame);
    pub fn librw_light_set_color(light: *mut rw_Light, red: f32, green: f32, blue: f32);
    pub fn librw_light_set_radius(light: *mut rw_Light, radius: f32);

    pub fn librw_camera_create() -> *mut rw_Camera;
    pub fn librw_camera_destroy(camera: *mut rw_Camera);
    pub fn librw_camera_set_frame(camera: *mut rw_Camera, frame: *mut rw_Frame);
    pub fn librw_camera_set_frame_buffer(camera: *mut rw_Camera, raster: *mut rw_Raster);
    pub fn librw_camera_set_z_buffer(camera: *mut rw_Camera, raster: *mut rw_Raster);
    pub fn librw_camera_set_near_plane(camera: *mut rw_Camera, value: f32);
    pub fn librw_camera_set_far_plane(camera: *mut rw_Camera, value: f32);
    pub fn librw_camera_set_fov(camera: *mut rw_Camera, degrees: f32, ratio: f32);
    pub fn librw_camera_clear(camera: *mut rw_Camera, color: *const rw_RGBA, mode: u32);
    pub fn librw_camera_begin_update(camera: *mut rw_Camera);
    pub fn librw_camera_end_update(camera: *mut rw_Camera);

    pub fn librw_world_create(bounds: *const rw_BBox) -> *mut rw_World;
    pub fn librw_world_destroy(world: *mut rw_World);
    pub fn librw_world_add_clump(world: *mut rw_World, clump: *mut rw_Clump);
    pub fn librw_world_remove_clump(world: *mut rw_World, clump: *mut rw_Clump);
    pub fn librw_world_add_light(world: *mut rw_World, light: *mut rw_Light);
    pub fn librw_world_remove_light(world: *mut rw_World, light: *mut rw_Light);
    pub fn librw_world_add_camera(world: *mut rw_World, camera: *mut rw_Camera);
    pub fn librw_world_remove_camera(world: *mut rw_World, camera: *mut rw_Camera);
    pub fn librw_world_render(world: *mut rw_World);
}
