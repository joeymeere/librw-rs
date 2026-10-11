#![cfg(not(feature = "gl3"))]

use core::ptr;

use librw::{Engine, Light, LightType, World};
use librw_sys as sys;

#[test]
fn test_world_lights() {
    let engine = Engine::new().expect("engine was created");
    let light = |kind| Light::new(&engine, kind).expect("light was created");

    let mut world = World::new(&engine, None).expect("world was created");

    for kind in [LightType::Ambient, LightType::Directional, LightType::Point] {
        world.add_light(light(kind));
    }

    let mut global = Vec::new();
    world.for_each_light(false, |l| global.push(l.kind()));

    let mut local = Vec::new();
    world.for_each_light(true, |l| local.push(l.kind()));

    assert_eq!(
        global,
        [Some(LightType::Ambient), Some(LightType::Directional)]
    );

    assert_eq!(local, [Some(LightType::Point)]);

    let ambient = light(LightType::Ambient);
    let directional = light(LightType::Directional);
    let point = light(LightType::Point);

    unsafe {
        let lights = sys::librw_world_lights_new(2, 1);

        assert!(!lights.is_null());
        assert_eq!(sys::librw_world_lights_num_directional(lights), 0);

        sys::librw_world_enumerate_lights(world.as_raw(), ptr::null_mut(), lights);
        assert_eq!(sys::librw_world_lights_num_directional(lights), 1);

        world.add_light(light(LightType::Directional));
        sys::librw_world_enumerate_lights(world.as_raw(), ptr::null_mut(), lights);

        assert_eq!(sys::librw_world_lights_num_directional(lights), 2);

        sys::librw_world_lights_clear(lights);

        assert_eq!(sys::librw_world_lights_num_directional(lights), 0);
        assert_eq!(sys::librw_world_lights_add(lights, ambient.as_raw()), 1);
        assert_eq!(sys::librw_world_lights_add(lights, ambient.as_raw()), 1);
        assert_eq!(sys::librw_world_lights_add(lights, directional.as_raw()), 1);
        assert_eq!(sys::librw_world_lights_add(lights, directional.as_raw()), 1);
        assert_eq!(sys::librw_world_lights_add(lights, directional.as_raw()), 0);
        assert_eq!(sys::librw_world_lights_add(lights, point.as_raw()), 1);
        assert_eq!(sys::librw_world_lights_add(lights, point.as_raw()), 0);
        assert_eq!(sys::librw_world_lights_num_ambient(lights), 2);
        assert_eq!(sys::librw_world_lights_num_local(lights), 1);
        assert_eq!(sys::librw_world_lights_local(lights, 0), point.as_raw());

        // clear
        sys::librw_world_lights_delete(lights);
    }
}
