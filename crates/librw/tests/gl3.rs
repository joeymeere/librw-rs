#![cfg(feature = "gl3")]

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use librw::{
    Atomic, Engine, Geometry, GeometryFlags, Gl3OpenParams, Gl3Pipeline, Material, Triangle, Vec3,
};

#[test]
fn test_gl3_callback_runs_during_atomic_render() {
    if std::env::var_os("LIBRW_GL3_TESTS").is_none() {
        return;
    }

    let mut window = core::ptr::null_mut();

    let engine = Engine::new_gl3(Gl3OpenParams {
        window: &mut window,
        fullscreen: false,
        width: 32,
        height: 32,
        title: c"librw-rs gl3 test",
    })
    .expect("GL3 engine was created");

    let mut geometry =
        Geometry::new(&engine, 3, 1, GeometryFlags::POSITIONS).expect("geometry was created");

    geometry
        .vertices(0)
        .expect("vertices are present")
        .copy_from_slice(&[
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ]);
    geometry.triangles().expect("triangles are present")[0] = Triangle {
        v: [0, 1, 2],
        mat_id: 0,
    };

    let material = Material::new(&engine).expect("material was created");

    geometry
        .add_material(&material)
        .expect("material was added");
    geometry.unlock();

    let mut atomic = Atomic::new(&engine).expect("atomic was created");
    atomic.set_geometry(Some(&geometry));

    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);

    let mut pipeline = Gl3Pipeline::new(&engine, move |_, instance| {
        assert_eq!(instance.mesh_count(), 1);
        callback_calls.fetch_add(1, Ordering::Relaxed);
    })
    .expect("pipeline was created");

    pipeline.render(&mut atomic);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}
