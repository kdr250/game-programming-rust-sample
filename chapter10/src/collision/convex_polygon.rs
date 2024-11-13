use core::f32;

use crate::math::{self, vector2::Vector2};

pub struct ConvexPolygon {
    // Vertices have a clockwise ordering
    vertices: Vec<Vector2>,
}

impl ConvexPolygon {
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self { vertices }
    }

    pub fn contains(&self, point: Vector2) -> bool {
        // Not effecient...
        let mut sum = 0.0;
        let mut a = Vector2::ZERO;
        let mut b = Vector2::ZERO;

        for i in 0..self.vertices.len() - 1 {
            // From point to first vertex
            a = self.vertices[i].clone() - point.clone();
            a.normalize_mut();
            // From point to second vertex
            b = self.vertices[i + 1].clone() - point.clone();
            b.normalize_mut();
            // Add angle to sum
            sum += Vector2::dot(&a, &b).acos();
        }

        // Have to ad angle for last vertex and first vertex
        a = self.vertices.last().unwrap().clone() - point.clone();
        a.normalize_mut();
        b = self.vertices.first().unwrap().clone() - point.clone();
        b.normalize_mut();
        sum += Vector2::dot(&a, &b);

        // Return true if approximately 2pi
        math::basic::near_zero(sum - f32::consts::TAU, 0.001)
    }
}
