use librw::{
    Atomic, Clump, Engine, Error, Geometry, GeometryFlags, Material, Rgba, SurfaceProperties, Vec3,
};

#[test]
fn test_create_geometry_and_add_to_clump() {
    let engine = Engine::new().expect("engine was created");
    let mut geometry = Geometry::new(
        &engine,
        3,
        1,
        GeometryFlags::POSITIONS | GeometryFlags::NORMALS,
    )
    .expect("geometry was created");

    geometry
        .vertices(0)
        .expect("geometry was created with vertices")
        .copy_from_slice(&[
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ]);

    geometry
        .normals(0)
        // pls forgive me :'(
        .expect("geometry was created with normals")
        .expect("geometry was created with normals")
        .fill(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

    let triangles = geometry
        .triangles()
        .expect("geometry was created with one triangle");
    triangles[0].v = [0, 1, 2];
    triangles[0].mat_id = 0;

    assert!(matches!(
        geometry.vertices(1),
        Err(Error::IndexOutOfBounds { index: 1, len: 1 })
    ));

    let mut material = Material::new(&engine).expect("material was created");
    let color = Rgba {
        red: 10,
        green: 20,
        blue: 30,
        alpha: 255,
    };
    let surface_properties = SurfaceProperties {
        ambient: 0.25,
        specular: 0.5,
        diffuse: 0.75,
    };
    material.set_color(color);
    material.set_surface_properties(surface_properties);

    assert_eq!(
        geometry
            .add_material(&material)
            .expect("material was added"),
        0
    );
    geometry.calculate_bounding_sphere();
    geometry.unlock();

    let mut atomic = Atomic::new(&engine).expect("atomic was created");
    atomic.set_geometry(Some(&geometry));

    let mut clump = Clump::new(&engine).expect("clump was created");
    clump.add_atomic(atomic);

    assert_eq!(clump.atomic_count(), 1);
    assert!(clump.atomic(1).is_none());
    assert_eq!(clump.atomics().count(), 1);

    let atomic = clump.atomic(0).expect("clump contains one atomic");
    let geometry = atomic.geometry().expect("atomic has geometry");

    assert_eq!(geometry.vertex_count(), 3);
    assert_eq!(geometry.triangle_count(), 1);
    assert_eq!(geometry.material_count(), 1);
    assert_eq!(
        geometry.vertices(0).expect("geometry has vertices")[2].y,
        1.0
    );
    assert!(matches!(
        geometry.vertices(1),
        Err(Error::IndexOutOfBounds { index: 1, len: 1 })
    ));

    let material = geometry
        .material(0)
        .expect("geometry contains one material");
    assert_eq!(material.color(), color);
    assert_eq!(material.surface_properties(), surface_properties);
}
