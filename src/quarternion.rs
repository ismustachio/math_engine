use crate::prelude::*;
use std::ops::{Mul, MulAssign};

#[derive(Default, Copy, Clone, Debug)]
/// Represents a hamiltonian quaternion having the form xi + yj + zk + w.
/// https://en.wikipedia.org/wiki/Quaternion
pub struct Quarternion {
    /// The x coordinate of the vector part.
    pub x: f32,
    /// The y coordinate of the vector part.
    pub y: f32,
    /// The z coordinate of the vector part.
    pub z: f32,
    /// The w coordinate is the scalar part.
    pub w: f32,
}

impl Quarternion {
    /// Returns a quaternion initialized with the floating point components x, y, z, and w.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the vector part.
    /// * `y` - The y coordinate of the vector part.
    /// * `z` - The z coordinate of the vector part.
    /// * `w` - The scalar part.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// let q = Quarternion::new(1.0,0.0,0.0,1.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quarternion {
        Quarternion { x, y, z, w }
    }

    /// Returns a quaternion initialized with the vector components assigned to x, y, and z,
    /// with the w coordinated set to s.
    ///
    /// # Arguments
    ///
    /// * `a` - A 3D vector.
    /// * `s` - The scalar part.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// use math_engine::vector3::Vector3;
    /// let q = Quarternion::new_with_vec_and_scalar(&Vector3::new(1.0,0.0,0.0),1.0);
    /// ```
    pub fn new_with_vec_and_scalar(a: &Vector3, s: f32) -> Quarternion {
        Quarternion {
            x: a.x,
            y: a.y,
            z: a.z,
            w: s,
        }
    }

    /// Returns a quaternion initialized with the vector components assigned to x, y, and z,
    /// with the w coordinated set to 0.
    ///
    /// # Arguments
    ///
    /// * `a` - A 3D vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// use math_engine::vector3::Vector3;
    /// let q = Quarternion::new_with_vec(&Vector3::new(1.0,0.0,0.0));
    /// ```
    pub fn new_with_vec(a: &Vector3) -> Quarternion {
        Quarternion {
            x: a.x,
            y: a.y,
            z: a.z,
            w: 0.0,
        }
    }

    /// Returns a quaternion initialized x, y, and z components to 0 and
    /// sets the w coordinated set to s.
    ///
    /// # Arguments
    ///
    /// * `s` - The scalar part.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// let q = Quarternion::new_with_scalar(1.0);
    /// ```
    pub fn new_with_scalar(s: f32) -> Quarternion {
        Quarternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: s,
        }
    }

    pub fn get_vector_part(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.x)
    }

    /// Returns a converted quaternion to a 3x3 matrix.
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// use math_engine::matrix3::Matrix3;
    /// let q = Quarternion::new_with_scalar(1.0);
    /// let m = q.get_rotation_matrix();
    /// ```
    pub fn get_rotation_matrix(&self) -> Matrix3 {
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

    /// Sets the component of the quaternion to represent the same rotation
    /// as specified by the matrix m.
    ///
    /// # Arguments
    ///
    /// * `m` - A 3x3 matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// use math_engine::matrix3::Matrix3;
    /// let mut q = Quarternion::new_with_scalar(1.0);
    /// q.set_rotation_matrix(&Matrix3::identity());
    /// ```
    pub fn set_rotation_matrix(&mut self, m: &Matrix3) {
        let m00 = m[(0, 0)];
        let m11 = m[(1, 1)];
        let m22 = m[(2, 2)];
        let sum = m00 + m11 + m22;

        if sum > 0.0 {
            self.w = f32::sqrt(sum + 1.0) * 0.5;
            let f = 0.25 / self.w;
            self.x = (m[(2, 1)] - m[(1, 2)]) * f;
            self.y = (m[(0, 2)] - m[(2, 0)]) * f;
            self.z = (m[(1, 0)] - m[(0, 1)]) * f;
        } else if (m00 > m11) && (m00 > m22) {
            self.x = f32::sqrt(m00 - m11 - m22 + 1.0) * 0.5;
            let f = 0.25 / self.x;
            self.y = (m[(1, 0)] + m[(0, 1)]) * f;
            self.z = (m[(0, 2)] + m[(2, 0)]) * f;
            self.w = (m[(2, 1)] - m[(1, 2)]) * f;
        } else if m11 > m22 {
            self.y = f32::sqrt(m11 - m00 - m22 + 1.0) * 0.5;
            let f = 0.25 / self.y;
            self.x = (m[(1, 0)] + m[(0, 1)]) * f;
            self.z = (m[(2, 1)] + m[(1, 2)]) * f;
            self.w = (m[(0, 2)] - m[(2, 0)]) * f;
        } else {
            self.z = f32::sqrt(m22 - m00 - m11 + 1.0) * 0.5;
            let f = 0.25 / self.z;
            self.x = (m[(0, 2)] + m[(2, 0)]) * f;
            self.y = (m[(2, 1)] + m[(1, 2)]) * f;
            self.w = (m[(1, 0)] - m[(0, 1)]) * f;
        }
    }

    /// Returns the transformation of the vector v with the quaternion.
    ///
    /// # Arguments
    ///
    /// * `v` - A 3D vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::quarternion::Quarternion;
    /// use math_engine::vector3::Vector3;
    /// let q = Quarternion::new_with_scalar(1.0);
    /// let v = q.transform(&Vector3::new(1.0,0.0,0.0));
    /// ```
    pub fn transform(&self, v: &Vector3) -> Vector3 {
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
