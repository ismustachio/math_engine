use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A three dimensional direction vector having float components
/// x, y, and z. It's w coordinated it's assumed to be 0.
pub struct Vector3 {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
    /// The z component.
    pub z: f32,
}

impl Vector3 {
    /// Returns a directional vector initialized with the floating point components x, y, and z.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinates.
    /// * `y` - The y coordinates.
    /// * `z` - The z coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let vec3 = Vector3::new(1.0,0.0,0.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Self { x, y, z }
    }

    /// Returns the dot product between this vector and other.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector3.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,0.0,0.0);
    /// let v2 = Vector3::new(1.0,0.0,1.0);
    /// let d = v1.dot(&v2);
    /// ```
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the length of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v = Vector3::new(1.0,0.0,0.0);
    /// let length = v.magnitude();
    /// ```
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Returns the cross product between this vector and other.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector3.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,0.0,0.0);
    /// let v2 = Vector3::new(1.0,0.0,1.0);
    /// let v3 = v1.cross(&v2);
    /// ```
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.z,
        )
    }

    /// Returns the projection of this vector onto other, under
    /// the assumption that magnitude of other is 1.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector3.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,0.0,0.0);
    /// let v2 = Vector3::new(1.0,0.0,1.0);
    /// let v3 = v1.project(&v2);
    /// ```
    pub fn project(&self, other: &Vector3) -> Vector3 {
        *other * self.dot(other)
    }

    /// Returns the rejection of this vector from other, under
    /// the assumption that magnitude of other is 1.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector3.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,0.0,0.0);
    /// let v2 = Vector3::new(1.0,0.0,1.0);
    /// let v3 = v1.reject(&v2);
    /// ```
    pub fn reject(&self, other: &Vector3) -> Vector3 {
        *self - *other * self.dot(other)
    }

    /// Returns the result of reflecting this vector around other
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector3.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,0.0,0.0);
    /// let v2 = Vector3::new(1.0,0.0,1.0);
    /// let v3 = v1.reflect(&v2);
    /// ```
    pub fn reflect(&self, other: &Vector3) -> Vector3 {
        (*self - *other) * 2.0 * self.dot(other)
    }

    /// Returns this vector multiplied by the inverse of it's magnitude
    /// normalizing to unit length.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let v1 = Vector3::new(1.0,2.0,3.0);
    /// let v2 = v1.normalize();
    /// ```
    pub fn normalize(&self) -> Vector3 {
        *self / self.magnitude()
    }

    /// Multiplies this vector by the inverse of it's magnitude
    /// normalizing to unit length.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector3::Vector3;
    /// let mut v = Vector3::new(1.0,2.0,3.0);
    /// v.normalize_mut();
    /// ```
    pub fn normalize_mut(&mut self) {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self.z /= m;
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < 3);
        if i == 0 {
            return &self.x;
        } else if i == 1 {
            return &self.y;
        }
        &self.z
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        assert!(i < 3);
        if i == 0 {
            return &mut self.x;
        } else if i == 1 {
            return &mut self.y;
        }
        &mut self.z
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        let s = 1.0 / other;
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Vector3) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<Matrix3> for Vector3 {
    type Output = Self;

    fn mul(self, other: Matrix3) -> Self::Output {
        Self::new(
            other[(0, 0)] * self.x + other[(0, 1)] * self.y + other[(0, 2)] * self.z,
            other[(1, 0)] * self.x + other[(1, 1)] * self.y + other[(1, 2)] * self.z,
            other[(2, 0)] * self.x + other[(2, 1)] * self.y + other[(2, 2)] * self.z,
        )
    }
}

impl MulAssign<Matrix3> for Vector3 {
    fn mul_assign(&mut self, other: Matrix3) {
        self.x = other[(0, 0)] * self.x + other[(0, 1)] * self.y + other[(0, 2)] * self.z;
        self.y = other[(1, 0)] * self.x + other[(1, 1)] * self.y + other[(1, 2)] * self.z;
        self.z = other[(2, 0)] * self.x + other[(2, 1)] * self.y + other[(2, 2)] * self.z;
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Self {
        Vector3 {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

#[test]
fn add() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(2.0, 3.0, 4.0);
    let result = a + b;
    let want = Vector3::new(3.0, 5.0, 7.0);
    assert!((a + b) == want);
}
