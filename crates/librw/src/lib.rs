mod clump;
mod engine;
mod error;
mod frame;
mod geometry;
mod image;
mod scene;
mod stream;
mod texture;
mod util;

pub use clump::{Atomic, AtomicRef, Atomics, Clump};
pub use engine::{Engine, Platform};
pub use error::{Error, Result};
pub use frame::{Combine, Frame};
pub use geometry::{Geometry, GeometryFlags, GeometryRef, Material, MaterialRef};
pub use image::{Image, ImageDepth, Raster, RasterLock, RasterLockMode, RasterRef};
pub use scene::{Camera, Light, LightType, World};
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
}
