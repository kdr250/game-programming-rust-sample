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
}
