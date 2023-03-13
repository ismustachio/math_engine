use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

use crate::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A 2x2 matrix.
pub struct Matrix2 {
    /// The column entries of the matrix.
    pub n: [Vector2; 2],
}

impl Matrix2 {
    /// Returns a matrix initialized with the four entries supplied, with the
    /// nij parameter specifies the entry in i-th row and j-th column.
    ///
    /// # Arguments
    ///
    /// * `nij` - The value of the entry in row i and column j.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::new(1.0,0.0,0.0,1.0);
    /// ```
    pub fn new(n00: f32, n01: f32, n10: f32, n11: f32) -> Matrix2 {
        let n: [Vector2; 2] = [Vector2::new(n00, n10), Vector2::new(n01, n11)];
        Self { n }
    }

    /// Returns a matrix initialized with the two vectors initialize as the two
    /// columns of the matrix.
    ///
    /// # Arguments
    ///
    /// * `a` - The value of the entry in the first column.
    /// * `b` - The value of the entry in the second column.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let m = Matrix2::new_with_vecs(Vector2::new(1.0,0.0),Vector2::new(0.0,1.0));
    /// ```
    pub fn new_with_vecs(a: Vector2, b: Vector2) -> Matrix2 {
        let n: [Vector2; 2] = [a, b];
        Self { n }
    }

    pub fn vec_at(&self, i: usize) -> Vector2 {
        self[i]
    }

    pub fn at(&self, i: usize, j: usize) -> f32 {
        self[j][i]
    }

    /// Sets all four entries of this matrix, with the
    /// nij parameter specifies the entry in i-th row and j-th column.
    ///
    /// # Arguments
    ///
    /// * `nij` - The value of the entry residing in row i and column j.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let mut m = Matrix2::new(1.0,1.0,1.0,1.0);
    /// m.set(1.0,0.0,1.0,0.0);
    /// ```
    pub fn set(&mut self, n00: f32, n01: f32, n10: f32, n11: f32) {
        self[0][0] = n00;
        self[1][0] = n01;
        self[0][1] = n10;
        self[1][1] = n11;
    }

    /// Sets all four entries of this matrix to the two column vectors given.
    ///
    /// # Arguments
    ///
    /// * `a` - The value of the entry in the first column.
    /// * `b` - The value of the entry in the second column.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let mut m = Matrix2::new(1.0,1.0,1.0,1.0);
    /// m.set_vecs(Vector2::new(1.0,0.0),Vector2::new(1.0,0.0));
    /// ```
    pub fn set_vecs(&mut self, a: Vector2, b: Vector2) {
        self[0] = a;
        self[1] = b;
    }

    /// Sets this matrix to the 2x2 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let mut m = Matrix2::new_with_vecs(Vector2::new(1.0,1.0),Vector2::new(1.0,1.0));
    /// m.set_identity();
    /// ```
    pub fn set_identity(&mut self) {
        self[0][0] = 1.0;
        self[1][0] = 0.0;
        self[0][1] = 0.0;
        self[1][1] = 1.0;
    }

    /// Returns the determinant of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let m = Matrix2::new_with_vecs(Vector2::new(1.0,0.0),Vector2::new(0.0,1.0));
    /// let det = m.determinant();
    /// ```
    pub fn determinant(&self) -> f32 {
        self.n[0][0] * self.n[1][1] - self.n[1][0] * self.n[0][1]
    }

    /// Returns the inverse of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let m = Matrix2::new_with_vecs(Vector2::new(1.0,0.0),Vector2::new(0.0,1.0));
    /// let m2 = m.inverse();
    /// ```
    pub fn inverse(&self) -> Matrix2 {
        let inv = 1.0 / self.determinant();
        Self::new(
            self.n[1][1] * inv,
            -self.n[1][0] * inv,
            -self.n[0][1] * inv,
            self.n[0][0] * inv,
        )
    }

    /// Returns the transpose of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// use math_engine::vector2::Vector2;
    /// let m = Matrix2::new_with_vecs(Vector2::new(1.0,0.0),Vector2::new(0.0,1.0));
    /// let m2 = m.transpose();
    /// ```
    pub fn transpose(&self) -> Matrix2 {
        Self::new(self.n[0][0], self.n[0][1], self.n[1][0], self.n[1][1])
    }

    /// Returns 2x2 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::identity();
    /// ```
    pub fn identity() -> Matrix2 {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that represents a rotation through the angle given.
    ///
    /// # Arguments
    ///
    /// * `a` - The angle through which to rotate, in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::make_rotation(2.5);
    /// ```
    pub fn make_rotation(a: f32) -> Matrix2 {
        let c = a.cos();
        let s = a.sin();
        Self::new(c, -s, s, -c)
    }

    /// Returns a matrix that represents a scale along the x axis.
    ///
    /// # Arguments
    ///
    /// * `sx` - The scale along the x axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::make_scale_x(2.5);
    /// ```
    pub fn make_scale_x(sx: f32) -> Matrix2 {
        Self::new(sx, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that represents a scale along the y axis.
    ///
    /// # Arguments
    ///
    /// * `sy` - The scale along the y axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::make_scale_y(2.5);
    /// ```
    pub fn make_scale_y(sy: f32) -> Matrix2 {
        Self::new(1.0, 0.0, 0.0, sy)
    }

    /// Returns a matrix that represents a scale along the both x and y axis.
    ///
    /// # Arguments
    ///
    /// * `s` - The scale along both the x and y axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::make_scale(2.5);
    /// ```
    pub fn make_scale(s: f32) -> Matrix2 {
        Self::new(s, 0.0, 0.0, s)
    }

    /// Returns a matrix that represents a scale along the both x and y axis.
    ///
    /// # Arguments
    ///
    /// * `sx` - The scale along the x axis.
    /// * `sy` - The scale along the y axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix2::Matrix2;
    /// let m = Matrix2::make_scale_xy(2.5, 2.5);
    /// ```
    pub fn make_scale_xy(sx: f32, sy: f32) -> Matrix2 {
        Self::new(sx, 0.0, 0.0, sy)
    }
}

impl Index<(usize, usize)> for Matrix2 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(col < 2 && row < 2);
        &self.n[col][row]
    }
}

impl Index<usize> for Matrix2 {
    type Output = Vector2;
    fn index(&self, col: usize) -> &Self::Output {
        assert!(col < 2);
        &self.n[col]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, col: usize) -> &mut Vector2 {
        assert!(col < 2);
        &mut self.n[col]
    }
}

impl IndexMut<(usize, usize)> for Matrix2 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        assert!(col < 2 && row < 2);
        &mut self.n[col][row]
    }
}

impl Mul<f32> for Matrix2 {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::new(
            self.n[0][0] * s,
            self.n[1][0] * s,
            self.n[0][1] * s,
            self.n[1][1] * s,
        )
    }
}

impl Mul<Matrix2> for Matrix2 {
    type Output = Self;

    fn mul(self, other: Matrix2) -> Self::Output {
        Self::new(
            self.n[0][0] * other[(0, 0)] + self.n[0][1] * other[(1, 0)],
            self.n[0][0] * other[(0, 1)] + self.n[0][1] * other[(1, 1)],
            self.n[1][0] * other[(0, 0)] + self.n[1][1] * other[(1, 0)],
            self.n[1][0] * other[(0, 1)] + self.n[0][1] * other[(1, 1)],
        )
    }
}

impl MulAssign<Matrix2> for Matrix2 {
    fn mul_assign(&mut self, other: Matrix2) {
        self.n[0][0] *= other[(0, 0)] + self.n[0][1] * other[(1, 0)];
        self.n[1][0] *= other[(0, 1)] + self.n[0][1] * other[(1, 1)];
        self.n[0][1] *= other[(0, 0)] + self.n[1][1] * other[(1, 0)];
        self.n[1][1] *= other[(0, 1)] + self.n[0][1] * other[(1, 1)];
    }
}

impl MulAssign<f32> for Matrix2 {
    fn mul_assign(&mut self, other: f32) {
        self.n[0][0] *= other;
        self.n[1][0] *= other;
        self.n[0][1] *= other;
        self.n[1][1] *= other;
    }
}

impl Div<f32> for Matrix2 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Matrix2::new(
            self.n[0][0] / other,
            self.n[1][0] / other,
            self.n[0][1] / other,
            self.n[1][1] / other,
        )
    }
}

impl DivAssign<f32> for Matrix2 {
    fn div_assign(&mut self, other: f32) {
        self.n[0][0] /= other;
        self.n[1][0] /= other;
        self.n[0][1] /= other;
        self.n[1][1] /= other;
    }
}
