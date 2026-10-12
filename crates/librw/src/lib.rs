mod clump;
mod engine;
mod error;
mod frame;
mod geometry;

#[cfg(feature = "gl3")]
mod gl3;

mod image;
mod immediate;
mod math;
mod plugins;
mod render;
mod scene;
mod stream;
mod texture;
mod util;

pub use clump::{Atomic, AtomicRef, Atomics, Clump};

#[cfg(feature = "gl3")]
pub use engine::Gl3OpenParams;

pub use engine::{Engine, Platform};
pub use error::{Error, Result};
pub use frame::{Combine, Frame, FrameRef};
pub use geometry::{Geometry, GeometryFlags, GeometryRef, Material, MaterialMut, MaterialRef};

#[cfg(feature = "gl3")]
pub use gl3::{
    Gl3Attribute, Gl3Instance, Gl3Mesh, Gl3Pipeline, Gl3Shader, Gl3Uniform, current_shader,
    current_shader_program, native_raster_offset, set_uniform, shader_decl, shader_header_fragment,
    shader_header_vertex,
};

pub use image::{
    Image, ImageDepth, Raster, RasterFlags, RasterFormat, RasterLock, RasterLockMode, RasterRef,
};

pub use immediate::{
    Im2DVertex, Im3D, Im3DFlags, Im3DVertex, im2d_far_z, im2d_near_z, render_im2d,
    render_im2d_indexed,
};

pub use math::{add, cross, dot, normalize, scale, sub};

pub use plugins::{
    HAnimFlags, HAnimHierarchy, ID_SKIN, MEMDUR_EVENT, MatFx, MatFxEffects, PluginOffset,
    PluginRegistrar, SkinData, SkinRef, skin_atomic_offset, skin_geometry_offset,
};

pub use render::{
    AlphaTestFunction, BlendFunction, CullMode, PrimitiveType, RenderState, render_state,
    render_state_ptr, set_render_state, set_render_state_ptr,
};

pub use scene::{Camera, Light, LightFlags, LightRef, LightType, Projection, World, WorldLights};
pub use stream::{Chunk, FileMode, FileStream, MemoryStream, SeekFrom, Stream};
pub use texture::{Texture, TextureDictionary, TextureFilter, TextureRef};

pub use librw_sys::{rw_BBox as BBox, rw_Matrix as Matrix, rw_RGBA as Rgba, rw_RGBAf as RgbaF};
pub use librw_sys::{rw_Sphere as Sphere, rw_SurfaceProperties as SurfaceProperties};
pub use librw_sys::{
    rw_TexCoords as TexCoords, rw_Triangle as Triangle, rw_V2d as Vec2, rw_V3d as Vec3,
};

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, MutexGuard};

    use super::*;

    static ENGINE_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn engine_lock_for_tests() -> MutexGuard<'static, ()> {
        ENGINE_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[cfg(feature = "gl3")]
    fn gl3_engine_for_tests() -> Option<Engine> {
        std::env::var_os("LIBRW_GL3_TESTS")?;
        let mut window = core::ptr::null_mut();
        Some(
            Engine::new_gl3(Gl3OpenParams {
                window: &mut window,
                fullscreen: false,
                width: 32,
                height: 32,
                title: c"librw-rs test",
            })
            .expect("GL3 engine was created"),
        )
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_engine_lifecycle() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");

        assert_eq!(engine.platform(), Platform::NULL.as_raw());
        assert!(matches!(Engine::new(), Err(Error::EngineAlreadyActive)));

        drop(engine);

        let engine = Engine::new().expect("engine can be restarted after being dropped");
        assert_eq!(engine.platform(), Platform::NULL.as_raw());
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_memory_stream_rw_seek_missing_chunk() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");
        let mut storage = [0_u8; 16];

        {
            let mut stream = engine
                .memory_writer(&mut storage)
                .expect("memory writer was created");

            assert!(stream.is_empty());
            assert_eq!(stream.write(b"librw").expect("bytes were written"), 5);
            assert_eq!(stream.len(), 5);
            assert_eq!(stream.position(), 5);

            stream.seek(0, SeekFrom::Start);
            let mut output = [0_u8; 5];
            assert_eq!(
                stream.read(&mut output).expect("bytes were read"),
                output.len()
            );
            assert_eq!(&output, b"librw");
        }

        assert_eq!(&storage[..5], b"librw");

        let mut stream = engine
            .memory_reader(&mut storage[..5])
            .expect("memory reader was created");

        stream.seek(2, SeekFrom::Start);
        assert_eq!(stream.position(), 2);

        stream.seek(-1, SeekFrom::Current);
        assert_eq!(stream.position(), 1);

        assert!(matches!(
            stream.find_chunk(0xDEAD_BEEF),
            Err(Error::ChunkNotFound(0xDEAD_BEEF))
        ));
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_img_dimensions_and_pixel_storage() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");

        let mut image =
            Image::new(&engine, 3, 2, ImageDepth::Rgba32).expect("RGBA image was created");

        assert_eq!(image.width(), 3);
        assert_eq!(image.height(), 2);
        assert_eq!(image.depth(), ImageDepth::Rgba32 as i32);
        assert_eq!(image.stride(), 12);
        assert!(image.palette().is_none());

        let pixels = image.pixels().expect("RGBA image has allocated pixels");
        assert_eq!(pixels.len(), 24);

        pixels.fill(0x7F);
        assert!(pixels.iter().all(|&value| value == 0x7F));
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_frame_translation_and_scale() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");
        let mut frame = Frame::new(&engine).expect("frame was created");

        frame.translate(
            Vec3 {
                x: 4.0,
                y: -2.0,
                z: 8.0,
            },
            Combine::Replace,
        );
        assert_eq!(
            frame.matrix().pos,
            Vec3 {
                x: 4.0,
                y: -2.0,
                z: 8.0,
            }
        );

        frame.scale(
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            Combine::PostConcat,
        );

        assert_eq!(frame.matrix().right.x, 2.0);
        assert_eq!(frame.matrix().up.y, 3.0);
        assert_eq!(frame.matrix().at.z, 4.0);
    }

    #[cfg(feature = "gl3")]
    #[test]
    fn test_texture_refcounts_through_raster_and_material() {
        let _guard = engine_lock_for_tests();
        let Some(engine) = gl3_engine_for_tests() else {
            return;
        };
        let raster = Raster::new(
            &engine,
            2,
            2,
            32,
            (RasterFlags::TEXTURE | RasterFlags::C8888).0,
            Platform::GL3,
        )
        .expect("raster was created");

        let mut texture = Texture::new(&engine, raster).expect("texture was created");
        texture.set_name("material-owned").expect("name was set");

        let mut material = Material::new(&engine).expect("material was created");
        material.set_texture(Some(&texture));

        drop(texture);

        let texture = material
            .texture()
            .expect("material retained its texture reference");
        assert_eq!(texture.name(), c"material-owned");
        let raster = texture.raster().expect("texture retained its raster");
        assert_eq!((raster.width(), raster.height()), (2, 2));
    }

    #[cfg(feature = "gl3")]
    #[test]
    fn test_texture_dictionary_owns_added_texture() {
        let _guard = engine_lock_for_tests();
        let Some(engine) = gl3_engine_for_tests() else {
            return;
        };
        let raster = Raster::new(
            &engine,
            1,
            1,
            32,
            (RasterFlags::TEXTURE | RasterFlags::C8888).0,
            Platform::GL3,
        )
        .expect("raster was created");

        let mut texture = Texture::new(&engine, raster).expect("texture was created");
        texture.set_name("dictionary-owned").expect("name was set");

        let mut dictionary = TextureDictionary::new(&engine).expect("dictionary was created");
        dictionary.add(texture);

        assert_eq!(dictionary.len(), 1);
        assert_eq!(
            dictionary
                .find("dictionary-owned")
                .expect("name was valid")
                .expect("dictionary retained the texture")
                .name(),
            c"dictionary-owned"
        );
    }

    #[test]
    fn test_vec3_math_helpers() {
        let x = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let y = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        assert_eq!(
            add(x, y),
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            sub(x, y),
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: 0.0
            }
        );
        assert_eq!(dot(x, y), 0.0);
        assert_eq!(
            cross(x, y),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        );
        assert_eq!(normalize(scale(x, 4.0)), x);
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_geometry_flags_materials_and_meshes() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");
        let flags = GeometryFlags::POSITIONS | GeometryFlags::TEXTURED | GeometryFlags(2 << 16);
        let mut geometry = Geometry::new(&engine, 3, 1, flags).expect("geometry was created");

        assert_eq!(geometry.tex_coord_set_count(), 2);
        geometry.set_flags(GeometryFlags::LIGHT | GeometryFlags::NORMALS);
        assert_eq!(
            geometry.flags(),
            GeometryFlags::POSITIONS | GeometryFlags::TEXTURED | GeometryFlags::LIGHT
        );

        geometry.triangles().expect("one triangle")[0] = Triangle {
            v: [0, 1, 2],
            mat_id: 1,
        };
        let material = Material::new(&engine).expect("material was created");
        geometry
            .add_material(&material)
            .expect("material was added");
        assert!(matches!(
            geometry.build_meshes(),
            Err(Error::IndexOutOfBounds { index: 1, len: 1 })
        ));
        geometry.triangles().expect("one triangle")[0].mat_id = 0;
        geometry.build_meshes().expect("meshes were built");

        let color = Rgba {
            red: 1,
            green: 2,
            blue: 3,
            alpha: 4,
        };
        let surface = SurfaceProperties {
            ambient: 0.5,
            specular: 0.25,
            diffuse: 0.125,
        };
        let mut material_mut = geometry.material_mut(0).expect("geometry has a material");
        material_mut.set_color(color);
        material_mut.set_surface_properties(surface);
        material_mut.set_texture(None);
        assert!(geometry.material_mut(1).is_none());
        assert_eq!(material.color(), color);
        assert_eq!(material.surface_properties(), surface);
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_skin_arrays_are_zeroed_and_attached() {
        let _guard = engine_lock_for_tests();
        let (engine, ()) = Engine::new_with_plugins(|plugins| {
            plugins.register_skin();
            Ok(())
        })
        .expect("engine and skin plugin were initialized");
        let mut geometry =
            Geometry::new(&engine, 3, 1, GeometryFlags::POSITIONS).expect("geometry was created");
        assert!(geometry.skin().is_none());

        geometry
            .set_skin(2, 2, |skin| {
                assert_eq!(skin.used_bones.len(), 2);
                assert_eq!(skin.indices.len(), 12);
                assert_eq!(skin.weights.len(), 12);
                assert_eq!(skin.inverse_matrices.len(), 2);
                assert!(skin.indices.iter().all(|&i| i == 0));
                assert!(skin.weights.iter().all(|&w| w == 0.0));
                skin.used_bones.copy_from_slice(&[0, 1]);
                skin.weights.iter_mut().step_by(4).for_each(|w| *w = 1.0);
            })
            .expect("skin was set");
        assert!(geometry.skin().is_some());
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_world_lights_filled_by_the_caller() {
        let _guard = engine_lock_for_tests();
        let engine = Engine::new().expect("engine was created");
        let mut world = World::new(&engine, None).expect("world was created");
        let mut point = Light::new(&engine, LightType::Directional).expect("light was created");
        point.set_kind(LightType::Point);
        point.set_radius(12.5);
        world.add_light(point);
        world.add_light(Light::new(&engine, LightType::Ambient).expect("light was created"));

        let mut lights = WorldLights::new(1, 1).expect("light list was created");
        world.for_each_light(true, |light| {
            assert_eq!(light.kind(), Some(LightType::Point));
            assert_eq!(light.radius(), 12.5);
            assert!(light.frame().is_none());
            assert!(lights.add(&light));
            assert!(!lights.add(&light));
        });
        world.for_each_light(false, |light| assert!(lights.add(&light)));
        assert_eq!(
            (
                lights.ambient_count(),
                lights.directional_count(),
                lights.local_count()
            ),
            (1, 0, 1)
        );

        let lights = lights.into_empty();
        assert_eq!(lights.local_count() + lights.ambient_count(), 0);
    }

    #[cfg(not(feature = "gl3"))]
    #[test]
    fn test_hanim_hierarchy_matrices() {
        let _guard = engine_lock_for_tests();

        let (engine, ()) = Engine::new_with_plugins(|plugins| {
            plugins.register_hanim();
            Ok(())
        })
        .expect("engine and HAnim plugin were initialized");

        let mut hierarchy = HAnimHierarchy::new(
            &engine,
            &[0, 0],
            &[10, 20],
            HAnimFlags::UPDATE_MODELING_MATRICES,
            36,
        )
        .expect("hierarchy was created");

        assert_eq!(hierarchy.node_count(), 2);
        assert_eq!(
            hierarchy.matrices().expect("matrices were allocated").len(),
            2
        );
        hierarchy.matrices_mut().expect("matrices were allocated")[0]
            .pos
            .x = 4.0;
        assert_eq!(
            hierarchy.matrices().expect("matrices were allocated")[0]
                .pos
                .x,
            4.0
        );
    }
}
