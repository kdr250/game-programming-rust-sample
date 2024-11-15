use crate::math::{quaternion::Quaternion, vector3::Vector3};

pub struct OBB {
    center: Vector3,
    rotation: Quaternion,
    extents: Vector3,
}
