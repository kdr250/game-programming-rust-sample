use crate::math::vector3::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, point: Vector3) -> bool {
        let dist_sq = (self.center.clone() - point).length_sq();
        dist_sq <= self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    use super::Sphere;

    #[test]
    fn test_contains() {
        let sphere = Sphere::new(Vector3::ZERO, 2.0);
        let actual = sphere.contains(Vector3::new(0.8, 0.8, 0.8));

        assert!(actual);
    }

    #[test]
    fn test_not_contains() {
        let sphere = Sphere::new(Vector3::ZERO, 1.0);
        let actual = sphere.contains(Vector3::new(0.8, 0.8, 0.8));

        assert!(!actual);
    }
}
