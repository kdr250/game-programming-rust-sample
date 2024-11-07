use crate::math::vector3::Vector3;

pub struct DirectionalLight {
    // Direction of light
    pub direction: Vector3,
    // Diffuse color
    pub diffuse_color: Vector3,
    // Specular color
    pub spec_color: Vector3,
}

impl DirectionalLight {
    pub fn new() -> Self {
        Self {
            direction: Vector3::ZERO,
            diffuse_color: Vector3::ZERO,
            spec_color: Vector3::ZERO,
        }
    }

    pub fn from(direction: Vector3, diffuse_color: Vector3, spec_color: Vector3) -> Self {
        Self {
            direction,
            diffuse_color,
            spec_color,
        }
    }
}
