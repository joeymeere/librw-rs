#include "librw_sys.h"

#include <cstddef>
#include <cstring>
#include <new>

#include "rw.h"

#define CHECK_SIZE(c_type, cpp_type) static_assert(sizeof(c_type) == sizeof(cpp_type), "layout mismatch: " #c_type)
#define CHECK_FIELD(c_type, c_field, cpp_type, cpp_field) \
    static_assert(offsetof(c_type, c_field) == offsetof(cpp_type, cpp_field), "field mismatch: " #c_type "." #c_field)

CHECK_SIZE(rw_RGBA, rw::RGBA);
CHECK_FIELD(rw_RGBA, red, rw::RGBA, red);
CHECK_FIELD(rw_RGBA, green, rw::RGBA, green);
CHECK_FIELD(rw_RGBA, blue, rw::RGBA, blue);
CHECK_FIELD(rw_RGBA, alpha, rw::RGBA, alpha);
CHECK_SIZE(rw_RGBAf, rw::RGBAf);
CHECK_FIELD(rw_RGBAf, red, rw::RGBAf, red);
CHECK_FIELD(rw_RGBAf, green, rw::RGBAf, green);
CHECK_FIELD(rw_RGBAf, blue, rw::RGBAf, blue);
CHECK_FIELD(rw_RGBAf, alpha, rw::RGBAf, alpha);
CHECK_SIZE(rw_V2d, rw::V2d);
CHECK_FIELD(rw_V2d, x, rw::V2d, x);
CHECK_FIELD(rw_V2d, y, rw::V2d, y);
CHECK_SIZE(rw_V3d, rw::V3d);
CHECK_FIELD(rw_V3d, x, rw::V3d, x);
CHECK_FIELD(rw_V3d, y, rw::V3d, y);
CHECK_FIELD(rw_V3d, z, rw::V3d, z);
CHECK_SIZE(rw_TexCoords, rw::TexCoords);
CHECK_FIELD(rw_TexCoords, u, rw::TexCoords, u);
CHECK_FIELD(rw_TexCoords, v, rw::TexCoords, v);
CHECK_SIZE(rw_Sphere, rw::Sphere);
CHECK_FIELD(rw_Sphere, center, rw::Sphere, center);
CHECK_FIELD(rw_Sphere, radius, rw::Sphere, radius);
CHECK_SIZE(rw_BBox, rw::BBox);
CHECK_FIELD(rw_BBox, sup, rw::BBox, sup);
CHECK_FIELD(rw_BBox, inf, rw::BBox, inf);
CHECK_SIZE(rw_Matrix, rw::Matrix);
CHECK_FIELD(rw_Matrix, right, rw::Matrix, right);
CHECK_FIELD(rw_Matrix, flags, rw::Matrix, flags);
CHECK_FIELD(rw_Matrix, up, rw::Matrix, up);
CHECK_FIELD(rw_Matrix, pad1, rw::Matrix, pad1);
CHECK_FIELD(rw_Matrix, at, rw::Matrix, at);
CHECK_FIELD(rw_Matrix, pad2, rw::Matrix, pad2);
CHECK_FIELD(rw_Matrix, pos, rw::Matrix, pos);
CHECK_FIELD(rw_Matrix, pad3, rw::Matrix, pad3);
CHECK_SIZE(rw_Triangle, rw::Triangle);
CHECK_FIELD(rw_Triangle, v, rw::Triangle, v);
CHECK_FIELD(rw_Triangle, mat_id, rw::Triangle, matId);
CHECK_SIZE(rw_SurfaceProperties, rw::SurfaceProperties);
CHECK_FIELD(rw_SurfaceProperties, ambient, rw::SurfaceProperties, ambient);
CHECK_FIELD(rw_SurfaceProperties, specular, rw::SurfaceProperties, specular);
CHECK_FIELD(rw_SurfaceProperties, diffuse, rw::SurfaceProperties, diffuse);
#ifdef RW_GL3
CHECK_SIZE(rw_Im2DVertex, rw::gl3::Im2DVertex);
CHECK_FIELD(rw_Im2DVertex, x, rw::gl3::Im2DVertex, x);
CHECK_FIELD(rw_Im2DVertex, red, rw::gl3::Im2DVertex, r);
CHECK_FIELD(rw_Im2DVertex, u, rw::gl3::Im2DVertex, u);
CHECK_SIZE(rw_Im3DVertex, rw::gl3::Im3DVertex);
CHECK_FIELD(rw_Im3DVertex, position, rw::gl3::Im3DVertex, position);
CHECK_FIELD(rw_Im3DVertex, normal, rw::gl3::Im3DVertex, normal);
CHECK_FIELD(rw_Im3DVertex, red, rw::gl3::Im3DVertex, r);
CHECK_FIELD(rw_Im3DVertex, u, rw::gl3::Im3DVertex, u);
#endif

#define CHECK_VALUE(c_value, cpp_value) static_assert((int64_t)(c_value) == (int64_t)(cpp_value), "value mismatch: " #c_value)
CHECK_VALUE(RW_COMBINE_PRECONCAT, rw::COMBINEPRECONCAT);
CHECK_VALUE(RW_COMBINE_POSTCONCAT, rw::COMBINEPOSTCONCAT);
CHECK_VALUE(RW_ID_SKIN, rw::ID_SKIN);
CHECK_VALUE(RW_GEOMETRY_TRISTRIP, rw::Geometry::TRISTRIP);
CHECK_VALUE(RW_GEOMETRY_POSITIONS, rw::Geometry::POSITIONS);
CHECK_VALUE(RW_GEOMETRY_TEXTURED, rw::Geometry::TEXTURED);
CHECK_VALUE(RW_GEOMETRY_PRELIT, rw::Geometry::PRELIT);
CHECK_VALUE(RW_GEOMETRY_NORMALS, rw::Geometry::NORMALS);
CHECK_VALUE(RW_GEOMETRY_LIGHT, rw::Geometry::LIGHT);
CHECK_VALUE(RW_GEOMETRY_MODULATE, rw::Geometry::MODULATE);
CHECK_VALUE(RW_GEOMETRY_TEXTURED2, rw::Geometry::TEXTURED2);
CHECK_VALUE(RW_GEOMETRY_NATIVE, rw::Geometry::NATIVE);
CHECK_VALUE(RW_GEOMETRY_LOCK_VERTICES, rw::Geometry::LOCKVERTICES);
CHECK_VALUE(RW_GEOMETRY_LOCK_NORMALS, rw::Geometry::LOCKNORMALS);
CHECK_VALUE(RW_GEOMETRY_LOCK_ALL, rw::Geometry::LOCKALL);
CHECK_VALUE(RW_ATOMIC_COLLISION_TEST, rw::Atomic::COLLISIONTEST);
CHECK_VALUE(RW_ATOMIC_RENDER, rw::Atomic::RENDER);
CHECK_VALUE(RW_RASTER_ZBUFFER, rw::Raster::ZBUFFER);
CHECK_VALUE(RW_RASTER_CAMERA, rw::Raster::CAMERA);
CHECK_VALUE(RW_RASTER_TEXTURE, rw::Raster::TEXTURE);
CHECK_VALUE(RW_RASTER_CAMERA_TEXTURE, rw::Raster::CAMERATEXTURE);
CHECK_VALUE(RW_RASTER_DONT_ALLOCATE, rw::Raster::DONTALLOCATE);
CHECK_VALUE(RW_RASTER_LOCK_WRITE, rw::Raster::LOCKWRITE);
CHECK_VALUE(RW_RASTER_LOCK_READ, rw::Raster::LOCKREAD);
CHECK_VALUE(RW_RASTER_LOCK_NO_FETCH, rw::Raster::LOCKNOFETCH);
CHECK_VALUE(RW_RASTER_FLIP_WAIT_VSYNC, rw::Raster::FLIPWAITVSYNCH);
CHECK_VALUE(RW_RASTER_FORMAT_C8888, rw::Raster::C8888);
CHECK_VALUE(RW_RASTER_FORMAT_AUTO_MIPMAP, rw::Raster::AUTOMIPMAP);
CHECK_VALUE(RW_RASTER_FORMAT_MIPMAP, rw::Raster::MIPMAP);
CHECK_VALUE(RW_TEXTURE_FILTER_NEAREST, rw::Texture::NEAREST);
CHECK_VALUE(RW_TEXTURE_FILTER_LINEAR, rw::Texture::LINEAR);
CHECK_VALUE(RW_TEXTURE_FILTER_LINEAR_MIP_LINEAR, rw::Texture::LINEARMIPLINEAR);
CHECK_VALUE(RW_LIGHT_DIRECTIONAL, rw::Light::DIRECTIONAL);
CHECK_VALUE(RW_LIGHT_AMBIENT, rw::Light::AMBIENT);
CHECK_VALUE(RW_LIGHT_POINT, rw::Light::POINT);
CHECK_VALUE(RW_LIGHT_SPOT, rw::Light::SPOT);
CHECK_VALUE(RW_LIGHT_SOFT_SPOT, rw::Light::SOFTSPOT);
CHECK_VALUE(RW_LIGHT_ATOMICS, rw::Light::LIGHTATOMICS);
CHECK_VALUE(RW_LIGHT_WORLD, rw::Light::LIGHTWORLD);
CHECK_VALUE(RW_PLATFORM_GL3, rw::PLATFORM_GL3);
CHECK_VALUE(RW_MEMDUR_EVENT, rw::MEMDUR_EVENT);
CHECK_VALUE(RW_PRIMITIVE_LINE_LIST, rw::PRIMTYPELINELIST);
CHECK_VALUE(RW_PRIMITIVE_POLY_LINE, rw::PRIMTYPEPOLYLINE);
CHECK_VALUE(RW_PRIMITIVE_TRI_LIST, rw::PRIMTYPETRILIST);
CHECK_VALUE(RW_PRIMITIVE_TRI_STRIP, rw::PRIMTYPETRISTRIP);
CHECK_VALUE(RW_PRIMITIVE_TRI_FAN, rw::PRIMTYPETRIFAN);
CHECK_VALUE(RW_PRIMITIVE_POINT_LIST, rw::PRIMTYPEPOINTLIST);
CHECK_VALUE(RW_RENDER_TEXTURE_RASTER, rw::TEXTURERASTER);
CHECK_VALUE(RW_RENDER_TEXTURE_ADDRESS, rw::TEXTUREADDRESS);
CHECK_VALUE(RW_RENDER_TEXTURE_ADDRESS_U, rw::TEXTUREADDRESSU);
CHECK_VALUE(RW_RENDER_TEXTURE_ADDRESS_V, rw::TEXTUREADDRESSV);
CHECK_VALUE(RW_RENDER_TEXTURE_FILTER, rw::TEXTUREFILTER);
CHECK_VALUE(RW_RENDER_VERTEX_ALPHA, rw::VERTEXALPHA);
CHECK_VALUE(RW_RENDER_SRC_BLEND, rw::SRCBLEND);
CHECK_VALUE(RW_RENDER_DEST_BLEND, rw::DESTBLEND);
CHECK_VALUE(RW_RENDER_Z_TEST_ENABLE, rw::ZTESTENABLE);
CHECK_VALUE(RW_RENDER_Z_WRITE_ENABLE, rw::ZWRITEENABLE);
CHECK_VALUE(RW_RENDER_FOG_ENABLE, rw::FOGENABLE);
CHECK_VALUE(RW_RENDER_FOG_COLOR, rw::FOGCOLOR);
CHECK_VALUE(RW_RENDER_CULL_MODE, rw::CULLMODE);
CHECK_VALUE(RW_RENDER_ALPHA_TEST_FUNC, rw::ALPHATESTFUNC);
CHECK_VALUE(RW_RENDER_ALPHA_TEST_REF, rw::ALPHATESTREF);
CHECK_VALUE(RW_ALPHA_ALWAYS, rw::ALPHAALWAYS);
CHECK_VALUE(RW_ALPHA_GREATER_EQUAL, rw::ALPHAGREATEREQUAL);
CHECK_VALUE(RW_ALPHA_LESS, rw::ALPHALESS);
CHECK_VALUE(RW_CULL_NONE, rw::CULLNONE);
CHECK_VALUE(RW_CULL_BACK, rw::CULLBACK);
CHECK_VALUE(RW_CULL_FRONT, rw::CULLFRONT);
CHECK_VALUE(RW_BLEND_ZERO, rw::BLENDZERO);
CHECK_VALUE(RW_BLEND_ONE, rw::BLENDONE);
CHECK_VALUE(RW_BLEND_SRC_ALPHA, rw::BLENDSRCALPHA);
CHECK_VALUE(RW_BLEND_INV_SRC_ALPHA, rw::BLENDINVSRCALPHA);
CHECK_VALUE(RW_IM3D_VERTEX_UV, rw::im3d::VERTEXUV);
CHECK_VALUE(RW_IM3D_ALL_OPAQUE, rw::im3d::ALLOPAQUE);
CHECK_VALUE(RW_IM3D_NO_CLIP, rw::im3d::NOCLIP);
CHECK_VALUE(RW_IM3D_VERTEX_XYZ, rw::im3d::VERTEXXYZ);
CHECK_VALUE(RW_IM3D_VERTEX_RGBA, rw::im3d::VERTEXRGBA);
CHECK_VALUE(RW_CAMERA_PERSPECTIVE, rw::Camera::PERSPECTIVE);
CHECK_VALUE(RW_CAMERA_PARALLEL, rw::Camera::PARALLEL);
#ifdef RW_GL3
CHECK_VALUE(RW_GL3_UNIFORM_NA, rw::gl3::UNIFORM_NA);
CHECK_VALUE(RW_GL3_UNIFORM_VEC4, rw::gl3::UNIFORM_VEC4);
CHECK_VALUE(RW_GL3_UNIFORM_IVEC4, rw::gl3::UNIFORM_IVEC4);
CHECK_VALUE(RW_GL3_UNIFORM_MAT4, rw::gl3::UNIFORM_MAT4);
#endif

#undef CHECK_VALUE
#undef CHECK_FIELD
#undef CHECK_SIZE

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
    rw::Skin *skin(rw_Skin *value) { return cast<rw::Skin>(value); }

    struct WorldLightsBox {
        rw::WorldLights lights;
        int32_t maxDirectionals;
        int32_t maxLocals;
    };

    WorldLightsBox *lightsBox(rw_WorldLights *value) { return cast<WorldLightsBox>(value); }
    const WorldLightsBox *lightsBox(const rw_WorldLights *value) { return cast<WorldLightsBox>(value); }

#ifdef RW_GL3
#include "src/gl/shaders/header_vs.inc"
#include "src/gl/shaders/header_fs.inc"

    struct PipelineSlot {
        rw::gl3::ObjPipeline *pipeline;
        rw_gl3_render_callback callback;
        void *userData;
    };
    PipelineSlot pipelineSlots[32] = {};

    rw::gl3::Gl3Raster *gl3Raster(const rw_Raster *value) {
        return reinterpret_cast<rw::gl3::Gl3Raster *>(
            reinterpret_cast<char *>(const_cast<rw_Raster *>(value)) +
            rw::gl3::nativeRasterOffset);
    }

    rw::gl3::InstanceDataHeader *instanceData(rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value);
    }

    PipelineSlot *findPipelineSlot(rw::gl3::ObjPipeline *pipeline) {
        for (size_t i = 0; i < sizeof(pipelineSlots)/sizeof(pipelineSlots[0]); i++)
            if (pipelineSlots[i].pipeline == pipeline)
                return &pipelineSlots[i];
        return NULL;
    }

    void gl3RenderTrampoline(rw::Atomic *object, rw::gl3::InstanceDataHeader *header) {
        rw::gl3::ObjPipeline *pipeline =
            static_cast<rw::gl3::ObjPipeline *>(object->getPipeline());
        PipelineSlot *slot = findPipelineSlot(pipeline);
        if (slot != NULL && slot->callback != NULL)
            slot->callback(cast<rw_Atomic>(object),
                cast<rw_Gl3InstanceData>(header), slot->userData);
    }
#endif
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
    void librw_frame_update_objects(rw_Frame *value) { frame(value)->updateObjects(); }

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
    int32_t librw_raster_type(const rw_Raster *value) { return cast<rw::Raster>(value)->type; }
    rw_Raster *librw_raster_parent(const rw_Raster *value) { return cast<rw_Raster>(cast<rw::Raster>(value)->parent); }
    
    void librw_raster_sub_raster(rw_Raster *value, rw_Raster *parent, int32_t x, int32_t y, int32_t width, int32_t height) {
        rw::Rect rect;
        rect.x = x; rect.y = y; rect.w = width; rect.h = height;
        raster(value)->subRaster(raster(parent), &rect);
    }

    rw_Raster *librw_raster_set_from_image(rw_Raster *value, rw_Image *img) {
        return cast<rw_Raster>(raster(value)->setFromImage(image(img)));
    }
    
    void librw_raster_show(rw_Raster *value, uint32_t flags) { raster(value)->show(flags); }

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
    uint32_t librw_texture_filter_addressing(const rw_Texture *value) { return cast<rw::Texture>(value)->filterAddressing; }
    void librw_texture_set_filter_addressing(rw_Texture *value, uint32_t v) { texture(value)->filterAddressing = v; }
    void librw_texture_set_raster(rw_Texture *value, rw_Raster *r) { texture(value)->raster = r ? raster(r) : NULL; }

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
    void librw_geometry_set_flags(rw_Geometry *value, uint32_t flags) { geometry(value)->flags = flags; }
    int32_t librw_geometry_num_tex_coord_sets(const rw_Geometry *value) { return cast<rw::Geometry>(value)->numTexCoordSets; }
    void librw_geometry_build_meshes(rw_Geometry *value) { geometry(value)->buildMeshes(); }
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
    const rw_Sphere *librw_atomic_world_bounding_sphere(rw_Atomic *value) {
        return cast<rw_Sphere>(atomic(value)->getWorldBoundingSphere());
    }

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
    float librw_light_radius(const rw_Light *value) { return cast<rw::Light>(value)->radius; }
    const rw_RGBAf *librw_light_color(const rw_Light *value) { return cast<rw_RGBAf>(&cast<rw::Light>(value)->color); }
    rw_Frame *librw_light_frame(const rw_Light *value) {
        return cast<rw_Frame>(const_cast<rw::Light *>(cast<rw::Light>(value))->getFrame());
    }
    rw_World *librw_light_world(const rw_Light *value) { return cast<rw_World>(cast<rw::Light>(value)->world); }
    void librw_light_set_type(rw_Light *value, int32_t kind) { light(value)->object.object.subType = (rw::uint8)kind; }

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
    rw_Frame *librw_camera_frame(const rw_Camera *value) {
        return cast<rw_Frame>(const_cast<rw::Camera *>(cast<rw::Camera>(value))->getFrame());
    }
    rw_Raster *librw_camera_frame_buffer(const rw_Camera *value) { return cast<rw_Raster>(cast<rw::Camera>(value)->frameBuffer); }
    rw_Raster *librw_camera_z_buffer(const rw_Camera *value) { return cast<rw_Raster>(cast<rw::Camera>(value)->zBuffer); }
    void librw_camera_set_fog_plane(rw_Camera *value, float v) { camera(value)->fogPlane = v; }

    rw_World *librw_world_create(const rw_BBox *bounds) { return cast<rw_World>(rw::World::create(const_cast<rw::BBox *>(cast<rw::BBox>(bounds)))); }
    void librw_world_destroy(rw_World *value) { world(value)->destroy(); }
    void librw_world_add_clump(rw_World *value, rw_Clump *object) { world(value)->addClump(clump(object)); }
    void librw_world_remove_clump(rw_World *value, rw_Clump *object) { world(value)->removeClump(clump(object)); }
    void librw_world_add_light(rw_World *value, rw_Light *object) { world(value)->addLight(light(object)); }
    void librw_world_remove_light(rw_World *value, rw_Light *object) { world(value)->removeLight(light(object)); }
    void librw_world_add_camera(rw_World *value, rw_Camera *object) { world(value)->addCamera(camera(object)); }
    void librw_world_remove_camera(rw_World *value, rw_Camera *object) { world(value)->removeCamera(camera(object)); }
    void librw_world_render(rw_World *value) { world(value)->render(); }
    void librw_world_for_each_light(rw_World *value, rw_bool32 local, rw_light_callback callback, void *userData) {
        rw::World *w = world(value);
        rw::LinkList &lights = local ? w->localLights : w->globalLights;
        FORLIST(link, lights)
            callback(cast<rw_Light>(rw::Light::fromWorld(link)), userData);
    }

    void librw_set_render_state(int32_t state, uint32_t value) { rw::SetRenderState(state, value); }
    void librw_set_render_state_ptr(int32_t state, void *value) { rw::SetRenderStatePtr(state, value); }
    uint32_t librw_get_render_state(int32_t state) { return rw::GetRenderState(state); }
    void *librw_get_render_state_ptr(int32_t state) { return rw::GetRenderStatePtr(state); }

    rw_bool32 librw_raster_image_find_format(rw_Image *value, int32_t type,
            int32_t *width, int32_t *height, int32_t *depth, int32_t *format, int32_t platform) {
        return rw::Raster::imageFindRasterFormat(image(value), type,
            width, height, depth, format, platform);
    }

    void librw_camera_set_projection(rw_Camera *value, int32_t projection) {
        camera(value)->setProjection(projection);
    }
    void librw_camera_set_view_window(rw_Camera *value, const rw_V2d *window) {
        camera(value)->setViewWindow(cast<rw::V2d>(window));
    }
    void librw_camera_set_view_offset(rw_Camera *value, const rw_V2d *offset) {
        camera(value)->setViewOffset(cast<rw::V2d>(offset));
    }
    void librw_camera_show_raster(rw_Camera *value, uint32_t flags) {
        camera(value)->showRaster(flags);
    }

    float librw_im2d_near_z(void) { return rw::im2d::GetNearZ(); }
    float librw_im2d_far_z(void) { return rw::im2d::GetFarZ(); }
    void librw_im2d_render_primitive(int32_t primitive, void *vertices, int32_t vertexCount) {
        rw::im2d::RenderPrimitive(static_cast<rw::PrimitiveType>(primitive), vertices, vertexCount);
    }
    void librw_im2d_render_indexed_primitive(int32_t primitive, void *vertices,
            int32_t vertexCount, void *indices, int32_t indexCount) {
        rw::im2d::RenderIndexedPrimitive(static_cast<rw::PrimitiveType>(primitive),
            vertices, vertexCount, indices, indexCount);
    }
    void librw_im3d_transform(void *vertices, int32_t vertexCount,
            const rw_Matrix *matrix, uint32_t flags) {
        rw::im3d::Transform(vertices, vertexCount,
            const_cast<rw::Matrix *>(cast<rw::Matrix>(matrix)), flags);
    }
    void librw_im3d_render_primitive(int32_t primitive) {
        rw::im3d::RenderPrimitive(static_cast<rw::PrimitiveType>(primitive));
    }
    void librw_im3d_render_indexed_primitive(int32_t primitive, void *indices, int32_t indexCount) {
        rw::im3d::RenderIndexedPrimitive(static_cast<rw::PrimitiveType>(primitive), indices, indexCount);
    }
    void librw_im3d_end(void) { rw::im3d::End(); }

    void librw_light_set_angle(rw_Light *value, float radians) { light(value)->setAngle(radians); }
    void librw_light_set_flags(rw_Light *value, uint32_t flags) { light(value)->setFlags(flags); }
    uint32_t librw_light_flags(const rw_Light *value) {
        return const_cast<rw::Light *>(cast<rw::Light>(value))->getFlags();
    }
    int32_t librw_light_type(const rw_Light *value) {
        return const_cast<rw::Light *>(cast<rw::Light>(value))->getType();
    }

    rw_WorldLights *librw_world_lights_new(int32_t directionalCapacity, int32_t localCapacity) {
        if (directionalCapacity < 0 || localCapacity < 0) return NULL;
        WorldLightsBox *box = new (std::nothrow) WorldLightsBox();
        if (box == NULL) return NULL;
        box->lights.directionals = directionalCapacity ?
            new (std::nothrow) rw::Light*[directionalCapacity] : NULL;
        box->lights.locals = localCapacity ? new (std::nothrow) rw::Light*[localCapacity] : NULL;
        if ((directionalCapacity && box->lights.directionals == NULL) ||
                (localCapacity && box->lights.locals == NULL)) {
            delete[] box->lights.directionals; delete[] box->lights.locals; delete box; return NULL;
        }
        box->maxDirectionals = directionalCapacity;
        box->maxLocals = localCapacity;
        librw_world_lights_clear(cast<rw_WorldLights>(box));
        return cast<rw_WorldLights>(box);
    }
    void librw_world_lights_delete(rw_WorldLights *value) {
        WorldLightsBox *box = lightsBox(value);
        delete[] box->lights.directionals; delete[] box->lights.locals; delete box;
    }
    void librw_world_lights_clear(rw_WorldLights *value) {
        rw::WorldLights &lights = lightsBox(value)->lights;
        lights.numAmbients = 0;
        lights.numDirectionals = 0;
        lights.numLocals = 0;
        lights.ambient.red = lights.ambient.green = lights.ambient.blue = 0.0f;
        lights.ambient.alpha = 1.0f;
    }
    rw_bool32 librw_world_lights_add(rw_WorldLights *value, rw_Light *object) {
        WorldLightsBox *box = lightsBox(value);
        rw::WorldLights &lights = box->lights;
        rw::Light *l = light(object);
        switch (l->getType()) {
        case rw::Light::AMBIENT:
            lights.ambient.red += l->color.red;
            lights.ambient.green += l->color.green;
            lights.ambient.blue += l->color.blue;
            lights.numAmbients++;
            return 1;
        case rw::Light::DIRECTIONAL:
            if (lights.numDirectionals >= box->maxDirectionals) return 0;
            lights.directionals[lights.numDirectionals++] = l;
            return 1;
        default:
            if (lights.numLocals >= box->maxLocals) return 0;
            lights.locals[lights.numLocals++] = l;
            return 1;
        }
    }
    void librw_world_enumerate_lights(rw_World *owner, rw_Atomic *object, rw_WorldLights *value) {
        WorldLightsBox *box = lightsBox(value);
        box->lights.numDirectionals = box->maxDirectionals;
        box->lights.numLocals = box->maxLocals;
        if (object)
            world(owner)->enumerateLights(atomic(object), &box->lights);
        else
            world(owner)->enumerateLights(&box->lights);
    }
    int32_t librw_world_lights_num_ambient(const rw_WorldLights *value) {
        return lightsBox(value)->lights.numAmbients;
    }
    int32_t librw_world_lights_num_directional(const rw_WorldLights *value) {
        return lightsBox(value)->lights.numDirectionals;
    }
    int32_t librw_world_lights_num_local(const rw_WorldLights *value) {
        return lightsBox(value)->lights.numLocals;
    }
    rw_Light *librw_world_lights_directional(const rw_WorldLights *value, int32_t index) {
        const rw::WorldLights &lights = lightsBox(value)->lights;
        return index >= 0 && index < lights.numDirectionals
            ? cast<rw_Light>(lights.directionals[index]) : NULL;
    }
    rw_Light *librw_world_lights_local(const rw_WorldLights *value, int32_t index) {
        const rw::WorldLights &lights = lightsBox(value)->lights;
        return index >= 0 && index < lights.numLocals
            ? cast<rw_Light>(lights.locals[index]) : NULL;
    }
    rw_Light *librw_light_from_world_link(void *link) {
        return cast<rw_Light>(rw::Light::fromWorld(static_cast<rw::LLLink *>(link)));
    }

    rw_Sphere *librw_geometry_bounding_sphere(rw_Geometry *value, int32_t target) {
        rw::Geometry *geo = geometry(value);
        return target >= 0 && target < geo->numMorphTargets
            ? cast<rw_Sphere>(&geo->morphTargets[target].boundingSphere) : NULL;
    }

    void librw_register_native_data_plugin(void) { rw::registerNativeDataPlugin(); }
    void librw_register_skin_plugin(void) { rw::registerSkinPlugin(); }
    void librw_register_hanim_plugin(void) { rw::registerHAnimPlugin(); }
    void librw_register_matfx_plugin(void) { rw::registerMatFXPlugin(); }
    int32_t librw_raster_register_plugin(int32_t size, uint32_t id) {
        return rw::Raster::registerPlugin(size, id, NULL, NULL, NULL);
    }
    int32_t librw_texture_register_plugin(int32_t size, uint32_t id) {
        return rw::Texture::registerPlugin(size, id, NULL, NULL, NULL);
    }

    void *librw_skin_get(const rw_Geometry *value) { return rw::Skin::get(cast<rw::Geometry>(value)); }
    void librw_skin_set(rw_Geometry *value, void *skin) {
        rw::Skin::set(geometry(value), static_cast<rw::Skin *>(skin));
    }
    void librw_skin_set_hierarchy(rw_Atomic *value, void *hierarchy) {
        rw::Skin::setHierarchy(atomic(value), static_cast<rw::HAnimHierarchy *>(hierarchy));
    }
    void librw_skin_set_pipeline(rw_Atomic *value, int32_t type) {
        rw::Skin::setPipeline(atomic(value), type);
    }
    int32_t librw_skin_geometry_offset(void) { return rw::skinGlobals.geoOffset; }
    int32_t librw_skin_atomic_offset(void) { return rw::skinGlobals.atomicOffset; }
    rw_Skin *librw_skin_create(int32_t bones, int32_t usedBones, int32_t vertices) {
        rw::Skin *result = rwNewT(rw::Skin, 1, rw::MEMDUR_EVENT | rw::ID_SKIN);
        if (result == NULL) return NULL;
        std::memset(result, 0, sizeof(*result));
        result->init(bones, usedBones, vertices);
        return cast<rw_Skin>(result);
    }
    uint8_t *librw_skin_used_bones(rw_Skin *value) { return skin(value)->usedBones; }
    uint8_t *librw_skin_indices(rw_Skin *value) { return skin(value)->indices; }
    float *librw_skin_weights(rw_Skin *value) { return skin(value)->weights; }
    rw_Matrix *librw_skin_inverse_matrices(rw_Skin *value) { return cast<rw_Matrix>(skin(value)->inverseMatrices); }
    void librw_skin_find_num_weights(rw_Skin *value, int32_t vertices) { skin(value)->findNumWeights(vertices); }

    void *librw_hanim_hierarchy_create(int32_t count, int32_t *flags,
            int32_t *ids, int32_t hierarchyFlags, int32_t maxKeySize) {
        return rw::HAnimHierarchy::create(count, flags, ids, hierarchyFlags, maxKeySize);
    }
    void librw_hanim_hierarchy_destroy(void *value) {
        static_cast<rw::HAnimHierarchy *>(value)->destroy();
    }
    rw_Matrix *librw_hanim_hierarchy_matrices(void *value) {
        return cast<rw_Matrix>(static_cast<rw::HAnimHierarchy *>(value)->matrices);
    }
    int32_t librw_hanim_hierarchy_node_count(const void *value) {
        return static_cast<const rw::HAnimHierarchy *>(value)->numNodes;
    }

    uint32_t librw_matfx_effects(const rw_Material *value) {
        return rw::MatFX::getEffects(cast<rw::Material>(value));
    }
    rw_Texture *librw_matfx_env_texture(const rw_Material *value) {
        rw::MatFX *fx = rw::MatFX::get(cast<rw::Material>(value));
        return fx ? cast<rw_Texture>(fx->getEnvTexture()) : NULL;
    }
    float librw_matfx_env_coefficient(const rw_Material *value) {
        rw::MatFX *fx = rw::MatFX::get(cast<rw::Material>(value));
        return fx ? fx->getEnvCoefficient() : 0.0f;
    }
    rw_Texture *librw_matfx_dual_texture(const rw_Material *value) {
        rw::MatFX *fx = rw::MatFX::get(cast<rw::Material>(value));
        return fx ? cast<rw_Texture>(fx->getDualTexture()) : NULL;
    }
    int32_t librw_matfx_dual_src_blend(const rw_Material *value) {
        rw::MatFX *fx = rw::MatFX::get(cast<rw::Material>(value));
        return fx ? fx->getDualSrcBlend() : 0;
    }
    int32_t librw_matfx_dual_dest_blend(const rw_Material *value) {
        rw::MatFX *fx = rw::MatFX::get(cast<rw::Material>(value));
        return fx ? fx->getDualDestBlend() : 0;
    }

#ifdef RW_GL3
    rw_bool32 librw_engine_open_gl3(void **window, rw_bool32 fullscreen,
            int32_t width, int32_t height, const char *title) {
        rw::EngineOpenParams params;
        params.window = reinterpret_cast<SDL_Window **>(window);
        params.fullscreen = fullscreen;
        params.width = width;
        params.height = height;
        params.windowtitle = title;
        return rw::Engine::open(&params);
    }
    int32_t librw_gl3_set_lights(rw_WorldLights *value) {
        return rw::gl3::setLights(cast<rw::WorldLights>(value));
    }
    int32_t librw_gl3_native_raster_offset(void) { return rw::gl3::nativeRasterOffset; }
    uint32_t librw_gl3_raster_texture(const rw_Raster *value) {
        return gl3Raster(value)->texid;
    }
    uint32_t librw_gl3_raster_fbo(const rw_Raster *value) {
        return gl3Raster(value)->fbo;
    }
    int32_t librw_gl3_raster_internal_format(const rw_Raster *value) {
        return gl3Raster(value)->internalFormat;
    }
    rw_Raster *librw_gl3_raster_fbo_mate(const rw_Raster *value) {
        return cast<rw_Raster>(gl3Raster(value)->fboMate);
    }
    int32_t librw_gl3_raster_format(const rw_Raster *value) {
        return gl3Raster(value)->format;
    }
    int32_t librw_gl3_raster_bytes_per_pixel(const rw_Raster *value) {
        return gl3Raster(value)->bpp;
    }
    uint32_t librw_gl3_instance_vbo(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->vbo;
    }
    uint32_t librw_gl3_instance_ibo(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->ibo;
    }
    uint32_t librw_gl3_instance_mesh_count(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->numMeshes;
    }
    uint32_t librw_gl3_instance_vertex_count(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->totalNumVertex;
    }
    uint32_t librw_gl3_instance_index_count(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->totalNumIndex;
    }
    int32_t librw_gl3_instance_attribute_count(const rw_Gl3InstanceData *value) {
        return cast<rw::gl3::InstanceDataHeader>(value)->numAttribs;
    }
    rw_bool32 librw_gl3_instance_attribute(const rw_Gl3InstanceData *value, int32_t index,
            uint32_t *location, int32_t *type, rw_bool32 *normalized, int32_t *size,
            uint32_t *stride, uint32_t *offset) {
        const rw::gl3::InstanceDataHeader *header = cast<rw::gl3::InstanceDataHeader>(value);
        if (index < 0 || index >= header->numAttribs) return 0;
        const rw::gl3::AttribDesc &attribute = header->attribDesc[index];
        *location = attribute.index; *type = attribute.type; *normalized = attribute.normalized;
        *size = attribute.size; *stride = attribute.stride; *offset = attribute.offset;
        return 1;
    }
    rw_bool32 librw_gl3_instance_mesh(const rw_Gl3InstanceData *value, int32_t index,
            uint32_t *indexCount, uint32_t *minVertex, int32_t *vertexCount,
            rw_Material **materialOut, rw_bool32 *vertexAlpha, uint32_t *program, uint32_t *offset) {
        const rw::gl3::InstanceDataHeader *header = cast<rw::gl3::InstanceDataHeader>(value);
        if (index < 0 || (uint32_t)index >= header->numMeshes) return 0;
        const rw::gl3::InstanceData &mesh = header->inst[index];
        *indexCount = mesh.numIndex; *minVertex = mesh.minVert; *vertexCount = mesh.numVertices;
        *materialOut = cast<rw_Material>(mesh.material); *vertexAlpha = mesh.vertexAlpha;
        *program = mesh.program; *offset = mesh.offset;
        return 1;
    }
    rw_Gl3Shader *librw_gl3_current_shader(void) {
        return cast<rw_Gl3Shader>(rw::gl3::currentShader);
    }
    uint32_t librw_gl3_shader_program(const rw_Gl3Shader *value) {
        return cast<rw::gl3::Shader>(value)->program;
    }
    void librw_gl3_shader_use(rw_Gl3Shader *value) { cast<rw::gl3::Shader>(value)->use(); }
    void librw_gl3_set_uniform(int32_t id, void *data) { rw::gl3::setUniform(id, data); }
    int32_t librw_gl3_register_uniform(const char *name, int32_t type, int32_t count) {
        return rw::gl3::registerUniform(name, static_cast<rw::gl3::UniformType>(type), count);
    }
    rw_bool32 librw_gl3_uniform_info(int32_t id, int32_t *type, int32_t *count) {
        if (id < 0 || id >= rw::gl3::uniformRegistry.numUniforms) return 0;
        const rw::gl3::Uniform &u = rw::gl3::uniformRegistry.uniforms[id];
        *type = u.type; *count = u.num;
        return 1;
    }
    rw_Gl3Shader *librw_gl3_shader_create(const char **vertex, const char **fragment) {
        return cast<rw_Gl3Shader>(rw::gl3::Shader::create(vertex, fragment));
    }
    void librw_gl3_shader_destroy(rw_Gl3Shader *value) { cast<rw::gl3::Shader>(value)->destroy(); }
    const char *librw_gl3_shader_decl(void) { return rw::gl3::shaderDecl; }
    const char *librw_gl3_shader_header_vs(void) { return header_vert_src; }
    const char *librw_gl3_shader_header_fs(void) { return header_frag_src; }
    void librw_gl3_set_world_matrix(const rw_Matrix *matrix) {
        rw::gl3::setWorldMatrix(const_cast<rw::Matrix *>(cast<rw::Matrix>(matrix)));
    }
    void librw_gl3_setup_vertex_input(rw_Gl3InstanceData *value) { rw::gl3::setupVertexInput(instanceData(value)); }
    void librw_gl3_teardown_vertex_input(rw_Gl3InstanceData *value) { rw::gl3::teardownVertexInput(instanceData(value)); }
    void librw_gl3_upload_skin_matrices(rw_Atomic *value) {
        rw::Atomic *a = atomic(value);
        rw::Skin *skin = a->geometry ? rw::Skin::get(a->geometry) : NULL;
        rw::HAnimHierarchy *hier = rw::Skin::getHierarchy(a);
        if (skin == NULL || skin->numBones > 64 ||
                (hier != NULL && (hier->numNodes != skin->numBones || hier->matrices == NULL ||
                    (!(hier->flags & rw::HAnimHierarchy::LOCALSPACEMATRICES) && a->getFrame() == NULL))))
            return;
        rw::gl3::uploadSkinMatrices(a);
    }
    void librw_gl3_set_material(uint32_t flags, const rw_RGBA *color, const rw_SurfaceProperties *surface) {
        rw::gl3::setMaterial(flags, *cast<rw::RGBA>(color), *cast<rw::SurfaceProperties>(surface));
    }
    void librw_gl3_set_texture(int32_t stage, rw_Texture *value) { rw::gl3::setTexture(stage, value ? texture(value) : NULL); }
    void librw_gl3_draw_mesh(rw_Gl3InstanceData *value, int32_t index) {
        rw::gl3::InstanceDataHeader *header = instanceData(value);
        if (index >= 0 && (uint32_t)index < header->numMeshes)
            rw::gl3::drawInst(header, &header->inst[index]);
    }
    rw_Gl3Pipeline *librw_gl3_pipeline_create_like(rw_Gl3Pipeline *base, rw_gl3_render_callback callback, void *userData) {
        rw::gl3::ObjPipeline *pipeline = rw::gl3::ObjPipeline::create();
        if (pipeline == NULL)
            return NULL;
        for (size_t i = 0; i < sizeof(pipelineSlots)/sizeof(pipelineSlots[0]); i++) {
            if (pipelineSlots[i].pipeline == NULL) {
                pipelineSlots[i].pipeline = pipeline;
                pipelineSlots[i].callback = callback;
                pipelineSlots[i].userData = userData;
                if (base != NULL) {
                    rw::gl3::ObjPipeline *from = cast<rw::gl3::ObjPipeline>(base);
                    pipeline->instanceCB = from->instanceCB;
                    pipeline->uninstanceCB = from->uninstanceCB;
                    pipeline->pluginID = from->pluginID;
                    pipeline->pluginData = from->pluginData;
                } else {
                    pipeline->instanceCB = rw::gl3::defaultInstanceCB;
                    pipeline->uninstanceCB = rw::gl3::defaultUninstanceCB;
                }
                pipeline->renderCB = gl3RenderTrampoline;
                return cast<rw_Gl3Pipeline>(pipeline);
            }
        }
        pipeline->destroy();
        return NULL;
    }
    rw_Gl3Pipeline *librw_gl3_pipeline_create(rw_gl3_render_callback callback, void *userData) {
        return librw_gl3_pipeline_create_like(NULL, callback, userData);
    }
    rw_Gl3Pipeline *librw_gl3_skin_pipeline(void) {
        return cast<rw_Gl3Pipeline>(rw::skinGlobals.pipelines[rw::PLATFORM_GL3]);
    }
    void librw_gl3_pipeline_destroy(rw_Gl3Pipeline *value) {
        rw::gl3::ObjPipeline *pipeline = cast<rw::gl3::ObjPipeline>(value);
        PipelineSlot *slot = findPipelineSlot(pipeline);
        if (slot != NULL)
            *slot = PipelineSlot();
        pipeline->destroy();
    }
    void librw_atomic_set_gl3_pipeline(rw_Atomic *value, rw_Gl3Pipeline *pipeline) {
        atomic(value)->pipeline = cast<rw::gl3::ObjPipeline>(pipeline);
    }
    rw_Gl3Pipeline *librw_atomic_gl3_pipeline(const rw_Atomic *value) {
        return cast<rw_Gl3Pipeline>(cast<rw::Atomic>(value)->pipeline);
    }
#endif
} // extern
