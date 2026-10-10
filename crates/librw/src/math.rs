use crate::Vec3;

#[inline]
pub fn add(left: Vec3, right: Vec3) -> Vec3 {
    Vec3 {
        x: left.x + right.x,
        y: left.y + right.y,
        z: left.z + right.z,
    }
}

#[inline]
pub fn sub(left: Vec3, right: Vec3) -> Vec3 {
    Vec3 {
        x: left.x - right.x,
        y: left.y - right.y,
        z: left.z - right.z,
    }
}

#[inline]
pub fn scale(value: Vec3, factor: f32) -> Vec3 {
    Vec3 {
        x: value.x * factor,
        y: value.y * factor,
        z: value.z * factor,
    }
}

#[inline]
pub fn dot(left: Vec3, right: Vec3) -> f32 {
    left.x * right.x + left.y * right.y + left.z * right.z
}

#[inline]
pub fn cross(left: Vec3, right: Vec3) -> Vec3 {
    Vec3 {
        x: left.y * right.z - left.z * right.y,
        y: left.z * right.x - left.x * right.z,
        z: left.x * right.y - left.y * right.x,
    }
}

#[inline]
pub fn normalize(value: Vec3) -> Vec3 {
    let length = dot(value, value).sqrt();
    if length == 0.0 {
        value
    } else {
        scale(value, length.recip())
    }
}
