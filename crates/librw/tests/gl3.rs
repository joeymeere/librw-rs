#![cfg(feature = "gl3")]

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use librw::{
    Atomic, Engine, Frame, Geometry, GeometryFlags, Gl3OpenParams, Gl3Pipeline, Gl3Shader,
    Material, Triangle, Vec3,
};

#[test]
fn test_gl3_callback_runs_during_atomic_render() {
    if std::env::var_os("LIBRW_GL3_TESTS").is_none() {
        return;
    }

    let mut window = core::ptr::null_mut();

    let (engine, tint) = Engine::new_gl3_with_plugins(
        Gl3OpenParams {
            window: &mut window,
            fullscreen: false,
            width: 32,
            height: 32,
            title: c"librw-rs gl3 test",
        },
        |plugins| {
            plugins.register_skin();
            plugins.register_gl3_uniform::<1>(c"u_tint")
        },
    )
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

    let decl = librw::shader_decl(&engine);
    let mut shader = Gl3Shader::new(
        &engine,
        &[
            decl,
            librw::shader_header_vertex(),
            c"VSIN(ATTRIB_POS) vec3 in_pos;
void main(void) { gl_Position = u_proj * u_view * u_world * vec4(in_pos, 1.0); }
",
        ],
        &[
            decl,
            librw::shader_header_fragment(),
            c"uniform vec4 u_tint;
void main(void) { FRAGCOLOR(u_tint); }
",
        ],
    )
    .expect("shader was compiled");

    let steps = Arc::new(AtomicUsize::new(0));
    let callback_steps = Arc::clone(&steps);
    let custom = Box::leak(Box::new(
        Gl3Pipeline::new(&engine, move |atomic, instance| {
            let frame = atomic.frame().expect("atomic has no frame");
            instance.set_world_matrix(&frame.local_to_world());
            instance.setup_vertex_input();
            let mesh = instance.mesh(0).expect("instance has one mesh");
            let material = mesh.material.expect("mesh has a material");
            instance.set_material(
                GeometryFlags::POSITIONS,
                material.color(),
                material.surface_properties(),
            );
            instance.set_texture(0, material.texture().as_ref());
            tint.set(&[[1.0, 0.5, 0.25, 1.0]]);
            instance.draw_mesh(0);
            instance.teardown_vertex_input();
            callback_steps.fetch_add(1, Ordering::Relaxed);
        })
        .expect("pipeline was created"),
    ));
    let frame = Frame::new(&engine).expect("frame was created");
    let mut framed = Atomic::new(&engine).expect("atomic was created");
    framed.set_geometry(Some(&geometry));
    unsafe { librw_sys::librw_atomic_set_frame(framed.as_raw(), frame.as_raw()) };
    framed.set_gl3_pipeline(custom);
    shader.bind();
    framed.render();
    assert_eq!(steps.load(Ordering::Relaxed), 1);

    Gl3Pipeline::new_skin(&engine, |_, _| {}).expect("skin pipeline was created");
}
