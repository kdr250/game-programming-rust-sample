use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::quaternion::Quaternion;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    pub const UNIT_X: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    pub const NEGATIVE_UNIT_X: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
    pub const NEGATIVE_UNIT_Y: Vector3 = Vector3::new(0.0, -1.0, 0.0);
    pub const NEGATIVE_UNIT_Z: Vector3 = Vector3::new(0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn transform(&self, q: &Quaternion) -> Vector3 {
        // v + 2.0*cross(q.xyz, cross(q.xyz,v) + q.w*v);
        let qv = Vector3::new(q.x, q.y, q.z);
        let mut result = qv.clone();
        result += Vector3::cross(&qv, &(Vector3::cross(&qv, &self) + self.clone() * q.w)) * 2.0;
        result
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn length_sqrt(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_sqrt().sqrt()
    }

    pub fn normalize_mut(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalize(&self) -> Vector3 {
        let mut temp = self.clone();
        temp.normalize_mut();
        temp
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        let mut temp = Vector3::ZERO;
        temp.x = self.y * other.z - self.z * other.y;
        temp.y = self.z * other.x - self.x * other.z;
        temp.z = self.x * other.y - self.y * other.x;
        temp
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Component-wise multiplication
impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_near_eq, math::vector3::Vector3};

    #[test]
    fn test_add() {
        let expected = Vector3::new(4.0, 6.0, 8.0);

        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(3.0, 4.0, 5.0);
        let actual = a + b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add_assign() {
        let expected = Vector3::new(4.0, 6.0, 8.0);

        let mut actual = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(3.0, 4.0, 5.0);
        actual += b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub() {
        let expected = Vector3::new(2.0, 1.0, 0.0);

        let a = Vector3::new(3.0, 3.0, 3.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let actual = a - b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_assign() {
        let expected = Vector3::new(2.0, 1.0, 0.0);

        let mut actual = Vector3::new(3.0, 3.0, 3.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        actual -= b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_vector() {
        let expected = Vector3::new(3.0, 6.0, 9.0);

        let a = Vector3::new(3.0, 3.0, 3.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let actual = a * b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_scalar() {
        let expected = Vector3::new(10.0, 6.0, 2.0);

        let a = Vector3::new(5.0, 3.0, 1.0);
        let actual = a * 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_assign_scalar() {
        let expected = Vector3::new(10.0, 6.0, 2.0);

        let mut actual = Vector3::new(5.0, 3.0, 1.0);
        actual *= 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length_sqrt() {
        let expected = 50.0;

        let a = Vector3::new(3.0, 4.0, 5.0);
        let actual = a.length_sqrt();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length() {
        let expected = 50.0_f32.sqrt();

        let a = Vector3::new(3.0, 4.0, 5.0);
        let actual = a.length();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_normalize_mut() {
        let expected = Vector3::new(
            1.0 / 3.0_f32.sqrt(),
            1.0 / 3.0_f32.sqrt(),
            1.0 / 3.0_f32.sqrt(),
        );

        let mut actual = Vector3::new(3.0, 3.0, 3.0);
        actual.normalize_mut();

        assert_near_eq!(expected.x, actual.x, 0.001);
        assert_near_eq!(expected.y, actual.y, 0.001);
        assert_near_eq!(expected.z, actual.z, 0.001);
    }

    #[test]
    fn test_normalize() {
        let expected = Vector3::new(
            1.0 / 3.0_f32.sqrt(),
            1.0 / 3.0_f32.sqrt(),
            1.0 / 3.0_f32.sqrt(),
        );

        let a = Vector3::new(3.0, 3.0, 3.0);
        let actual = a.normalize();

        assert_near_eq!(expected.x, actual.x, 0.001);
        assert_near_eq!(expected.y, actual.y, 0.001);
        assert_near_eq!(expected.z, actual.z, 0.001);
    }

    #[test]
    fn test_dot() {
        let expected = 20.0;

        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(3.0, 1.0, 5.0);
        let actual = Vector3::dot(&a, &b);

        assert_near_eq!(expected, actual, 0.001);
    }

    #[test]
    fn test_cross() {
        let expected = Vector3::new(-1.0, 2.0, -1.0);

        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(2.0, 3.0, 4.0);
        let actual = Vector3::cross(&a, &b);

        assert_eq!(expected, actual);
    }
}
