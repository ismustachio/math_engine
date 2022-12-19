use crate::prelude::*;
use std::ops::{Mul, MulAssign};

pub struct Quarternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quarternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quarternion {
        Quarternion { x, y, z, w }
    }

    fn new_with_vec(a: &Vector3, s: f32) -> Quarternion {
        Quarternion {
            x: a.x,
            y: a.y,
            z: a.z,
            w: s,
        }
    }

    fn get_vector_part(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.x)
    }

    fn get_rotation_matrix(&self) -> Matrix3 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;
        Matrix3::new(
            1.0 - 2.0 * (y2 + z2),
            2.0 * (xy - wz),
            2.0 * (xz + wy),
            2.0 * (xy + wz),
            1.0 - 2.0 * (x2 + z2),
            2.0 * (yz - wx),
            2.0 * (xz - wy),
            2.0 * (yz + wx),
            1.0 - 2.0 * (x2 + y2),
        )
    }

    fn set_rotation_matrix(&mut self, matrix: &Matrix3) {
        let m00 = matrix[(0, 0)];
        let m11 = matrix[(1, 1)];
        let m22 = matrix[(2, 2)];
        let sum = m00 + m11 + m22;

        if sum > 0.0 {
            self.w = f32::sqrt(sum + 1.0) * 0.5;
            let f = 0.25 / self.w;
            self.x = (matrix[(2, 1)] - matrix[(1, 2)]) * f;
            self.y = (matrix[(0, 2)] - matrix[(2, 0)]) * f;
            self.z = (matrix[(1, 0)] - matrix[(0, 1)]) * f;
        } else if (m00 > m11) && (m00 > m22) {
            self.x = f32::sqrt(m00 - m11 - m22 + 1.0) * 0.5;
            let f = 0.25 / self.x;
            self.y = (matrix[(1, 0)] + matrix[(0, 1)]) * f;
            self.z = (matrix[(0, 2)] + matrix[(2, 0)]) * f;
            self.w = (matrix[(2, 1)] - matrix[(1, 2)]) * f;
        } else if m11 > m22 {
            self.y = f32::sqrt(m11 - m00 - m22 + 1.0) * 0.5;
            let f = 0.25 / self.y;
            self.x = (matrix[(1, 0)] + matrix[(0, 1)]) * f;
            self.z = (matrix[(2, 1)] + matrix[(1, 2)]) * f;
            self.w = (matrix[(0, 2)] - matrix[(2, 0)]) * f;
        } else {
            self.z = f32::sqrt(m22 - m00 - m11 + 1.0) * 0.5;
            let f = 0.25 / self.z;
            self.x = (matrix[(0, 2)] + matrix[(2, 0)]) * f;
            self.y = (matrix[(2, 1)] + matrix[(1, 2)]) * f;
            self.w = (matrix[(1, 0)] - matrix[(0, 1)]) * f;
        }
    }

    fn transform(&self, v: &Vector3) -> Vector3 {
        let b = self.get_vector_part();
        let b2 = b.x * b.x + b.y * b.y + b.z * b.z;
        *v * (self.w * self.w - b2) + b * (v.dot(&b) * 2.0) + b.cross(&v) * (self.w * 2.0)
    }
}

impl Mul<Quarternion> for Quarternion {
    type Output = Self;

    fn mul(self, rhs: Quarternion) -> Self::Output {
        Quarternion::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl Default for Quarternion {
    fn default() -> Quarternion {
        Quarternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl MulAssign<Quarternion> for Quarternion {
    fn mul_assign(&mut self, rhs: Quarternion) {
        self.x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        self.y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        self.z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
        self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
    }
}

impl MulAssign<f32> for Quarternion {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
