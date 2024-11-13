use crate::math::{self, vector3::Vector3};

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

    pub fn min_dist_sq_line(&self, other: &LineSegment) -> f32 {
        let u = self.end.clone() - other.start.clone();
        let v = other.end.clone() - self.start.clone();
        let w = self.start.clone() - other.start.clone();

        let a = Vector3::dot(&u, &u); // always >= 0
        let b = Vector3::dot(&u, &v);
        let c = Vector3::dot(&v, &v); // always >= 0
        let d = Vector3::dot(&u, &w);
        let e = Vector3::dot(&v, &w);

        let discriminant = a * c - b * b; // always >= 0
        let mut sc = discriminant; // sc = sN / sD, default sD = D >= 0
        let mut sn = discriminant;
        let mut sd = discriminant;
        let mut tc = discriminant; // tc = tN / tD, default tD = D >= 0
        let mut tn = discriminant;
        let mut td = discriminant;

        // compute the line parameters of the two closest points
        if math::basic::near_zero(discriminant, 0.001) {
            // the lines are almost parallel
            // force using point P0 on segment S1
            // to prevent possible division by 0.0 later
            sn = 0.0;
            sd = 1.0;
            tn = e;
            td = c;
        } else {
            // get the closest points on the infinite lines
            sn = b * e - c * d;
            tn = a * e - b * d;
            if sn < 0.0 {
                // sc < 0 => the s=0 edge is visible
                sn = 0.0;
                tn = e;
                td = c;
            } else if sn > sd {
                // sc > 1  => the s=1 edge is visible
                sn = sd;
                tn = e + b;
                td = c;
            }
        }

        if tn < 0.0 {
            // tc < 0 => the t=0 edge is visible
            tn = 0.0;
            // recompute sc for this edge
            if -d < 0.0 {
                sn = 0.0;
            } else if -d > a {
                sn = sd;
            } else {
                sn = -d;
                sd = a;
            }
        } else if tn > td {
            // tc > 1  => the t=1 edge is visible
            tn = td;
            // recompute sc for this edge
            if (-d + b) < 0.0 {
                sn = 0.0;
            } else if (-d + b) > a {
                sn = sd;
            } else {
                sn = -d + b;
                sd = a;
            }
        }

        // finally do the division to get sc and tc
        sc = if math::basic::near_zero(sn, 0.001) {
            0.0
        } else {
            sn / sd
        };
        tc = if math::basic::near_zero(tn, 0.001) {
            0.0
        } else {
            tn / td
        };

        // get the difference of the two closest points
        let dp = w + (u * sc) - (v * tc);

        dp.length_sq()
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
