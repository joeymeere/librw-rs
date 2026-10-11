#ifndef LIBRW_RS_SYS_H
#define LIBRW_RS_SYS_H

/* C ABI implemented by src/librw_sys.cpp
 * lib.rs mirrors this header, and C/C++ projects can also just use this directly */

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int32_t rw_bool32;
typedef struct rw_Stream rw_Stream;
typedef struct rw_Frame rw_Frame;
typedef struct rw_Image rw_Image;
typedef struct rw_Raster rw_Raster;
typedef struct rw_Texture rw_Texture;
typedef struct rw_Material rw_Material;
typedef struct rw_Geometry rw_Geometry;
typedef struct rw_Atomic rw_Atomic;
typedef struct rw_Light rw_Light;
typedef struct rw_Camera rw_Camera;
typedef struct rw_Clump rw_Clump;
typedef struct rw_World rw_World;
typedef struct rw_TexDictionary rw_TexDictionary;
typedef struct rw_WorldLights rw_WorldLights;
typedef struct rw_Gl3Pipeline rw_Gl3Pipeline;
typedef struct rw_Gl3InstanceData rw_Gl3InstanceData;
typedef struct rw_Gl3Shader rw_Gl3Shader;
typedef struct rw_Skin rw_Skin;

typedef struct { uint8_t red, green, blue, alpha; } rw_RGBA;
typedef struct { float red, green, blue, alpha; } rw_RGBAf;
typedef struct { float x, y; } rw_V2d;
typedef struct { float x, y, z; } rw_V3d;
typedef struct { float u, v; } rw_TexCoords;
typedef struct { rw_V3d center; float radius; } rw_Sphere;
typedef struct { rw_V3d sup, inf; } rw_BBox;
typedef struct {
    rw_V3d right; uint32_t flags;
    rw_V3d up; uint32_t pad1;
    rw_V3d at; uint32_t pad2;
    rw_V3d pos; uint32_t pad3;
} rw_Matrix;
typedef struct { uint16_t v[3]; uint16_t mat_id; } rw_Triangle;
typedef struct { float ambient, specular, diffuse; } rw_SurfaceProperties;
typedef struct {
    float x, y, z, w;
    uint8_t red, green, blue, alpha;
    float u, v;
} rw_Im2DVertex;
typedef struct {
    rw_V3d position, normal;
    uint8_t red, green, blue, alpha;
    float u, v;
} rw_Im3DVertex;

enum {
    RW_COMBINE_REPLACE = 0,
    RW_COMBINE_PRECONCAT = 1,
    RW_COMBINE_POSTCONCAT = 2,

    RW_ID_TEXTURE = 0x06,
    RW_ID_MATERIAL = 0x07,
    RW_ID_GEOMETRY = 0x0F,
    RW_ID_CLUMP = 0x10,
    RW_ID_TEXTURE_NATIVE = 0x15,
    RW_ID_TEX_DICTIONARY = 0x16,
    RW_ID_SKIN = 0x116,

    RW_GEOMETRY_TRISTRIP = 0x01,
    RW_GEOMETRY_POSITIONS = 0x02,
    RW_GEOMETRY_TEXTURED = 0x04,
    RW_GEOMETRY_PRELIT = 0x08,
    RW_GEOMETRY_NORMALS = 0x10,
    RW_GEOMETRY_LIGHT = 0x20,
    RW_GEOMETRY_MODULATE = 0x40,
    RW_GEOMETRY_TEXTURED2 = 0x80,
    RW_GEOMETRY_NATIVE = 0x01000000,
    RW_GEOMETRY_LOCK_POLYGONS = 0x0001,
    RW_GEOMETRY_LOCK_VERTICES = 0x0002,
    RW_GEOMETRY_LOCK_NORMALS = 0x0004,
    RW_GEOMETRY_LOCK_PRELIGHT = 0x0008,
    RW_GEOMETRY_LOCK_TEXCOORDS_ALL = 0x0FF0,
    RW_GEOMETRY_LOCK_ALL = 0x0FFF,

    RW_ATOMIC_COLLISION_TEST = 0x01,
    RW_ATOMIC_RENDER = 0x04,

    RW_RASTER_NORMAL = 0x00,
    RW_RASTER_ZBUFFER = 0x01,
    RW_RASTER_CAMERA = 0x02,
    RW_RASTER_TEXTURE = 0x04,
    RW_RASTER_CAMERA_TEXTURE = 0x05,
    RW_RASTER_DONT_ALLOCATE = 0x80,
    RW_RASTER_LOCK_WRITE = 1,
    RW_RASTER_LOCK_READ = 2,
    RW_RASTER_LOCK_NO_FETCH = 4,
    RW_RASTER_FLIP_WAIT_VSYNC = 1,
    RW_RASTER_FORMAT_C8888 = 0x0500,
    RW_RASTER_FORMAT_AUTO_MIPMAP = 0x1000,
    RW_RASTER_FORMAT_MIPMAP = 0x8000,

    RW_TEXTURE_FILTER_NEAREST = 1,
    RW_TEXTURE_FILTER_LINEAR = 2,
    RW_TEXTURE_FILTER_MIP_NEAREST = 3,
    RW_TEXTURE_FILTER_MIP_LINEAR = 4,
    RW_TEXTURE_FILTER_LINEAR_MIP_NEAREST = 5,
    RW_TEXTURE_FILTER_LINEAR_MIP_LINEAR = 6,

    RW_LIGHT_DIRECTIONAL = 1,
    RW_LIGHT_AMBIENT = 2,
    RW_LIGHT_POINT = 0x80,
    RW_LIGHT_SPOT = 0x81,
    RW_LIGHT_SOFT_SPOT = 0x82,
    RW_LIGHT_ATOMICS = 1,
    RW_LIGHT_WORLD = 2,

    RW_PLATFORM_GL3 = 12,
    RW_MEMDUR_EVENT = 0x00030000,

    RW_PRIMITIVE_NONE = 0,
    RW_PRIMITIVE_LINE_LIST = 1,
    RW_PRIMITIVE_POLY_LINE = 2,
    RW_PRIMITIVE_TRI_LIST = 3,
    RW_PRIMITIVE_TRI_STRIP = 4,
    RW_PRIMITIVE_TRI_FAN = 5,
    RW_PRIMITIVE_POINT_LIST = 6,

    RW_RENDER_TEXTURE_RASTER = 0,
    RW_RENDER_TEXTURE_ADDRESS = 1,
    RW_RENDER_TEXTURE_ADDRESS_U = 2,
    RW_RENDER_TEXTURE_ADDRESS_V = 3,
    RW_RENDER_TEXTURE_FILTER = 4,
    RW_RENDER_VERTEX_ALPHA = 5,
    RW_RENDER_SRC_BLEND = 6,
    RW_RENDER_DEST_BLEND = 7,
    RW_RENDER_Z_TEST_ENABLE = 8,
    RW_RENDER_Z_WRITE_ENABLE = 9,
    RW_RENDER_FOG_ENABLE = 10,
    RW_RENDER_FOG_COLOR = 11,
    RW_RENDER_CULL_MODE = 12,
    RW_RENDER_ALPHA_TEST_FUNC = 21,
    RW_RENDER_ALPHA_TEST_REF = 22,

    RW_ALPHA_ALWAYS = 0,
    RW_ALPHA_GREATER_EQUAL = 1,
    RW_ALPHA_LESS = 2,
    RW_CULL_NONE = 1,
    RW_CULL_BACK = 2,
    RW_CULL_FRONT = 3,
    RW_BLEND_ZERO = 1,
    RW_BLEND_ONE = 2,
    RW_BLEND_SRC_ALPHA = 5,
    RW_BLEND_INV_SRC_ALPHA = 6,

    RW_IM3D_VERTEX_UV = 1,
    RW_IM3D_ALL_OPAQUE = 2,
    RW_IM3D_NO_CLIP = 4,
    RW_IM3D_VERTEX_XYZ = 8,
    RW_IM3D_VERTEX_RGBA = 16,
    RW_CAMERA_PERSPECTIVE = 1,
    RW_CAMERA_PARALLEL = 2,

    RW_MATFX_NOTHING = 0,
    RW_MATFX_BUMP_MAP = 1,
    RW_MATFX_ENV_MAP = 2,
    RW_MATFX_BUMP_ENV_MAP = 3,
    RW_MATFX_DUAL = 4,
    RW_MATFX_UV_TRANSFORM = 5,
    RW_MATFX_DUAL_UV_TRANSFORM = 6
};

rw_bool32 librw_engine_init(void);
rw_bool32 librw_engine_open(void *params);
rw_bool32 librw_engine_start(void);
void librw_engine_stop(void);
void librw_engine_close(void);
void librw_engine_term(void);
int librw_get_platform(void);
void librw_set_platform(int platform);
int librw_get_version(void);
void librw_set_version(int version);

rw_Stream *librw_stream_memory_new(uint8_t *data, uint32_t len, uint32_t capacity);
void librw_stream_memory_delete(rw_Stream *stream);
uint8_t *librw_stream_memory_data(rw_Stream *stream);
uint32_t librw_stream_memory_len(rw_Stream *stream);
rw_Stream *librw_stream_file_new(const char *path, const char *mode);
void librw_stream_file_delete(rw_Stream *stream);
uint32_t librw_stream_read(rw_Stream *stream, void *data, uint32_t len);
uint32_t librw_stream_write(rw_Stream *stream, const void *data, uint32_t len);
void librw_stream_seek(rw_Stream *stream, int32_t offset, int whence);
uint32_t librw_stream_tell(rw_Stream *stream);
rw_bool32 librw_stream_eof(rw_Stream *stream);
rw_bool32 librw_stream_find_chunk(rw_Stream *stream, uint32_t kind, uint32_t *len, uint32_t *version);

rw_Frame *librw_frame_create(void);
void librw_frame_destroy(rw_Frame *frame);
void librw_frame_destroy_hierarchy(rw_Frame *frame);
void librw_frame_add_child(rw_Frame *parent, rw_Frame *child, rw_bool32 append);
rw_Frame *librw_frame_remove_child(rw_Frame *frame);
rw_Frame *librw_frame_parent(const rw_Frame *frame);
rw_Frame *librw_frame_child(const rw_Frame *frame);
rw_Frame *librw_frame_next(const rw_Frame *frame);
rw_Matrix *librw_frame_matrix(rw_Frame *frame);
const rw_Matrix *librw_frame_ltm(rw_Frame *frame);
void librw_frame_translate(rw_Frame *frame, const rw_V3d *value, int combine);
void librw_frame_scale(rw_Frame *frame, const rw_V3d *value, int combine);
void librw_frame_rotate(rw_Frame *frame, const rw_V3d *axis, float angle, int combine);
void librw_frame_update_objects(rw_Frame *frame);

rw_Image *librw_image_create(int32_t width, int32_t height, int32_t depth);
void librw_image_destroy(rw_Image *image);
void librw_image_allocate(rw_Image *image);
rw_Image *librw_image_read(const char *path);
int32_t librw_image_width(const rw_Image *image);
int32_t librw_image_height(const rw_Image *image);
int32_t librw_image_depth(const rw_Image *image);
int32_t librw_image_stride(const rw_Image *image);
uint8_t *librw_image_pixels(rw_Image *image);
uint8_t *librw_image_palette(rw_Image *image);

rw_Raster *librw_raster_create(int32_t width, int32_t height, int32_t depth, int32_t format, int32_t platform);
rw_Raster *librw_raster_from_image(rw_Image *image, int32_t platform);
rw_Image *librw_raster_to_image(rw_Raster *raster);
void librw_raster_destroy(rw_Raster *raster);
int32_t librw_raster_width(const rw_Raster *raster);
int32_t librw_raster_height(const rw_Raster *raster);
int32_t librw_raster_depth(const rw_Raster *raster);
int32_t librw_raster_stride(const rw_Raster *raster);
int32_t librw_raster_num_levels(rw_Raster *raster);
uint8_t *librw_raster_lock(rw_Raster *raster, int32_t level, int32_t mode);
void librw_raster_unlock(rw_Raster *raster, int32_t level);
int32_t librw_raster_type(const rw_Raster *raster);
rw_Raster *librw_raster_parent(const rw_Raster *raster);
void librw_raster_sub_raster(rw_Raster *raster, rw_Raster *parent,
    int32_t x, int32_t y, int32_t width, int32_t height);
rw_Raster *librw_raster_set_from_image(rw_Raster *raster, rw_Image *image);
void librw_raster_show(rw_Raster *raster, uint32_t flags);

rw_Texture *librw_texture_create(rw_Raster *raster);
void librw_texture_destroy(rw_Texture *texture);
const char *librw_texture_name(const rw_Texture *texture);
const char *librw_texture_mask(const rw_Texture *texture);
void librw_texture_set_name(rw_Texture *texture, const char *name);
void librw_texture_set_mask(rw_Texture *texture, const char *mask);
rw_Raster *librw_texture_raster(const rw_Texture *texture);
int librw_texture_filter(const rw_Texture *texture);
void librw_texture_set_filter(rw_Texture *texture, int filter);
uint32_t librw_texture_filter_addressing(const rw_Texture *texture);
void librw_texture_set_filter_addressing(rw_Texture *texture, uint32_t value);
/* Replaces the raster without destroying the old one: a texture destroyed
 * after set_raster(NULL) leaves its raster alive. */
void librw_texture_set_raster(rw_Texture *texture, rw_Raster *raster);

rw_Material *librw_material_create(void);
void librw_material_destroy(rw_Material *material);
void librw_material_set_texture(rw_Material *material, rw_Texture *texture);
rw_Texture *librw_material_texture(const rw_Material *material);
rw_RGBA *librw_material_color(rw_Material *material);
rw_SurfaceProperties *librw_material_surface_properties(rw_Material *material);

rw_Geometry *librw_geometry_create(int32_t vertices, int32_t triangles, uint32_t flags);
void librw_geometry_destroy(rw_Geometry *geometry);
void librw_geometry_lock(rw_Geometry *geometry, int32_t flags);
void librw_geometry_unlock(rw_Geometry *geometry);
int32_t librw_geometry_num_vertices(const rw_Geometry *geometry);
int32_t librw_geometry_num_triangles(const rw_Geometry *geometry);
int32_t librw_geometry_num_morph_targets(const rw_Geometry *geometry);
uint32_t librw_geometry_flags(const rw_Geometry *geometry);
void librw_geometry_set_flags(rw_Geometry *geometry, uint32_t flags);
int32_t librw_geometry_num_tex_coord_sets(const rw_Geometry *geometry);
void librw_geometry_build_meshes(rw_Geometry *geometry);
rw_V3d *librw_geometry_vertices(rw_Geometry *geometry, int32_t target);
rw_V3d *librw_geometry_normals(rw_Geometry *geometry, int32_t target);
rw_Triangle *librw_geometry_triangles(rw_Geometry *geometry);
rw_RGBA *librw_geometry_colors(rw_Geometry *geometry);
rw_TexCoords *librw_geometry_tex_coords(rw_Geometry *geometry, int32_t set);
int32_t librw_geometry_add_material(rw_Geometry *geometry, rw_Material *material);
int32_t librw_geometry_material_count(const rw_Geometry *geometry);
rw_Material *librw_geometry_material(const rw_Geometry *geometry, int32_t index);
void librw_geometry_calculate_bounding_sphere(rw_Geometry *geometry);

rw_Atomic *librw_atomic_create(void);
void librw_atomic_destroy(rw_Atomic *atomic);
void librw_atomic_set_geometry(rw_Atomic *atomic, rw_Geometry *geometry, uint32_t flags);
rw_Geometry *librw_atomic_geometry(const rw_Atomic *atomic);
void librw_atomic_set_frame(rw_Atomic *atomic, rw_Frame *frame);
rw_Frame *librw_atomic_frame(const rw_Atomic *atomic);
uint32_t librw_atomic_flags(const rw_Atomic *atomic);
void librw_atomic_set_flags(rw_Atomic *atomic, uint32_t flags);
void librw_atomic_render(rw_Atomic *atomic);
const rw_Sphere *librw_atomic_world_bounding_sphere(rw_Atomic *atomic);

rw_Clump *librw_clump_create(void);
void librw_clump_destroy(rw_Clump *clump);
rw_Clump *librw_clump_read(rw_Stream *stream);
rw_bool32 librw_clump_write(rw_Clump *clump, rw_Stream *stream);
void librw_clump_set_frame(rw_Clump *clump, rw_Frame *frame);
rw_Frame *librw_clump_frame(const rw_Clump *clump);
void librw_clump_add_atomic(rw_Clump *clump, rw_Atomic *atomic);
void librw_clump_remove_atomic(rw_Clump *clump, rw_Atomic *atomic);
int32_t librw_clump_atomic_count(const rw_Clump *clump);
rw_Atomic *librw_clump_atomic(const rw_Clump *clump, int32_t index);
void librw_clump_render(rw_Clump *clump);

rw_TexDictionary *librw_tex_dictionary_create(void);
void librw_tex_dictionary_destroy(rw_TexDictionary *dict);
rw_TexDictionary *librw_tex_dictionary_read(rw_Stream *stream);
void librw_tex_dictionary_write(rw_TexDictionary *dict, rw_Stream *stream);
void librw_tex_dictionary_add(rw_TexDictionary *dict, rw_Texture *texture);
void librw_tex_dictionary_remove(rw_TexDictionary *dict, rw_Texture *texture);
int32_t librw_tex_dictionary_count(const rw_TexDictionary *dict);
rw_Texture *librw_tex_dictionary_texture(const rw_TexDictionary *dict, int32_t index);
rw_Texture *librw_tex_dictionary_find(rw_TexDictionary *dict, const char *name);
void librw_tex_dictionary_set_current(rw_TexDictionary *dict);

rw_Light *librw_light_create(int32_t kind);
void librw_light_destroy(rw_Light *light);
void librw_light_set_frame(rw_Light *light, rw_Frame *frame);
void librw_light_set_color(rw_Light *light, float red, float green, float blue);
void librw_light_set_radius(rw_Light *light, float radius);
float librw_light_radius(const rw_Light *light);
const rw_RGBAf *librw_light_color(const rw_Light *light);
rw_Frame *librw_light_frame(const rw_Light *light);
rw_World *librw_light_world(const rw_Light *light);
void librw_light_set_type(rw_Light *light, int32_t kind);

rw_Camera *librw_camera_create(void);
void librw_camera_destroy(rw_Camera *camera);
void librw_camera_set_frame(rw_Camera *camera, rw_Frame *frame);
void librw_camera_set_frame_buffer(rw_Camera *camera, rw_Raster *raster);
void librw_camera_set_z_buffer(rw_Camera *camera, rw_Raster *raster);
void librw_camera_set_near_plane(rw_Camera *camera, float value);
void librw_camera_set_far_plane(rw_Camera *camera, float value);
void librw_camera_set_fov(rw_Camera *camera, float degrees, float ratio);
void librw_camera_clear(rw_Camera *camera, const rw_RGBA *color, uint32_t mode);
void librw_camera_begin_update(rw_Camera *camera);
void librw_camera_end_update(rw_Camera *camera);
rw_Frame *librw_camera_frame(const rw_Camera *camera);
rw_Raster *librw_camera_frame_buffer(const rw_Camera *camera);
rw_Raster *librw_camera_z_buffer(const rw_Camera *camera);
void librw_camera_set_fog_plane(rw_Camera *camera, float value);

rw_World *librw_world_create(const rw_BBox *bounds);
void librw_world_destroy(rw_World *world);
void librw_world_add_clump(rw_World *world, rw_Clump *clump);
void librw_world_remove_clump(rw_World *world, rw_Clump *clump);
void librw_world_add_light(rw_World *world, rw_Light *light);
void librw_world_remove_light(rw_World *world, rw_Light *light);
void librw_world_add_camera(rw_World *world, rw_Camera *camera);
void librw_world_remove_camera(rw_World *world, rw_Camera *camera);
void librw_world_render(rw_World *world);
typedef void (*rw_light_callback)(rw_Light *, void *);
/* Visits the world's global (ambient, directional) or local (point, spot) lights. */
void librw_world_for_each_light(rw_World *world, rw_bool32 local, rw_light_callback callback, void *user_data);

/* Rendering and port-facing extensions. */
void librw_set_render_state(int32_t state, uint32_t value);
void librw_set_render_state_ptr(int32_t state, void *value);
uint32_t librw_get_render_state(int32_t state);
void *librw_get_render_state_ptr(int32_t state);

rw_bool32 librw_raster_image_find_format(rw_Image *image, int32_t type,
    int32_t *width, int32_t *height, int32_t *depth, int32_t *format, int32_t platform);

void librw_camera_set_projection(rw_Camera *camera, int32_t projection);
void librw_camera_set_view_window(rw_Camera *camera, const rw_V2d *window);
void librw_camera_set_view_offset(rw_Camera *camera, const rw_V2d *offset);
void librw_camera_show_raster(rw_Camera *camera, uint32_t flags);

float librw_im2d_near_z(void);
float librw_im2d_far_z(void);
void librw_im2d_render_primitive(int32_t primitive, void *vertices, int32_t vertex_count);
void librw_im2d_render_indexed_primitive(int32_t primitive, void *vertices,
    int32_t vertex_count, void *indices, int32_t index_count);
void librw_im3d_transform(void *vertices, int32_t vertex_count,
    const rw_Matrix *world, uint32_t flags);
void librw_im3d_render_primitive(int32_t primitive);
void librw_im3d_render_indexed_primitive(int32_t primitive, void *indices, int32_t index_count);
void librw_im3d_end(void);

void librw_light_set_angle(rw_Light *light, float radians);
void librw_light_set_flags(rw_Light *light, uint32_t flags);
uint32_t librw_light_flags(const rw_Light *light);
int32_t librw_light_type(const rw_Light *light);

rw_WorldLights *librw_world_lights_new(int32_t directional_capacity, int32_t local_capacity);
void librw_world_lights_delete(rw_WorldLights *lights);
void librw_world_lights_clear(rw_WorldLights *lights);
/* Ambient lights add to the ambient colour, directional and local lights take
 * a slot; false when the light's list is full. */
rw_bool32 librw_world_lights_add(rw_WorldLights *lights, rw_Light *light);
void librw_world_enumerate_lights(rw_World *world, rw_Atomic *atomic, rw_WorldLights *lights);
int32_t librw_world_lights_num_ambient(const rw_WorldLights *lights);
int32_t librw_world_lights_num_directional(const rw_WorldLights *lights);
int32_t librw_world_lights_num_local(const rw_WorldLights *lights);
rw_Light *librw_world_lights_directional(const rw_WorldLights *lights, int32_t index);
rw_Light *librw_world_lights_local(const rw_WorldLights *lights, int32_t index);
rw_Light *librw_light_from_world_link(void *link);

rw_Sphere *librw_geometry_bounding_sphere(rw_Geometry *geometry, int32_t target);

void librw_register_native_data_plugin(void);
void librw_register_skin_plugin(void);
void librw_register_hanim_plugin(void);
void librw_register_matfx_plugin(void);
int32_t librw_raster_register_plugin(int32_t size, uint32_t id);
int32_t librw_texture_register_plugin(int32_t size, uint32_t id);

void *librw_skin_get(const rw_Geometry *geometry);
void librw_skin_set(rw_Geometry *geometry, void *skin);
void librw_skin_set_hierarchy(rw_Atomic *atomic, void *hierarchy);
void librw_skin_set_pipeline(rw_Atomic *atomic, int32_t type);
int32_t librw_skin_geometry_offset(void);
int32_t librw_skin_atomic_offset(void);
/* A zeroed skin with its bone and vertex arrays allocated (Skin::init); owned
 * by the geometry once passed to librw_skin_set. */
rw_Skin *librw_skin_create(int32_t bones, int32_t used_bones, int32_t vertices);
uint8_t *librw_skin_used_bones(rw_Skin *skin);
uint8_t *librw_skin_indices(rw_Skin *skin);
float *librw_skin_weights(rw_Skin *skin);
rw_Matrix *librw_skin_inverse_matrices(rw_Skin *skin);
void librw_skin_find_num_weights(rw_Skin *skin, int32_t vertices);

void *librw_hanim_hierarchy_create(int32_t node_count, int32_t *node_flags,
    int32_t *node_ids, int32_t flags, int32_t max_key_size);
void librw_hanim_hierarchy_destroy(void *hierarchy);
rw_Matrix *librw_hanim_hierarchy_matrices(void *hierarchy);
int32_t librw_hanim_hierarchy_node_count(const void *hierarchy);

uint32_t librw_matfx_effects(const rw_Material *material);
rw_Texture *librw_matfx_env_texture(const rw_Material *material);
float librw_matfx_env_coefficient(const rw_Material *material);
rw_Texture *librw_matfx_dual_texture(const rw_Material *material);
int32_t librw_matfx_dual_src_blend(const rw_Material *material);
int32_t librw_matfx_dual_dest_blend(const rw_Material *material);

#ifdef RW_GL3
typedef void (*rw_gl3_render_callback)(rw_Atomic *, rw_Gl3InstanceData *, void *);
enum {
    RW_GL3_UNIFORM_NA = 0,
    RW_GL3_UNIFORM_VEC4 = 1,
    RW_GL3_UNIFORM_IVEC4 = 2,
    RW_GL3_UNIFORM_MAT4 = 3
};
rw_bool32 librw_engine_open_gl3(void **window, rw_bool32 fullscreen,
    int32_t width, int32_t height, const char *title);
int32_t librw_gl3_set_lights(rw_WorldLights *lights);
int32_t librw_gl3_native_raster_offset(void);
uint32_t librw_gl3_raster_texture(const rw_Raster *raster);
uint32_t librw_gl3_raster_fbo(const rw_Raster *raster);
int32_t librw_gl3_raster_internal_format(const rw_Raster *raster);
rw_Raster *librw_gl3_raster_fbo_mate(const rw_Raster *raster);
int32_t librw_gl3_raster_format(const rw_Raster *raster);
int32_t librw_gl3_raster_bytes_per_pixel(const rw_Raster *raster);
uint32_t librw_gl3_instance_vbo(const rw_Gl3InstanceData *data);
uint32_t librw_gl3_instance_ibo(const rw_Gl3InstanceData *data);
uint32_t librw_gl3_instance_mesh_count(const rw_Gl3InstanceData *data);
uint32_t librw_gl3_instance_vertex_count(const rw_Gl3InstanceData *data);
uint32_t librw_gl3_instance_index_count(const rw_Gl3InstanceData *data);
int32_t librw_gl3_instance_attribute_count(const rw_Gl3InstanceData *data);
rw_bool32 librw_gl3_instance_attribute(const rw_Gl3InstanceData *data, int32_t index,
    uint32_t *location, int32_t *type, rw_bool32 *normalized, int32_t *size,
    uint32_t *stride, uint32_t *offset);
rw_bool32 librw_gl3_instance_mesh(const rw_Gl3InstanceData *data, int32_t index,
    uint32_t *index_count, uint32_t *min_vertex, int32_t *vertex_count,
    rw_Material **material, rw_bool32 *vertex_alpha, uint32_t *program, uint32_t *offset);
rw_Gl3Shader *librw_gl3_current_shader(void);
uint32_t librw_gl3_shader_program(const rw_Gl3Shader *shader);
void librw_gl3_shader_use(rw_Gl3Shader *shader);
void librw_gl3_set_uniform(int32_t id, void *data);
/* Before librw_engine_open: shaders only know the uniforms registered before them. */
int32_t librw_gl3_register_uniform(const char *name, int32_t type, int32_t count);
/* NULL-terminated source lists, compiled and linked; NULL on failure. */
rw_Gl3Shader *librw_gl3_shader_create(const char **vertex, const char **fragment);
void librw_gl3_shader_destroy(rw_Gl3Shader *shader);
/* The GLSL version line, and the vertex and fragment headers (uniforms,
 * DoDynamicLight, DoFog, DoAlphaTest) librw's own shaders start with. */
const char *librw_gl3_shader_decl(void);
const char *librw_gl3_shader_header_vs(void);
const char *librw_gl3_shader_header_fs(void);
/* For render callbacks: librw's per-object and per-mesh steps. */
void librw_gl3_set_world_matrix(const rw_Matrix *matrix);
void librw_gl3_setup_vertex_input(rw_Gl3InstanceData *data);
void librw_gl3_teardown_vertex_input(rw_Gl3InstanceData *data);
void librw_gl3_upload_skin_matrices(rw_Atomic *atomic);
void librw_gl3_set_material(uint32_t geometry_flags, const rw_RGBA *color,
    const rw_SurfaceProperties *surface);
void librw_gl3_set_texture(int32_t stage, rw_Texture *texture);
void librw_gl3_draw_mesh(rw_Gl3InstanceData *data, int32_t index);
rw_Gl3Pipeline *librw_gl3_pipeline_create(rw_gl3_render_callback callback, void *user_data);
/* A pipeline instancing as `base` does (its instance callbacks and plugin
 * id), rendering through `callback`. */
rw_Gl3Pipeline *librw_gl3_pipeline_create_like(rw_Gl3Pipeline *base,
    rw_gl3_render_callback callback, void *user_data);
void librw_gl3_pipeline_destroy(rw_Gl3Pipeline *pipeline);
rw_Gl3Pipeline *librw_gl3_skin_pipeline(void);
void librw_atomic_set_gl3_pipeline(rw_Atomic *atomic, rw_Gl3Pipeline *pipeline);
rw_Gl3Pipeline *librw_atomic_gl3_pipeline(const rw_Atomic *atomic);
#endif

#ifdef __cplusplus
}
#endif
#endif
