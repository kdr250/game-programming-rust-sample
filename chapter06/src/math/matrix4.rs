use std::ops::{Mul, MulAssign};

use super::{quaternion::Quaternion, vector3::Vector3};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix4 {
    pub mat: [[f32; 4]; 4],
}

impl Matrix4 {
    const IDENTITY: Matrix4 = Matrix4 {
        mat: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn new() -> Self {
        Matrix4::IDENTITY
    }

    pub fn from(in_mat: [[f32; 4]; 4]) -> Self {
        Self { mat: in_mat }
    }

    // Cast to a const float pointer
    pub fn get_as_float_ptr(&self) -> *const f32 {
        &self.mat[0][0]
    }

    // Create a scale matrix with x, y, and z scales
    pub fn create_scale_xyz(x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        let temp = [
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, y_scale, 0.0, 0.0],
            [0.0, 0.0, z_scale, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4::from(temp)
    }

    pub fn create_scale_vec3(scale_vector: &Vector3) -> Self {
        Matrix4::create_scale_xyz(scale_vector.x, scale_vector.y, scale_vector.z)
    }

    pub fn create_scale(scale: f32) -> Self {
        Matrix4::create_scale_xyz(scale, scale, scale)
    }

    // Rotation about x-axis
    pub fn create_rotation_x(theta: f32) -> Self {
        let temp = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), theta.sin(), 0.0],
            [0.0, -theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4::from(temp)
    }

    // Rotation about y-axis
    pub fn create_rotation_y(theta: f32) -> Self {
        let temp = [
            [theta.cos(), 0.0, -theta.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4::from(temp)
    }

    // Rotation about z-axis
    pub fn create_rotation_z(theta: f32) -> Self {
        let temp = [
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4::from(temp)
    }

    pub fn create_translation(trans: &Vector3) -> Self {
        let temp = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [trans.x, trans.y, trans.z, 1.0],
        ];
        Matrix4::from(temp)
    }

    // Create "Simple" View-Projection Matrix from Chapter 5
    pub fn create_simple_view_proj(width: f32, height: f32) -> Self {
        let temp = [
            [2.0 / width, 0.0, 0.0, 0.0],
            [0.0, 2.0 / height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 1.0],
        ];
        Matrix4::from(temp)
    }

    /// Create rotation matrix from quaternion
    pub fn create_from_quaternion(q: &Quaternion) -> Self {
        let mut mat = [[0.0; 4]; 4];

        mat[0][0] = 1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z;
        mat[0][1] = 2.0 * q.x * q.y + 2.0 * q.w * q.z;
        mat[0][2] = 2.0 * q.x * q.z - 2.0 * q.w * q.y;
        mat[0][3] = 0.0;

        mat[1][0] = 2.0 * q.x * q.y - 2.0 * q.w * q.z;
        mat[1][1] = 1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z;
        mat[1][2] = 2.0 * q.y * q.z + 2.0 * q.w * q.x;
        mat[1][3] = 0.0;

        mat[2][0] = 2.0 * q.x * q.z + 2.0 * q.w * q.y;
        mat[2][1] = 2.0 * q.y * q.z - 2.0 * q.w * q.x;
        mat[2][2] = 1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y;
        mat[2][3] = 0.0;

        mat[3][0] = 0.0;
        mat[3][1] = 0.0;
        mat[3][2] = 0.0;
        mat[3][3] = 1.0;

        Self { mat }
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix4::new();
        for row in 0..4 {
            for column in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += self.mat[row][i] * rhs.mat[i][column];
                }
                result.mat[row][column] = sum;
            }
        }
        result
    }
}

impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        for row in 0..4 {
            let original_row = self.mat[row].clone();
            for column in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += original_row[i] * rhs.mat[i][column];
                }
                self.mat[row][column] = sum;
            }
        }
    }
}
