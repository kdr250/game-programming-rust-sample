use std::ops::{Mul, MulAssign};

use super::vector2::Vector2;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix3 {
    pub mat: [[f32; 3]; 3],
}

impl Matrix3 {
    const IDENTITY: Matrix3 = Matrix3 {
        mat: [
            [1.0, 0.0, 0.0], //
            [0.0, 1.0, 0.0], //
            [0.0, 0.0, 1.0], //
        ],
    };

    pub fn new() -> Self {
        Matrix3::IDENTITY
    }

    pub fn from(in_mat: [[f32; 3]; 3]) -> Self {
        Self { mat: in_mat }
    }

    // Cast to a const float pointer
    pub fn get_as_float_ptr(&self) -> *const f32 {
        &self.mat[0][0]
    }

    // Create a scale matrix with x and y scales
    pub fn create_scale_xy(x_scale: f32, y_scale: f32) -> Self {
        let temp = [
            [x_scale, 0.0, 0.0], //
            [0.0, y_scale, 0.0], //
            [0.0, 0.0, 1.0],     //
        ];
        Matrix3::from(temp)
    }

    pub fn create_scale_vec2(scale_vector: &Vector2) -> Self {
        Matrix3::create_scale_xy(scale_vector.x, scale_vector.y)
    }

    pub fn create_scale(scale: f32) -> Self {
        Matrix3::create_scale_xy(scale, scale)
    }

    // Create a rotation matrix about the Z axis. theta is in radians
    pub fn create_rotation(theta: f32) -> Self {
        let temp = [
            [theta.cos(), theta.sin(), 0.0],
            [-theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];
        Matrix3::from(temp)
    }

    pub fn create_translation(trans: &Vector2) -> Self {
        let temp = [
            [1.0, 0.0, 0.0],         //
            [0.0, 1.0, 0.0],         //
            [trans.x, trans.y, 1.0], //
        ];
        Matrix3::from(temp)
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix3::new();
        for row in 0..3 {
            for column in 0..3 {
                let mut sum = 0.0;
                for i in 0..3 {
                    sum += self.mat[row][i] * rhs.mat[i][column];
                }
                result.mat[row][column] = sum;
            }
        }
        result
    }
}

impl MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Self) {
        for row in 0..3 {
            let original_row = self.mat[row].clone();
            for column in 0..3 {
                let mut sum = 0.0;
                for i in 0..3 {
                    sum += original_row[i] * rhs.mat[i][column];
                }
                self.mat[row][column] = sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{self, vector2::Vector2};

    use super::Matrix3;

    #[test]
    fn test_mul() {
        let expected = Matrix3::from([
            [-37.0, 18.0, 31.0],
            [-2.0, 24.0, 36.0],
            [-50.0, 26.0, -19.0],
        ]);

        let a = Matrix3::from([
            [1.0, -5.0, 3.0], //
            [0.0, -2.0, 6.0], //
            [7.0, 2.0, -4.0], //
        ]);
        let b = Matrix3::from([
            [-8.0, 6.0, 1.0], //
            [7.0, 0.0, -3.0], //
            [2.0, 4.0, 5.0],  //
        ]);
        let actual = a * b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_assign() {
        let expected = Matrix3::from([
            [-37.0, 18.0, 31.0],
            [-2.0, 24.0, 36.0],
            [-50.0, 26.0, -19.0],
        ]);

        let mut actual = Matrix3::from([
            [1.0, -5.0, 3.0], //
            [0.0, -2.0, 6.0], //
            [7.0, 2.0, -4.0], //
        ]);
        let b = Matrix3::from([
            [-8.0, 6.0, 1.0], //
            [7.0, 0.0, -3.0], //
            [2.0, 4.0, 5.0],  //
        ]);
        actual *= b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scale_xy() {
        let expected = Matrix3::from([
            [2.0, 0.0, 0.0], //
            [0.0, 3.0, 0.0], //
            [0.0, 0.0, 1.0], //
        ]);
        let actual = Matrix3::create_scale_xy(2.0, 3.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_scale_vec2() {
        let expected = Matrix3::from([
            [2.0, 0.0, 0.0], //
            [0.0, 3.0, 0.0], //
            [0.0, 0.0, 1.0], //
        ]);
        let actual = Matrix3::create_scale_vec2(&&Vector2::new(2.0, 3.0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_scale() {
        let expected = Matrix3::from([
            [2.0, 0.0, 0.0], //
            [0.0, 2.0, 0.0], //
            [0.0, 0.0, 1.0], //
        ]);
        let actual = Matrix3::create_scale(2.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_rotation() {
        let expected = Matrix3::from([
            [3.0_f32.sqrt() / 2.0, 0.5, 0.0],
            [-0.5, 3.0_f32.sqrt() / 2.0, 0.0],
            [0.0, 0.0, 1.0],
        ]);
        let theta = math::basic::to_radians(30.0);
        let actual = Matrix3::create_rotation(theta);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_translation() {
        let expected = Matrix3::from([
            [1.0, 0.0, 0.0], //
            [0.0, 1.0, 0.0], //
            [2.0, 3.0, 1.0], //
        ]);
        let actual = Matrix3::create_translation(&Vector2::new(2.0, 3.0));

        assert_eq!(expected, actual);
    }
}
