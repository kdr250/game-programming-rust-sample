use super::{vector2::Vector2, vector3::Vector3};
use rand::prelude::*;

pub struct Random {
    generator: ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        let rng = thread_rng();
        Self { generator: rng }
    }

    pub fn get_float(&mut self) -> f32 {
        self.generator.gen()
    }

    pub fn get_float_range(&mut self, min: f32, max: f32) -> f32 {
        self.generator.gen_range(min..=max)
    }

    pub fn get_vector2(&mut self, min: Vector2, max: Vector2) -> Vector2 {
        let random = Vector2::new(self.get_float(), self.get_float());
        min.clone() + (max - min) * random
    }

    pub fn get_vector3(&mut self, min: Vector3, max: Vector3) -> Vector3 {
        let random = Vector3::new(self.get_float(), self.get_float(), self.get_float());
        min.clone() + (max - min) * random
    }

    // TODO: Not yet implemented
}
