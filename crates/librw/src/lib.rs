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
pub use frame::{Combine, Frame};
pub use geometry::{Geometry, GeometryFlags, GeometryRef, Material, MaterialRef};
#[cfg(feature = "gl3")]
pub use gl3::{
    Gl3Attribute, Gl3Instance, Gl3Mesh, Gl3Pipeline, Gl3Shader, current_shader,
    current_shader_program, native_raster_offset, set_uniform,
};
pub use image::{Image, ImageDepth, Raster, RasterFlags, RasterLock, RasterLockMode, RasterRef};
pub use immediate::{
    Im2DVertex, Im3D, Im3DFlags, Im3DVertex, im2d_far_z, im2d_near_z, render_im2d,
    render_im2d_indexed,
};
pub use math::{add, cross, dot, normalize, scale, sub};
pub use plugins::{
    HAnimFlags, HAnimHierarchy, ID_SKIN, MEMDUR_EVENT, MatFx, MatFxEffects, PluginOffset,
    PluginRegistrar, SkinRef, skin_atomic_offset, skin_geometry_offset,
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
