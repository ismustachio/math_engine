use crate::prelude::*;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Default, Copy, Clone, Debug)]
/// A three dimensional direction vector having float components
/// x, and y. It's w coordinated it's assumed to be 0.
pub struct Vector2 {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
}

impl Vector2 {
    /// Returns a vector initialized with the floating point components x, and y.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinates.
    /// * `y` - The y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v = Vector2::new(1.0,0.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Vector2 {
        Self { x, y }
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
    /// use math_engine::vector2::Vector2;
    /// let v1 = Vector2::new(1.0,0.0);
    /// let v2 = Vector2::new(1.0,0.0);
    /// let d = v1.dot(&v2);
    /// ```
    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the length of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v = Vector2::new(1.0,0.0);
    /// let length = v.magnitude();
    /// ```
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Returns the projection of this vector onto other, under
    /// the assumption that magnitude of other is 1.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector2.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v1 = Vector2::new(1.0,0.0);
    /// let v2 = Vector2::new(1.0,0.0);
    /// let v3 = v1.project(&v2);
    /// ```
    pub fn project(&self, other: &Vector2) -> Vector2 {
        *other * self.dot(other)
    }

    /// Returns the rejection of this vector from other, under
    /// the assumption that magnitude of other is 1.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector2.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v1 = Vector2::new(1.0,0.0);
    /// let v2 = Vector2::new(1.0,0.0);
    /// let v3 = v1.reject(&v2);
    /// ```
    pub fn reject(&self, other: &Vector2) -> Vector2 {
        *self - *other * self.dot(other)
    }

    /// Returns the result of reflecting this vector around other
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to a vector2.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v1 = Vector2::new(1.0,0.0);
    /// let v2 = Vector2::new(1.0,0.0);
    /// let v3 = v1.reflect(&v2);
    /// ```
    pub fn reflect(&self, other: &Vector2) -> Vector2 {
        (*self - *other) * 2.0 * self.dot(other)
    }

    /// Returns this vector multiplied by the inverse of it's magnitude
    /// normalizing to unit length.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let v1 = Vector2::new(1.0,2.0);
    /// let v2 = v1.normalize();
    /// ```
    pub fn normalize(&self) -> Vector2 {
        *self / self.magnitude()
    }

    /// Multiplies this vector by the inverse of it's magnitude
    /// normalizing to unit length.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::vector2::Vector2;
    /// let mut v = Vector2::new(1.0,2.0);
    /// v.normalize_mut();
    /// ```
    pub fn normalize_mut(&mut self) {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < 2);
        if i == 0 {
            return &self.x;
        }
        &self.y
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        assert!(i < 2);
        if i == 0 {
            return &mut self.x;
        }
        &mut self.y
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vector2::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;

    fn mul(self, other: Vector2) -> Self::Output {
        Vector2::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<Matrix2> for Vector2 {
    type Output = Self;

    fn mul(self, other: Matrix2) -> Self::Output {
        Vector2::new(
            other[(0, 0)] * self.x + other[(0, 1)] * self.y,
            other[(1, 0)] * self.x + other[(1, 1)] * self.y,
        )
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Vector2::new(self.x / other, self.y / other)
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, other: Vector2) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, other: Vector2) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}
