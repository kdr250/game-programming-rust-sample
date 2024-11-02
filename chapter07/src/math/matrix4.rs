use std::ops::{Mul, MulAssign};

use super::{basic, quaternion::Quaternion, vector3::Vector3};

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

    pub fn create_look_at(eye: &Vector3, target: &Vector3, up: &Vector3) -> Self {
        let zaxis = (target.clone() - eye.clone()).normalize();
        let xaxis = Vector3::cross(up, &zaxis).normalize();
        let yaxis = Vector3::cross(&zaxis, &xaxis).normalize();

        let trans = Vector3::new(
            -Vector3::dot(&xaxis, &eye),
            -Vector3::dot(&yaxis, &eye),
            -Vector3::dot(&zaxis, &eye),
        );

        let temp = [
            [xaxis.x, yaxis.x, zaxis.x, 0.0],
            [xaxis.y, yaxis.y, zaxis.y, 0.0],
            [xaxis.z, yaxis.z, zaxis.z, 0.0],
            [trans.x, trans.y, trans.z, 1.0],
        ];

        Matrix4::from(temp)
    }

    pub fn create_ortho(width: f32, height: f32, near: f32, far: f32) -> Self {
        let temp = [
            [2.0 / width, 0.0, 0.0, 0.0],
            [0.0, 2.0 / height, 0.0, 0.0],
            [0.0, 0.0, 1.0 / (far - near), 0.0],
            [0.0, 0.0, near / (near - far), 1.0],
        ];
        Matrix4::from(temp)
    }

    pub fn create_perspective_fov(
        fov_y: f32,
        width: f32,
        height: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let y_scale = basic::cot(fov_y / 2.0);
        let x_scale = y_scale * height / width;
        let temp = [
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, y_scale, 0.0, 0.0],
            [0.0, 0.0, far / (far - near), 1.0],
            [0.0, 0.0, -near * far / (far - near), 0.0],
        ];
        Matrix4::from(temp)
    }

    // Get the translation component of the matrix
    pub fn get_translation(&self) -> Vector3 {
        Vector3::new(self.mat[3][0], self.mat[3][1], self.mat[3][2])
    }

    // Invert the matrix - super slow
    pub fn invert(&mut self) {
        let mut tmp = [0.0; 12];
        let mut src = [0.0; 16];
        let mut dst = [0.0; 16];
        let mut det = 0.0;

        // Transpose matrix
        // row 1 to col 1
        src[0] = self.mat[0][0];
        src[4] = self.mat[0][1];
        src[8] = self.mat[0][2];
        src[12] = self.mat[0][3];

        // row 2 to col 2
        src[1] = self.mat[1][0];
        src[5] = self.mat[1][1];
        src[9] = self.mat[1][2];
        src[13] = self.mat[1][3];

        // row 3 to col 3
        src[2] = self.mat[2][0];
        src[6] = self.mat[2][1];
        src[10] = self.mat[2][2];
        src[14] = self.mat[2][3];

        // row 4 to col 4
        src[3] = self.mat[3][0];
        src[7] = self.mat[3][1];
        src[11] = self.mat[3][2];
        src[15] = self.mat[3][3];

        // Calculate cofactors
        tmp[0] = src[10] * src[15];
        tmp[1] = src[11] * src[14];
        tmp[2] = src[9] * src[15];
        tmp[3] = src[11] * src[13];
        tmp[4] = src[9] * src[14];
        tmp[5] = src[10] * src[13];
        tmp[6] = src[8] * src[15];
        tmp[7] = src[11] * src[12];
        tmp[8] = src[8] * src[14];
        tmp[9] = src[10] * src[12];
        tmp[10] = src[8] * src[13];
        tmp[11] = src[9] * src[12];

        dst[0] = tmp[0] * src[5] + tmp[3] * src[6] + tmp[4] * src[7];
        dst[0] -= tmp[1] * src[5] + tmp[2] * src[6] + tmp[5] * src[7];
        dst[1] = tmp[1] * src[4] + tmp[6] * src[6] + tmp[9] * src[7];
        dst[1] -= tmp[0] * src[4] + tmp[7] * src[6] + tmp[8] * src[7];
        dst[2] = tmp[2] * src[4] + tmp[7] * src[5] + tmp[10] * src[7];
        dst[2] -= tmp[3] * src[4] + tmp[6] * src[5] + tmp[11] * src[7];
        dst[3] = tmp[5] * src[4] + tmp[8] * src[5] + tmp[11] * src[6];
        dst[3] -= tmp[4] * src[4] + tmp[9] * src[5] + tmp[10] * src[6];
        dst[4] = tmp[1] * src[1] + tmp[2] * src[2] + tmp[5] * src[3];
        dst[4] -= tmp[0] * src[1] + tmp[3] * src[2] + tmp[4] * src[3];
        dst[5] = tmp[0] * src[0] + tmp[7] * src[2] + tmp[8] * src[3];
        dst[5] -= tmp[1] * src[0] + tmp[6] * src[2] + tmp[9] * src[3];
        dst[6] = tmp[3] * src[0] + tmp[6] * src[1] + tmp[11] * src[3];
        dst[6] -= tmp[2] * src[0] + tmp[7] * src[1] + tmp[10] * src[3];
        dst[7] = tmp[4] * src[0] + tmp[9] * src[1] + tmp[10] * src[2];
        dst[7] -= tmp[5] * src[0] + tmp[8] * src[1] + tmp[11] * src[2];

        tmp[0] = src[2] * src[7];
        tmp[1] = src[3] * src[6];
        tmp[2] = src[1] * src[7];
        tmp[3] = src[3] * src[5];
        tmp[4] = src[1] * src[6];
        tmp[5] = src[2] * src[5];
        tmp[6] = src[0] * src[7];
        tmp[7] = src[3] * src[4];
        tmp[8] = src[0] * src[6];
        tmp[9] = src[2] * src[4];
        tmp[10] = src[0] * src[5];
        tmp[11] = src[1] * src[4];

        dst[8] = tmp[0] * src[13] + tmp[3] * src[14] + tmp[4] * src[15];
        dst[8] -= tmp[1] * src[13] + tmp[2] * src[14] + tmp[5] * src[15];
        dst[9] = tmp[1] * src[12] + tmp[6] * src[14] + tmp[9] * src[15];
        dst[9] -= tmp[0] * src[12] + tmp[7] * src[14] + tmp[8] * src[15];
        dst[10] = tmp[2] * src[12] + tmp[7] * src[13] + tmp[10] * src[15];
        dst[10] -= tmp[3] * src[12] + tmp[6] * src[13] + tmp[11] * src[15];
        dst[11] = tmp[5] * src[12] + tmp[8] * src[13] + tmp[11] * src[14];
        dst[11] -= tmp[4] * src[12] + tmp[9] * src[13] + tmp[10] * src[14];
        dst[12] = tmp[2] * src[10] + tmp[5] * src[11] + tmp[1] * src[9];
        dst[12] -= tmp[4] * src[11] + tmp[0] * src[9] + tmp[3] * src[10];
        dst[13] = tmp[8] * src[11] + tmp[0] * src[8] + tmp[7] * src[10];
        dst[13] -= tmp[6] * src[10] + tmp[9] * src[11] + tmp[1] * src[8];
        dst[14] = tmp[6] * src[9] + tmp[11] * src[11] + tmp[3] * src[8];
        dst[14] -= tmp[10] * src[11] + tmp[2] * src[8] + tmp[7] * src[9];
        dst[15] = tmp[10] * src[10] + tmp[4] * src[8] + tmp[9] * src[9];
        dst[15] -= tmp[8] * src[9] + tmp[11] * src[10] + tmp[5] * src[8];

        // Calculate determinant
        det = src[0] * dst[0] + src[1] * dst[1] + src[2] * dst[2] + src[3] * dst[3];

        // Inverse of matrix is divided by determinant
        det = 1.0 / det;
        for j in 0..16 {
            dst[j] *= det;
        }

        // Set it back
        for i in 0..4 {
            for j in 0..4 {
                self.mat[i][j] = dst[i * 4 + j];
            }
        }
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
