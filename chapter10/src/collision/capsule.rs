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
}
