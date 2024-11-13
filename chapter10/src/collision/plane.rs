use crate::math::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vector3, d: f32) -> Self {
        Self { normal, d }
    }

    pub fn from(a: Vector3, b: Vector3, c: Vector3) -> Self {
        // Compute vectors from a to b and a to c
        let ab = b.clone() - a.clone();
        let ac = c.clone() - a.clone();

        // Cross product and normalize to get normal
        let mut normal = Vector3::cross(&ab, &ac);
        normal.normalize_mut();

        // d = -P dot n
        let d = -Vector3::dot(&a, &normal);

        Self { normal, d }
    }

    pub fn signed_dist(&self, point: &Vector3) -> f32 {
        let s = Vector3::dot(point, &self.normal);
        let result = s - self.d;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    use super::Plane;

    #[test]
    fn test_from() {
        let expected = Plane::new(Vector3::new(0.0, -1.0, 0.0), 1.0);

        let actual = Plane::from(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(3.0, 1.0, 5.0),
            Vector3::new(10.0, 1.0, 20.0),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signed_dist() {
        let expected = 1.0;

        let plane = Plane::new(Vector3::new(0.0, 1.0, 0.0), 1.0);
        let actual = plane.signed_dist(&Vector3::new(0.0, 2.0, 0.0));

        assert_eq!(expected, actual);
    }
}
