use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);
    pub const UNIT_X: Vector2 = Vector2::new(1.0, 0.0);
    pub const UNIT_Y: Vector2 = Vector2::new(0.0, 1.0);
    pub const NEGATIVE_UNIT_X: Vector2 = Vector2::new(-1.0, 0.0);
    pub const NEGATIVE_UNIT_Y: Vector2 = Vector2::new(0.0, -1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn length_sqrt(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_sqrt().sqrt()
    }

    pub fn normalize_mut(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
    }

    pub fn normalize(&self) -> Vector2 {
        let mut temp = self.clone();
        temp.normalize_mut();
        temp
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Component-wise multiplication
impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_near_eq,
        math::{self, vector2::Vector2},
    };

    #[test]
    fn test_add() {
        let expected = Vector2::new(4.0, 6.0);

        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let actual = a + b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add_assign() {
        let expected = Vector2::new(4.0, 6.0);

        let mut actual = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        actual += b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub() {
        let expected = Vector2::new(2.0, 1.0);

        let a = Vector2::new(3.0, 3.0);
        let b = Vector2::new(1.0, 2.0);
        let actual = a - b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_assign() {
        let expected = Vector2::new(2.0, 1.0);

        let mut actual = Vector2::new(3.0, 3.0);
        let b = Vector2::new(1.0, 2.0);
        actual -= b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_vector() {
        let expected = Vector2::new(3.0, 6.0);

        let a = Vector2::new(3.0, 3.0);
        let b = Vector2::new(1.0, 2.0);
        let actual = a * b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_scalar() {
        let expected = Vector2::new(10.0, 6.0);

        let a = Vector2::new(5.0, 3.0);
        let actual = a * 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_assign_scalar() {
        let expected = Vector2::new(10.0, 6.0);

        let mut actual = Vector2::new(5.0, 3.0);
        actual *= 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length_sqrt() {
        let expected = 25.0;

        let a = Vector2::new(3.0, 4.0);
        let actual = a.length_sqrt();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length() {
        let expected = 5.0;

        let a = Vector2::new(3.0, 4.0);
        let actual = a.length();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_normalize_mut() {
        let expected = Vector2::new(1.0 / 2.0, 3.0_f32.sqrt() / 2.0);

        let mut actual = Vector2::new(1.0, 3.0_f32.sqrt());
        actual.normalize_mut();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_normalize() {
        let expected = Vector2::new(1.0 / 2.0, 3.0_f32.sqrt() / 2.0);

        let a = Vector2::new(1.0, 3.0_f32.sqrt());
        let actual = a.normalize();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_dot() {
        let expected = math::basic::to_radians(45.0);

        let mut a = Vector2::new(1.0, 2.0);
        let mut b = Vector2::new(3.0, 1.0);
        a.normalize_mut();
        b.normalize_mut();
        let dot_result = Vector2::dot(&a, &b);
        let angle = dot_result.acos();

        assert_near_eq!(expected, angle, 0.001);
    }
}
