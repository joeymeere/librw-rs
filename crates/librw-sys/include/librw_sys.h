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

rw_Texture *librw_texture_create(rw_Raster *raster);
void librw_texture_destroy(rw_Texture *texture);
const char *librw_texture_name(const rw_Texture *texture);
const char *librw_texture_mask(const rw_Texture *texture);
void librw_texture_set_name(rw_Texture *texture, const char *name);
void librw_texture_set_mask(rw_Texture *texture, const char *mask);
rw_Raster *librw_texture_raster(const rw_Texture *texture);
int librw_texture_filter(const rw_Texture *texture);
void librw_texture_set_filter(rw_Texture *texture, int filter);

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

rw_World *librw_world_create(const rw_BBox *bounds);
void librw_world_destroy(rw_World *world);
void librw_world_add_clump(rw_World *world, rw_Clump *clump);
void librw_world_remove_clump(rw_World *world, rw_Clump *clump);
void librw_world_add_light(rw_World *world, rw_Light *light);
void librw_world_remove_light(rw_World *world, rw_Light *light);
void librw_world_add_camera(rw_World *world, rw_Camera *camera);
void librw_world_remove_camera(rw_World *world, rw_Camera *camera);
void librw_world_render(rw_World *world);

#ifdef __cplusplus
}
#endif
#endif
