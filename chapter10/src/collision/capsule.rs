use crate::math::vector3::Vector3;

use super::line_segment::LineSegment;

pub struct Capsule {
    segment: LineSegment,
    radius: f32,
}

impl Capsule {
    pub fn new(start: Vector3, end: Vector3, radius: f32) -> Self {
        Self {
            segment: LineSegment::new(start, end),
            radius,
        }
    }

    pub fn point_on_segment(&self, t: f32) -> Vector3 {
        self.segment.point_on_segment(t)
    }

    pub fn contains(&self, point: &Vector3) -> bool {
        let dist_sq = self.segment.min_dist_sq(point);
        dist_sq <= self.radius * self.radius
    }

    pub fn intersect(&self, other: &Capsule) -> bool {
        let line_dist_sq = LineSegment::min_dist_sq_line(&self.segment, &other.segment);
        let sum_radius = self.radius + other.radius;
        line_dist_sq <= sum_radius * sum_radius
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    use super::Capsule;

    #[test]
    fn test_contains() {
        let capsule = Capsule::new(Vector3::ZERO, Vector3::new(0.0, 5.0, 0.0), 1.0);
        let actual = capsule.contains(&Vector3::new(0.8, 3.0, 0.0));

        assert!(actual);
    }

    #[test]
    fn test_not_contains() {
        let capsule = Capsule::new(Vector3::ZERO, Vector3::new(0.0, 5.0, 0.0), 1.0);
        let actual = capsule.contains(&Vector3::new(1.8, 3.0, 0.0));

        assert!(!actual);
    }

    #[test]
    fn test_intersect() {
        let a = Capsule::new(Vector3::ZERO, Vector3::new(0.0, 5.0, 0.0), 1.0);
        let b = Capsule::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 5.0, 0.0),
            1.0,
        );
        let actual = Capsule::intersect(&a, &b);

        assert!(actual);
    }

    #[test]
    fn test_not_intersect() {
        let a = Capsule::new(Vector3::ZERO, Vector3::new(0.0, 5.0, 0.0), 1.0);
        let b = Capsule::new(
            Vector3::new(3.0, 0.0, 0.0),
            Vector3::new(3.0, 5.0, 0.0),
            1.0,
        );
        let actual = Capsule::intersect(&a, &b);

        assert!(!actual);
    }
}
