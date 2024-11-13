use crate::math::{quaternion::Quaternion, vector3::Vector3};

pub struct AABB {
    min: Vector3,
    max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }

    pub fn update_min_max(&mut self, point: Vector3) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    pub fn rotate(&mut self, q: &Quaternion) {
        // Construct the 8 points for the corners of the box
        let mut points = [Vector3::ZERO; 8];

        // Min point is always a corner
        points[0] = self.min.clone();

        // Permutations with 2 min and 1 max
        points[1] = Vector3::new(self.min.x, self.min.y, self.max.z);
        points[2] = Vector3::new(self.min.x, self.max.y, self.min.z);
        points[3] = Vector3::new(self.max.x, self.min.y, self.min.z);

        // Permutations with 2 max and 1 min
        points[4] = Vector3::new(self.max.x, self.max.y, self.min.z);
        points[5] = Vector3::new(self.max.x, self.min.y, self.max.z);
        points[6] = Vector3::new(self.min.x, self.max.y, self.max.z);

        // Max point corner
        points[7] = self.max.clone();

        // Rotate first point
        let mut p = Vector3::transform(&points[0], &q);

        // Reset min/max to first point rotated
        self.min = p.clone();
        self.max = p.clone();

        // Update min/max based on remaining points, rotated
        for i in 1..points.len() {
            p = Vector3::transform(&points[i], &q);
            self.update_min_max(p);
        }
    }

    pub fn contains(&self, point: &Vector3) -> bool {
        let outside = point.x < self.min.x
            || point.y < self.min.y
            || point.z < self.min.z
            || point.x > self.max.x
            || point.y > self.max.y
            || point.x > self.max.z;

        !outside
    }

    pub fn intersect(&self, other: &AABB) -> bool {
        let no = self.max.x < other.min.x
            || self.max.y < other.min.y
            || self.max.z < other.min.z
            || other.max.x < self.min.x
            || other.max.y < self.min.y
            || other.max.z < self.min.z;

        !no
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    use super::AABB;

    #[test]
    fn test_contains() {
        let aabb = AABB::new(Vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
        let actual = aabb.contains(&Vector3::new(0.8, 0.8, 0.8));

        assert!(actual);
    }

    #[test]
    fn test_not_contains() {
        let aabb = AABB::new(Vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
        let actual = aabb.contains(&Vector3::new(1.8, 1.8, 1.8));

        assert!(!actual);
    }

    #[test]
    fn test_intersect() {
        let a = AABB::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let b = AABB::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(3.0, 3.0, 3.0));
        let actual = AABB::intersect(&a, &b);

        assert!(actual);
    }

    #[test]
    fn test_not_intersect() {
        let a = AABB::new(Vector3::ZERO, Vector3::new(2.0, 2.0, 2.0));
        let b = AABB::new(Vector3::new(3.0, 1.0, 1.0), Vector3::new(4.0, 3.0, 3.0));
        let actual = AABB::intersect(&a, &b);

        assert!(!actual);
    }
}
