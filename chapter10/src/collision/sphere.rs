use crate::math::vector3::Vector3;

use super::aabb::AABB;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, point: Vector3) -> bool {
        let dist_sq = (self.center.clone() - point).length_sq();
        dist_sq <= self.radius * self.radius
    }

    pub fn intersect(&self, other: &Sphere) -> bool {
        let dist_sq = (self.center.clone() - other.center.clone()).length_sq();
        let sum_radius = self.radius + other.radius;
        dist_sq <= sum_radius * sum_radius
    }

    pub fn intersect_aabb(&self, aabb: &AABB) -> bool {
        let dist_sq = aabb.min_dist_sq(&self.center);
        dist_sq <= self.radius * self.radius
    }

    fn swept_sphere(p0: &Sphere, p1: &Sphere, q0: &Sphere, q1: &Sphere) -> Option<f32> {
        // Compute X, Y, a, b, and c
        let x = p0.center.clone() - q0.center.clone();
        let y = p1.center.clone() - p0.center.clone() - (q1.center.clone() - q0.center.clone());
        let a = Vector3::dot(&y, &y);
        let b = 2.0 * Vector3::dot(&x, &y);
        let sum_radius = p0.radius + q0.radius;
        let c = Vector3::dot(&x, &x) - sum_radius * sum_radius;

        // Solve discriminant
        let mut discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // We only care about the smaller solution
        discriminant = discriminant.sqrt();
        let result = (-b - discriminant) / (2.0 * a);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{collision::aabb::AABB, math::vector3::Vector3};

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

    #[test]
    fn test_intersect() {
        let a = Sphere::new(Vector3::ZERO, 5.0);
        let b = Sphere::new(Vector3::new(1.0, 1.0, 1.0), 1.0);
        let actual = Sphere::intersect(&a, &b);

        assert!(actual);
    }

    #[test]
    fn test_not_intersect() {
        let a = Sphere::new(Vector3::ZERO, 1.0);
        let b = Sphere::new(Vector3::new(2.0, 2.0, 2.0), 1.0);
        let actual = Sphere::intersect(&a, &b);

        assert!(!actual);
    }

    #[test]
    fn test_intersect_aabb() {
        let sphere = Sphere::new(Vector3::ZERO, 5.0);
        let aabb = AABB::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
        let actual = Sphere::intersect_aabb(&sphere, &aabb);

        assert!(actual);
    }

    #[test]
    fn test_not_intersect_aabb() {
        let sphere = Sphere::new(Vector3::ZERO, 1.0);
        let aabb = AABB::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
        let actual = Sphere::intersect_aabb(&sphere, &aabb);

        assert!(!actual);
    }

    #[test]
    fn test_swept_sphere() {
        let expected = Some(0.25);

        let p0 = Sphere::new(Vector3::new(-1.0, -1.0, -1.0), 0.5);
        let p1 = Sphere::new(Vector3::new(1.0, 1.0, 1.0), 0.5);
        let q0 = Sphere::new(Vector3::new(1.0, -1.0, -1.0), 0.5);
        let q1 = Sphere::new(Vector3::new(-1.0, 1.0, 1.0), 0.5);
        let actual = Sphere::swept_sphere(&p0, &p1, &q0, &q1);

        assert_eq!(expected, actual);
    }
}
