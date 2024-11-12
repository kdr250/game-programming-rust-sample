use crate::math::vector2::Vector2;

pub struct ConvexPolygon {
    // Vertices have a clockwise ordering
    vertices: Vec<Vector2>,
}
