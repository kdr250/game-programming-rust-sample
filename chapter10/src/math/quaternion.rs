use super::{basic, vector3::Vector3};

#[derive(Debug, PartialEq, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new() -> Self {
        Quaternion::IDENTITY
    }

    /// This directly sets the quaternion components
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Construct the quaternion from an axis and angle
    /// It is assumed that axis is already normalized,
    /// and the angle is in radians
    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Self {
        let scalar = (angle / 2.0).sin();
        let x = axis.x * scalar;
        let y = axis.y * scalar;
        let z = axis.z * scalar;
        let w = (angle / 2.0).cos();
        Self { x, y, z, w }
    }

    pub fn set(&mut self, in_x: f32, in_y: f32, in_z: f32, in_w: f32) {
        self.x = in_x;
        self.y = in_y;
        self.z = in_z;
        self.w = in_w;
    }

    pub fn conjugate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize_mut(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    /// Normalize the provided quaternion
    pub fn normalize(&self) -> Quaternion {
        let mut result = self.clone();
        result.normalize_mut();
        result
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Quaternion, f: f32) -> Quaternion {
        let x = basic::lerp(self.x, other.x, f);
        let y = basic::lerp(self.y, other.y, f);
        let z = basic::lerp(self.z, other.z, f);
        let w = basic::lerp(self.w, other.w, f);

        let mut result = Quaternion::from_xyzw(x, y, z, w);
        result.normalize_mut();
        result
    }

    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Spherical Linear Interpolation
    pub fn slerp(&self, other: &Quaternion, f: f32) -> Quaternion {
        let row_cosm = Quaternion::dot(&self, other);
        let cosom = row_cosm.abs();

        let (scale0, mut scale1) = if cosom < 0.9999 {
            let omega = cosom.acos();
            let inv_sin = 1.0 / omega.sin();
            let scale0 = ((1.0 - f) * omega).sin() * inv_sin;
            let scale1 = (f * omega).sin() * inv_sin;
            (scale0, scale1)
        } else {
            (1.0 - f, f) // Use linear interpolation if the quaternions are collinear
        };

        if row_cosm < 0.0 {
            scale1 = -scale1;
        }

        let x = scale0 * self.x + scale1 * other.x;
        let y = scale0 * self.y + scale1 * other.y;
        let z = scale0 * self.z + scale1 * other.z;
        let w = scale0 * self.w + scale1 * other.w;

        let mut result = Quaternion::from_xyzw(x, y, z, w);
        result.normalize_mut();
        result
    }

    /// Concatenate. Rotate by q FOLLOWED BY p
    pub fn concatenate(&self, other: &Quaternion) -> Quaternion {
        // Vector component is:
        // ps * qv + qs * pv + pv x qv
        let qv = Vector3::new(self.x, self.y, self.z);
        let pv = Vector3::new(other.x, other.y, other.z);
        let new_vec = qv.clone() * other.w + pv.clone() * self.w + Vector3::cross(&pv, &qv);
        let Vector3 { x, y, z } = new_vec;

        // Scalar component is:
        // ps * qs - pv . qv
        let w = other.w * self.w - Vector3::dot(&pv, &qv);

        Quaternion::from_xyzw(x, y, z, w)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_near_eq, math::vector3::Vector3};

    use super::Quaternion;

    #[test]
    fn test_normalize() {
        let expected = Quaternion::from_xyzw(0.0695048049, 0.347524017, 0.625543237, 0.695048034);

        let mut actual = Quaternion::from_xyzw(0.1, 0.5, 0.9, 1.0);
        actual.normalize_mut();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_slerp() {
        let p = Quaternion::IDENTITY;
        let q =
            Quaternion::from_axis_angle(&Vector3::new(1.0, 0.0, 0.0), std::f32::consts::FRAC_PI_2);

        let actual = Quaternion::slerp(&p, &q, 0.5);

        assert_near_eq!(actual.x, 0.382683426, 0.000001);
        assert_near_eq!(actual.y, 0.0, 0.000001);
        assert_near_eq!(actual.z, 0.0, 0.000001);
        assert_near_eq!(actual.w, 0.923879564, 0.000001);
    }
}
