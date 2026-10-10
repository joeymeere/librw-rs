#include "librw_sys.h"

#include <cstring>
#include <new>

#include "rw.h"

namespace {
    template <typename To, typename From>
    To *cast(From *value) { return reinterpret_cast<To *>(value); }

    template <typename To, typename From>
    const To *cast(const From *value) { return reinterpret_cast<const To *>(value); }

    rw::Stream *stream(rw_Stream *value) { return cast<rw::Stream>(value); }
    rw::Frame *frame(rw_Frame *value) { return cast<rw::Frame>(value); }
    rw::Image *image(rw_Image *value) { return cast<rw::Image>(value); }
    rw::Raster *raster(rw_Raster *value) { return cast<rw::Raster>(value); }
    rw::Texture *texture(rw_Texture *value) { return cast<rw::Texture>(value); }
    rw::Material *material(rw_Material *value) { return cast<rw::Material>(value); }
    rw::Geometry *geometry(rw_Geometry *value) { return cast<rw::Geometry>(value); }
    rw::Atomic *atomic(rw_Atomic *value) { return cast<rw::Atomic>(value); }
    rw::Light *light(rw_Light *value) { return cast<rw::Light>(value); }
    rw::Camera *camera(rw_Camera *value) { return cast<rw::Camera>(value); }
    rw::Clump *clump(rw_Clump *value) { return cast<rw::Clump>(value); }
    rw::World *world(rw_World *value) { return cast<rw::World>(value); }
    rw::TexDictionary *dict(rw_TexDictionary *value) { return cast<rw::TexDictionary>(value); }
} // namespace

extern "C" {
    rw_bool32 librw_engine_init(void) { return rw::Engine::init(); }
    rw_bool32 librw_engine_open(void *params) { return rw::Engine::open(cast<rw::EngineOpenParams>(params)); }
    rw_bool32 librw_engine_start(void) { return rw::Engine::start(); }
    void librw_engine_stop(void) { rw::Engine::stop(); }
    void librw_engine_close(void) { rw::Engine::close(); }
    void librw_engine_term(void) { rw::Engine::term(); }
    int librw_get_platform(void) { return rw::platform; }
    void librw_set_platform(int platform) { rw::platform = platform; }
    int librw_get_version(void) { return rw::version; }
    void librw_set_version(int version) { rw::version = version; }

    rw_Stream *librw_stream_memory_new(uint8_t *data, uint32_t len, uint32_t capacity) {
        rw::StreamMemory *result = new (std::nothrow) rw::StreamMemory;
        if (result == NULL) return NULL;
        if (result->open(data, len, capacity) == NULL) { delete result; return NULL; }
        return cast<rw_Stream>(result);
    }
    void librw_stream_memory_delete(rw_Stream *value) { delete cast<rw::StreamMemory>(value); }
    uint8_t *librw_stream_memory_data(rw_Stream *value) { return cast<rw::StreamMemory>(value)->data; }
    uint32_t librw_stream_memory_len(rw_Stream *value) { return cast<rw::StreamMemory>(value)->getLength(); }
    rw_Stream *librw_stream_file_new(const char *path, const char *mode) {
        rw::StreamFile *result = new (std::nothrow) rw::StreamFile;
        if (result == NULL) return NULL;
        if (result->open(path, mode) == NULL) { delete result; return NULL; }
        return cast<rw_Stream>(result);
    }
    void librw_stream_file_delete(rw_Stream *value) {
        rw::StreamFile *file = cast<rw::StreamFile>(value);
        file->close();
        delete file;
    }
    uint32_t librw_stream_read(rw_Stream *value, void *data, uint32_t len) { return stream(value)->read8(data, len); }
    uint32_t librw_stream_write(rw_Stream *value, const void *data, uint32_t len) { return stream(value)->write8(data, len); }
    void librw_stream_seek(rw_Stream *value, int32_t offset, int whence) { stream(value)->seek(offset, whence); }
    uint32_t librw_stream_tell(rw_Stream *value) { return stream(value)->tell(); }
    rw_bool32 librw_stream_eof(rw_Stream *value) { return stream(value)->eof(); }
    rw_bool32 librw_stream_find_chunk(rw_Stream *value, uint32_t kind, uint32_t *len, uint32_t *version) {
        return rw::findChunk(stream(value), kind, len, version);
    }

    rw_Frame *librw_frame_create(void) { return cast<rw_Frame>(rw::Frame::create()); }
    void librw_frame_destroy(rw_Frame *value) { frame(value)->destroy(); }
    void librw_frame_destroy_hierarchy(rw_Frame *value) { frame(value)->destroyHierarchy(); }
    void librw_frame_add_child(rw_Frame *parent, rw_Frame *child, rw_bool32 append) { frame(parent)->addChild(frame(child), append); }
    rw_Frame *librw_frame_remove_child(rw_Frame *value) { return cast<rw_Frame>(frame(value)->removeChild()); }
    rw_Frame *librw_frame_parent(const rw_Frame *value) { return cast<rw_Frame>(cast<rw::Frame>(value)->getParent()); }
    rw_Frame *librw_frame_child(const rw_Frame *value) { return cast<rw_Frame>(cast<rw::Frame>(value)->child); }
    rw_Frame *librw_frame_next(const rw_Frame *value) { return cast<rw_Frame>(cast<rw::Frame>(value)->next); }
    rw_Matrix *librw_frame_matrix(rw_Frame *value) { return cast<rw_Matrix>(&frame(value)->matrix); }
    const rw_Matrix *librw_frame_ltm(rw_Frame *value) { return cast<rw_Matrix>(frame(value)->getLTM()); }
    void librw_frame_translate(rw_Frame *value, const rw_V3d *v, int combine) {
        frame(value)->translate(cast<rw::V3d>(v), static_cast<rw::CombineOp>(combine));
    }
    void librw_frame_scale(rw_Frame *value, const rw_V3d *v, int combine) {
        frame(value)->scale(cast<rw::V3d>(v), static_cast<rw::CombineOp>(combine));
    }
    void librw_frame_rotate(rw_Frame *value, const rw_V3d *axis, float angle, int combine) {
        frame(value)->rotate(cast<rw::V3d>(axis), angle, static_cast<rw::CombineOp>(combine));
    }

    rw_Image *librw_image_create(int32_t width, int32_t height, int32_t depth) { return cast<rw_Image>(rw::Image::create(width, height, depth)); }
    void librw_image_destroy(rw_Image *value) { image(value)->destroy(); }
    void librw_image_allocate(rw_Image *value) { image(value)->allocate(); }
    rw_Image *librw_image_read(const char *path) { return cast<rw_Image>(rw::Image::read(path)); }
    int32_t librw_image_width(const rw_Image *value) { return cast<rw::Image>(value)->width; }
    int32_t librw_image_height(const rw_Image *value) { return cast<rw::Image>(value)->height; }
    int32_t librw_image_depth(const rw_Image *value) { return cast<rw::Image>(value)->depth; }
    int32_t librw_image_stride(const rw_Image *value) { return cast<rw::Image>(value)->stride; }
    uint8_t *librw_image_pixels(rw_Image *value) { return image(value)->pixels; }
    uint8_t *librw_image_palette(rw_Image *value) { return image(value)->palette; }

    rw_Raster *librw_raster_create(int32_t width, int32_t height, int32_t depth, int32_t format, int32_t platform) {
        return cast<rw_Raster>(rw::Raster::create(width, height, depth, format, platform));
    }
    rw_Raster *librw_raster_from_image(rw_Image *value, int32_t platform) { return cast<rw_Raster>(rw::Raster::createFromImage(image(value), platform)); }
    rw_Image *librw_raster_to_image(rw_Raster *value) { return cast<rw_Image>(raster(value)->toImage()); }
    void librw_raster_destroy(rw_Raster *value) { raster(value)->destroy(); }
    int32_t librw_raster_width(const rw_Raster *value) { return cast<rw::Raster>(value)->width; }
    int32_t librw_raster_height(const rw_Raster *value) { return cast<rw::Raster>(value)->height; }
    int32_t librw_raster_depth(const rw_Raster *value) { return cast<rw::Raster>(value)->depth; }
    int32_t librw_raster_stride(const rw_Raster *value) { return cast<rw::Raster>(value)->stride; }
    int32_t librw_raster_num_levels(rw_Raster *value) { return raster(value)->getNumLevels(); }
    uint8_t *librw_raster_lock(rw_Raster *value, int32_t level, int32_t mode) { return raster(value)->lock(level, mode); }
    void librw_raster_unlock(rw_Raster *value, int32_t level) { raster(value)->unlock(level); }

    rw_Texture *librw_texture_create(rw_Raster *value) { return cast<rw_Texture>(rw::Texture::create(raster(value))); }
    void librw_texture_destroy(rw_Texture *value) { texture(value)->destroy(); }
    const char *librw_texture_name(const rw_Texture *value) { return cast<rw::Texture>(value)->name; }
    const char *librw_texture_mask(const rw_Texture *value) { return cast<rw::Texture>(value)->mask; }
    void librw_texture_set_name(rw_Texture *value, const char *name) {
        std::strncpy(texture(value)->name, name, sizeof(texture(value)->name));
        texture(value)->name[sizeof(texture(value)->name) - 1] = '\0';
    }
    void librw_texture_set_mask(rw_Texture *value, const char *mask) {
        std::strncpy(texture(value)->mask, mask, sizeof(texture(value)->mask));
        texture(value)->mask[sizeof(texture(value)->mask) - 1] = '\0';
    }
    rw_Raster *librw_texture_raster(const rw_Texture *value) { return cast<rw_Raster>(cast<rw::Texture>(value)->raster); }
    int librw_texture_filter(const rw_Texture *value) { return const_cast<rw::Texture *>(cast<rw::Texture>(value))->getFilter(); }
    void librw_texture_set_filter(rw_Texture *value, int filter) { texture(value)->setFilter(static_cast<rw::Texture::FilterMode>(filter)); }

    rw_Material *librw_material_create(void) { return cast<rw_Material>(rw::Material::create()); }
    void librw_material_destroy(rw_Material *value) { material(value)->destroy(); }
    void librw_material_set_texture(rw_Material *value, rw_Texture *tex) { material(value)->setTexture(tex ? texture(tex) : NULL); }
    rw_Texture *librw_material_texture(const rw_Material *value) { return cast<rw_Texture>(cast<rw::Material>(value)->texture); }
    rw_RGBA *librw_material_color(rw_Material *value) { return cast<rw_RGBA>(&material(value)->color); }
    rw_SurfaceProperties *librw_material_surface_properties(rw_Material *value) { return cast<rw_SurfaceProperties>(&material(value)->surfaceProps); }

    rw_Geometry *librw_geometry_create(int32_t vertices, int32_t triangles, uint32_t flags) {
        return cast<rw_Geometry>(rw::Geometry::create(vertices, triangles, flags));
    }
    void librw_geometry_destroy(rw_Geometry *value) { geometry(value)->destroy(); }
    void librw_geometry_lock(rw_Geometry *value, int32_t flags) { geometry(value)->lock(flags); }
    void librw_geometry_unlock(rw_Geometry *value) { geometry(value)->unlock(); }
    int32_t librw_geometry_num_vertices(const rw_Geometry *value) { return cast<rw::Geometry>(value)->numVertices; }
    int32_t librw_geometry_num_triangles(const rw_Geometry *value) { return cast<rw::Geometry>(value)->numTriangles; }
    int32_t librw_geometry_num_morph_targets(const rw_Geometry *value) { return cast<rw::Geometry>(value)->numMorphTargets; }
    uint32_t librw_geometry_flags(const rw_Geometry *value) { return cast<rw::Geometry>(value)->flags; }
    rw_V3d *librw_geometry_vertices(rw_Geometry *value, int32_t target) {
        rw::Geometry *g = geometry(value);
        return target >= 0 && target < g->numMorphTargets ? cast<rw_V3d>(g->morphTargets[target].vertices) : NULL;
    }
    rw_V3d *librw_geometry_normals(rw_Geometry *value, int32_t target) {
        rw::Geometry *g = geometry(value);
        return target >= 0 && target < g->numMorphTargets ? cast<rw_V3d>(g->morphTargets[target].normals) : NULL;
    }
    rw_Triangle *librw_geometry_triangles(rw_Geometry *value) { return cast<rw_Triangle>(geometry(value)->triangles); }
    rw_RGBA *librw_geometry_colors(rw_Geometry *value) { return cast<rw_RGBA>(geometry(value)->colors); }
    rw_TexCoords *librw_geometry_tex_coords(rw_Geometry *value, int32_t set) {
        return set >= 0 && set < geometry(value)->numTexCoordSets ? cast<rw_TexCoords>(geometry(value)->texCoords[set]) : NULL;
    }
    int32_t librw_geometry_add_material(rw_Geometry *value, rw_Material *mat) { return geometry(value)->matList.appendMaterial(material(mat)); }
    int32_t librw_geometry_material_count(const rw_Geometry *value) { return cast<rw::Geometry>(value)->matList.numMaterials; }
    rw_Material *librw_geometry_material(const rw_Geometry *value, int32_t index) {
        const rw::Geometry *g = cast<rw::Geometry>(value);
        return index >= 0 && index < g->matList.numMaterials ? cast<rw_Material>(g->matList.materials[index]) : NULL;
    }
    void librw_geometry_calculate_bounding_sphere(rw_Geometry *value) { geometry(value)->calculateBoundingSphere(); }

    rw_Atomic *librw_atomic_create(void) { return cast<rw_Atomic>(rw::Atomic::create()); }
    void librw_atomic_destroy(rw_Atomic *value) { atomic(value)->destroy(); }
    void librw_atomic_set_geometry(rw_Atomic *value, rw_Geometry *geo, uint32_t flags) { atomic(value)->setGeometry(geo ? geometry(geo) : NULL, flags); }
    rw_Geometry *librw_atomic_geometry(const rw_Atomic *value) { return cast<rw_Geometry>(cast<rw::Atomic>(value)->geometry); }
    void librw_atomic_set_frame(rw_Atomic *value, rw_Frame *frm) { atomic(value)->setFrame(frm ? frame(frm) : NULL); }
    rw_Frame *librw_atomic_frame(const rw_Atomic *value) { return cast<rw_Frame>(cast<rw::Atomic>(value)->getFrame()); }
    uint32_t librw_atomic_flags(const rw_Atomic *value) { return cast<rw::Atomic>(value)->getFlags(); }
    void librw_atomic_set_flags(rw_Atomic *value, uint32_t flags) { atomic(value)->setFlags(flags); }
    void librw_atomic_render(rw_Atomic *value) { atomic(value)->render(); }

    rw_Clump *librw_clump_create(void) { return cast<rw_Clump>(rw::Clump::create()); }
    void librw_clump_destroy(rw_Clump *value) { clump(value)->destroy(); }
    rw_Clump *librw_clump_read(rw_Stream *value) { return cast<rw_Clump>(rw::Clump::streamRead(stream(value))); }
    rw_bool32 librw_clump_write(rw_Clump *value, rw_Stream *out) { return clump(value)->streamWrite(stream(out)); }
    void librw_clump_set_frame(rw_Clump *value, rw_Frame *frm) { clump(value)->setFrame(frm ? frame(frm) : NULL); }
    rw_Frame *librw_clump_frame(const rw_Clump *value) { return cast<rw_Frame>(cast<rw::Clump>(value)->getFrame()); }
    void librw_clump_add_atomic(rw_Clump *value, rw_Atomic *a) { clump(value)->addAtomic(atomic(a)); }
    void librw_clump_remove_atomic(rw_Clump *value, rw_Atomic *a) { clump(value)->removeAtomic(atomic(a)); }
    int32_t librw_clump_atomic_count(const rw_Clump *value) { return const_cast<rw::Clump *>(cast<rw::Clump>(value))->countAtomics(); }
    rw_Atomic *librw_clump_atomic(const rw_Clump *value, int32_t index) {
        if (index < 0) return NULL;
        int32_t current = 0;
        rw::Clump *mutable_value = const_cast<rw::Clump *>(cast<rw::Clump>(value));
        FORLIST(link, mutable_value->atomics) {
            if (current++ == index) return cast<rw_Atomic>(rw::Atomic::fromClump(link));
        }
        return NULL;
    }
    void librw_clump_render(rw_Clump *value) { clump(value)->render(); }

    rw_TexDictionary *librw_tex_dictionary_create(void) { return cast<rw_TexDictionary>(rw::TexDictionary::create()); }
    void librw_tex_dictionary_destroy(rw_TexDictionary *value) { dict(value)->destroy(); }
    rw_TexDictionary *librw_tex_dictionary_read(rw_Stream *value) { return cast<rw_TexDictionary>(rw::TexDictionary::streamRead(stream(value))); }
    void librw_tex_dictionary_write(rw_TexDictionary *value, rw_Stream *out) { dict(value)->streamWrite(stream(out)); }
    void librw_tex_dictionary_add(rw_TexDictionary *value, rw_Texture *tex) { dict(value)->add(texture(tex)); }
    void librw_tex_dictionary_remove(rw_TexDictionary *value, rw_Texture *tex) { dict(value)->remove(texture(tex)); }
    int32_t librw_tex_dictionary_count(const rw_TexDictionary *value) { return const_cast<rw::TexDictionary *>(cast<rw::TexDictionary>(value))->count(); }
    rw_Texture *librw_tex_dictionary_texture(const rw_TexDictionary *value, int32_t index) {
        if (index < 0) return NULL;
        int32_t current = 0;
        rw::TexDictionary *mutable_value = const_cast<rw::TexDictionary *>(cast<rw::TexDictionary>(value));
        FORLIST(link, mutable_value->textures) {
            if (current++ == index) return cast<rw_Texture>(rw::Texture::fromDict(link));
        }
        return NULL;
    }
    rw_Texture *librw_tex_dictionary_find(rw_TexDictionary *value, const char *name) { return cast<rw_Texture>(dict(value)->find(name)); }
    void librw_tex_dictionary_set_current(rw_TexDictionary *value) { rw::TexDictionary::setCurrent(value ? dict(value) : NULL); }

    rw_Light *librw_light_create(int32_t kind) { return cast<rw_Light>(rw::Light::create(kind)); }
    void librw_light_destroy(rw_Light *value) { light(value)->destroy(); }
    void librw_light_set_frame(rw_Light *value, rw_Frame *frm) { light(value)->setFrame(frm ? frame(frm) : NULL); }
    void librw_light_set_color(rw_Light *value, float red, float green, float blue) { light(value)->setColor(red, green, blue); }
    void librw_light_set_radius(rw_Light *value, float radius) { light(value)->radius = radius; }

    rw_Camera *librw_camera_create(void) { return cast<rw_Camera>(rw::Camera::create()); }
    void librw_camera_destroy(rw_Camera *value) { camera(value)->destroy(); }
    void librw_camera_set_frame(rw_Camera *value, rw_Frame *frm) { camera(value)->setFrame(frm ? frame(frm) : NULL); }
    void librw_camera_set_frame_buffer(rw_Camera *value, rw_Raster *buffer) { camera(value)->frameBuffer = buffer ? raster(buffer) : NULL; }
    void librw_camera_set_z_buffer(rw_Camera *value, rw_Raster *buffer) { camera(value)->zBuffer = buffer ? raster(buffer) : NULL; }
    void librw_camera_set_near_plane(rw_Camera *value, float v) { camera(value)->setNearPlane(v); }
    void librw_camera_set_far_plane(rw_Camera *value, float v) { camera(value)->setFarPlane(v); }
    void librw_camera_set_fov(rw_Camera *value, float degrees, float ratio) { camera(value)->setFOV(degrees, ratio); }
    void librw_camera_clear(rw_Camera *value, const rw_RGBA *color, uint32_t mode) { camera(value)->clear(const_cast<rw::RGBA *>(cast<rw::RGBA>(color)), mode); }
    void librw_camera_begin_update(rw_Camera *value) { camera(value)->beginUpdate(); }
    void librw_camera_end_update(rw_Camera *value) { camera(value)->endUpdate(); }

    rw_World *librw_world_create(const rw_BBox *bounds) { return cast<rw_World>(rw::World::create(const_cast<rw::BBox *>(cast<rw::BBox>(bounds)))); }
    void librw_world_destroy(rw_World *value) { world(value)->destroy(); }
    void librw_world_add_clump(rw_World *value, rw_Clump *object) { world(value)->addClump(clump(object)); }
    void librw_world_remove_clump(rw_World *value, rw_Clump *object) { world(value)->removeClump(clump(object)); }
    void librw_world_add_light(rw_World *value, rw_Light *object) { world(value)->addLight(light(object)); }
    void librw_world_remove_light(rw_World *value, rw_Light *object) { world(value)->removeLight(light(object)); }
    void librw_world_add_camera(rw_World *value, rw_Camera *object) { world(value)->addCamera(camera(object)); }
    void librw_world_remove_camera(rw_World *value, rw_Camera *object) { world(value)->removeCamera(camera(object)); }
    void librw_world_render(rw_World *value) { world(value)->render(); }
} // extern
