use crate::math::vector3::Vector3;

pub struct LineSegment {
    start: Vector3,
    end: Vector3,
}

impl LineSegment {
    pub fn new(start: Vector3, end: Vector3) -> Self {
        Self { start, end }
    }

    pub fn point_on_segment(&self, t: f32) -> Vector3 {
        self.start.clone() + (self.end.clone() - self.start.clone()) * t
    }

    pub fn min_dist_sq(&self, point: &Vector3) -> f32 {
        // Construct vectors
        let ab = self.end.clone() - self.start.clone();
        let ba = ab.clone() * -1.0;
        let ac = point.clone() - self.start.clone();
        let bc = point.clone() - self.end.clone();

        // Case 1: C projects prior to A
        if Vector3::dot(&ab, &ac) < 0.0 {
            return ac.length_sq();
        }

        // Case 2: C projects after B
        if Vector3::dot(&ba, &bc) < 0.0 {
            return bc.length_sq();
        }

        // Case 3: C projects onto line
        let scaler = Vector3::dot(&ac, &ab) / Vector3::dot(&ab, &ab);
        let p = ab * scaler;
        (ac - p).length_sq()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    use super::LineSegment;

    #[test]
    fn test_point_on_segment() {
        let expected = Vector3::new(1.0, 1.0, 1.0);

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let actual = segment.point_on_segment(0.5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_min_dist_sq_case_1_c_projects_prior_to_a() {
        let expected = 2.0;

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let actual = segment.min_dist_sq(&Vector3::new(-1.0, -1.0, 0.0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_min_dist_sq_case_2_c_projects_after_b() {
        let expected = 2.0;

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let actual = segment.min_dist_sq(&Vector3::new(3.0, 3.0, 2.0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_min_dist_sq_case_3_c_onto_line() {
        let expected = 0.5;

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 0.0));
        let actual = segment.min_dist_sq(&Vector3::new(0.5, 1.5, 0.0));

        assert_eq!(expected, actual);
    }
}
