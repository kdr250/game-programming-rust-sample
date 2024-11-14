use crate::math::{self, vector3::Vector3};

use super::{aabb::AABB, plane::Plane, sphere::Sphere};

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

    pub fn intersect_plane(&self, plane: &Plane) -> Option<(bool, f32)> {
        // First test if there's a solution for t
        let denom = Vector3::dot(&(self.end.clone() - self.start.clone()), &plane.normal);

        if math::basic::near_zero(denom, 0.001) {
            // The only way they intersect is if start
            // is a point on the plane (P dot N) == d
            if math::basic::near_zero(Vector3::dot(&self.start, &plane.normal) - plane.d, 0.001) {
                println!("denom near 0 -> number near 0");
                return Some((false, 0.0));
            }
            println!("denom near 0 -> number not near 0");
            return None;
        }

        let number = -Vector3::dot(&self.start, &plane.normal) - plane.d;
        let result = number / denom;

        // Validate t is within bounds of the line segment
        if result >= 0.0 && result <= 1.0 {
            println!("result 0-1, value = {result}");
            Some((true, result))
        } else {
            println!("result not 0-1, value = {result}");
            Some((false, result))
        }
    }

    pub fn intersect_sphere(&self, sphere: &Sphere) -> Option<f32> {
        // Compute X, Y, a, b, c as per equations
        let x = self.start.clone() - sphere.center.clone();
        let y = self.end.clone() - self.start.clone();
        let a = Vector3::dot(&y, &y);
        let b = 2.0 * Vector3::dot(&x, &y);
        let c = Vector3::dot(&x, &x) - sphere.radius * sphere.radius;

        // Compute discriminant
        let mut discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Compute min and max solutions of t
        discriminant = discriminant.sqrt();
        let t_min = (-b - discriminant) / (2.0 * a);
        let t_max = (-b + discriminant) / (2.0 * a);

        // Check whether either t is within bounds of segment
        if t_min > 0.0 && t_min <= 1.0 {
            return Some(t_min);
        }
        if t_max >= 0.0 && t_max <= 1.0 {
            return Some(t_max);
        }
        None
    }

    pub fn intersect_aabb(&self, aabb: &AABB) -> Option<(f32, Vector3)> {
        // Vector to save all possible t values for those sides
        let mut t_values = vec![];
        // Test the x planes
        LineSegment::test_side_plane(
            self.start.x,
            self.end.x,
            aabb.min.x,
            Vector3::NEGATIVE_UNIT_X,
            &mut t_values,
        );
        LineSegment::test_side_plane(
            self.start.x,
            self.end.x,
            aabb.max.x,
            Vector3::UNIT_X,
            &mut t_values,
        );
        // Test the y planes
        LineSegment::test_side_plane(
            self.start.y,
            self.end.y,
            aabb.min.y,
            Vector3::NEGATIVE_UNIT_Y,
            &mut t_values,
        );
        LineSegment::test_side_plane(
            self.start.y,
            self.end.y,
            aabb.max.y,
            Vector3::UNIT_Y,
            &mut t_values,
        );
        // Test the z planes
        LineSegment::test_side_plane(
            self.start.z,
            self.end.z,
            aabb.min.z,
            Vector3::NEGATIVE_UNIT_Z,
            &mut t_values,
        );
        LineSegment::test_side_plane(
            self.start.z,
            self.end.z,
            aabb.max.z,
            Vector3::UNIT_Z,
            &mut t_values,
        );

        // Sort the t values in ascending order
        t_values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Test if the box contains any of these points of intersection
        for t_and_normal in t_values {
            let point = self.point_on_segment(t_and_normal.0);
            if aabb.contains(&point) {
                return Some(t_and_normal);
            }
        }

        //None of the intersections are within bounds of box
        None
    }

    fn test_side_plane(
        start: f32,
        end: f32,
        negd: f32,
        norm: Vector3,
        out: &mut Vec<(f32, Vector3)>,
    ) -> bool {
        let denom = end - start;
        if math::basic::near_zero(denom, 0.001) {
            return false;
        }

        let numer = -start + negd;
        let t = numer / denom;
        // Test that t is within bounds
        if t >= 0.0 && t <= 1.0 {
            out.push((t, norm));
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        collision::{aabb::AABB, plane::Plane, sphere::Sphere},
        math::vector3::Vector3,
    };

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

    #[test]
    fn test_intersect_plane() {
        let expected = Some((true, 0.5));

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let plane = Plane::new(Vector3::new(0.0, -1.0, 0.0), 1.0);
        let actual = LineSegment::intersect_plane(&segment, &plane);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_intersect_sphere() {
        let expected = Some(0.5);

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(0.0, 2.0, 0.0));
        let sphere = Sphere::new(Vector3::new(1.0, 1.0, 0.0), 1.0);
        let actual = LineSegment::intersect_sphere(&segment, &sphere);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_intersect_aabb() {
        let expected = Some((0.5, Vector3::UNIT_Y));

        let segment = LineSegment::new(Vector3::ZERO, Vector3::new(0.0, 2.0, 0.0));
        let aabb = AABB::new(Vector3::new(-1.0, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0));
        let actual = LineSegment::intersect_aabb(&segment, &aabb);

        assert_eq!(expected, actual);
    }
}
