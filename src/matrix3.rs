use crate::prelude::*;
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Default, Copy, Clone, Debug)]
/// A 3x3 matrix.
pub struct Matrix3 {
    /// The column entries of the matrix.
    n: [Vector3; 3],
}

impl Matrix3 {
    /// Returns a matrix initialized with the nine entries supplied, with the
    /// nij parameter specifies the entry in i-th row and j-th column.
    ///
    /// # Arguments
    ///
    /// * `nij` - The value of the entry in row i and column j.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let m = Matrix3::new(1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0);
    /// ```
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Matrix3 {
        let n: [Vector3; 3] = [
            Vector3::new(a, d, g),
            Vector3::new(b, e, h),
            Vector3::new(c, f, i),
        ];
        Self { n }
    }

    /// Returns a matrix initialized with the three vectors initialize as the three
    /// columns of the matrix.
    ///
    /// # Arguments
    ///
    /// * `a` - The value of the entry in the first column.
    /// * `b` - The value of the entry in the second column.
    /// * `c` - The value of the entry in the third column.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// use math_engine::vector3::Vector3;
    /// let m = Matrix3::new_with_vecs(Vector3::new(1.0,0.0,0.0),Vector3::new(0.0,1.0,0.0), Vector3::new(0.0,0.0,1.0));
    /// ```
    pub fn new_with_vecs(a: Vector3, b: Vector3, c: Vector3) -> Matrix3 {
        let n: [Vector3; 3] = [a, b, c];
        Self { n }
    }

    pub fn vec_at(&self, index: usize) -> Vector3 {
        self[index]
    }

    pub fn at(&self, i: usize, j: usize) -> f32 {
        self[j][i]
    }

    /// Sets all nine entries of this matrix, with the
    /// nij parameter specifies the entry in i-th row and j-th column.
    ///
    /// # Arguments
    ///
    /// * `nij` - The value of the entry residing in row i and column j.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let mut m = Matrix3::new(1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0);
    /// m.set(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
    /// ```
    pub fn set(
        &mut self,
        n00: f32,
        n01: f32,
        n02: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n20: f32,
        n21: f32,
        n22: f32,
    ) {
        self[0][0] = n00;
        self[1][0] = n01;
        self[2][0] = n02;
        self[0][1] = n10;
        self[1][1] = n11;
        self[2][1] = n12;
        self[0][2] = n20;
        self[1][2] = n21;
        self[2][2] = n22;
    }

    /// Sets all nine entries of this matrix to the three column vectors given.
    ///
    /// # Arguments
    ///
    /// * `a` - The value of the entry in the first column.
    /// * `b` - The value of the entry in the second column.
    /// * `c` - The value of the entry in the third column.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// use math_engine::vector3::Vector3;
    /// let mut m = Matrix3::new(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
    /// m.set_vecs(Vector3::new(1.0,0.0,0.0),Vector3::new(0.0,1.0,0.0), Vector3::new(0.0,0.0,1.0));
    /// ```
    pub fn set_vecs(&mut self, a: Vector3, b: Vector3, c: Vector3) {
        self[0] = a;
        self[1] = b;
        self[2] = c;
    }

    /// Sets this matrix to the 3x3 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let mut m = Matrix3::new(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
    /// m.set_identity();
    /// ```
    pub fn set_identity(&mut self) {
        self[0][0] = 1.0;
        self[1][0] = 0.0;
        self[2][0] = 0.0;
        self[0][1] = 0.0;
        self[1][1] = 1.0;
        self[2][1] = 0.0;
        self[0][2] = 0.0;
        self[1][2] = 0.0;
        self[2][2] = 1.0;
    }

    /// Returns the determinant of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let m = Matrix3::new(1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0);
    /// let det = m.determinant();
    /// ```
    pub fn determinant(&self) -> f32 {
        (self.n[0][0] * self.n[1][1] * self.n[2][2] - self.n[2][1] * self.n[1][2])
            - self.n[1][0] * (self.n[0][1] * self.n[2][2] - self.n[2][1] * self.n[2][0])
            + self.n[0][2] * (self.n[0][1] * self.n[1][2] - self.n[1][1] * self.n[0][2])
    }

    /// Returns the inverse of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let m = Matrix3::new(1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0);
    /// let det = m.inverse();
    /// ```
    pub fn inverse(&self) -> Matrix3 {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let r0 = b.cross(&c);
        let r1 = c.cross(&a);
        let r2 = a.cross(&b);
        let inv = 1.0 / r2.dot(&c);
        Self::new(
            r0.x * inv,
            r0.y * inv,
            r0.z * inv,
            r1.x * inv,
            r1.y * inv,
            r1.z * inv,
            r2.x * inv,
            r2.y * inv,
            r2.z * inv,
        )
    }

    /// Returns the transpose of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let m = Matrix3::new(1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0);
    /// let det = m.transpose();
    /// ```
    pub fn transpose(&self) -> Matrix3 {
        Self::new(
            self.n[0][0],
            self.n[0][1],
            self.n[0][2],
            self.n[1][0],
            self.n[1][1],
            self.n[1][2],
            self.n[2][0],
            self.n[2][1],
            self.n[2][2],
        )
    }

    /// Returns 3x3 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::matrix3::Matrix3;
    /// let m = Matrix3::identity();
    /// ```
    pub fn identity() -> Matrix3 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
    }

    pub fn make_rotation(a: f32, v: &Vector3) -> Matrix3 {
        let c = a.cos();
        let s = a.sin();
        let d = 1.0 - c;
        let x = v.x * d;
        let y = v.y * d;
        let z = v.z * d;
        let axay = x * v.y;
        let axaz = x * v.z;
        let ayaz = y * v.z;
        Self::new(
            c + x * v.x,
            axay - s * v.z,
            axaz + s * v.y,
            axay + s * v.z,
            c + y * v.y,
            ayaz - s * v.x,
            axaz - s * v.y,
            ayaz + s * v.x,
            c + z * v.z,
        )
    }

    pub fn make_rotation_x(a: f32) -> Matrix3 {
        let c = a.cos();
        let s = a.sin();
        Self::new(1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c)
    }

    fn make_rotation_y(a: f32) -> Matrix3 {
        let c = a.cos();
        let s = a.sin();
        Self::new(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, s)
    }

    fn make_rotation_z(a: f32) -> Matrix3 {
        let c = a.cos();
        let s = a.sin();
        Self::new(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, s)
    }

    fn make_skew(angle: f32, a: &Vector3, b: &Vector3) -> Matrix3 {
        let t = angle.tan();
        let x = a.x * t;
        let y = a.y * t;
        let z = a.z * t;
        Self::new(
            x * b.x + 1.0,
            x * b.y,
            x * b.z,
            y * b.x + 1.0,
            y * b.y,
            y * b.z,
            z * b.x + 1.0,
            z * b.y,
            z * b.z,
        )
    }

    fn make_scale_vec(s: f32, a: &Vector3) -> Matrix3 {
        let ss = s - 1.0;
        let x = a.x * ss;
        let y = a.y * ss;
        let z = a.z * ss;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;
        Self::new(
            x * a.x + 1.0,
            axay,
            axaz,
            axay,
            y * a.y + 1.0,
            ayaz,
            axaz,
            ayaz,
            z * a.z + 1.0,
        )
    }

    fn make_involution(a: &Vector3) -> Matrix3 {
        let x = a.x * 2.0;
        let y = a.y * 2.0;
        let z = a.z * 2.0;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;
        Self::new(
            x * a.x - 1.0,
            axay,
            axaz,
            axay,
            y * a.y - 1.0,
            ayaz,
            axaz,
            ayaz,
            z * a.z - 1.0,
        )
    }

    fn make_scale(sx: f32, sy: f32, sz: f32) -> Matrix3 {
        Self::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, sz)
    }

    fn make_scale_x(sx: f32) -> Matrix3 {
        Self::new(sx, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    fn make_scale_y(sy: f32) -> Matrix3 {
        Self::new(1.0, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0)
    }

    fn make_scale_z(sz: f32) -> Matrix3 {
        Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, sz)
    }

    fn make_reflection(a: &Vector3) -> Matrix3 {
        let x = a.x * -2.0;
        let y = a.y * -2.0;
        let z = a.z * -2.0;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;

        Self::new(
            x * a.x + 1.0,
            axay,
            axaz,
            axay,
            y * a.y + 1.0,
            ayaz,
            axaz,
            ayaz,
            z * a.z + 1.0,
        )
    }
}

impl Index<usize> for Matrix3 {
    type Output = Vector3;
    fn index(&self, col: usize) -> &Self::Output {
        assert!(col < 3);
        &self.n[col]
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(col < 3 && row < 3);
        &self.n[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix3 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        assert!(col < 3 && row < 3);
        &mut self.n[col][row]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, col: usize) -> &mut Vector3 {
        assert!(col < 3);
        &mut self.n[col]
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::new(
            self.n[0][0] * s,
            self.n[1][0] * s,
            self.n[2][0] * s,
            self.n[0][1] * s,
            self.n[1][1] * s,
            self.n[2][1] * s,
            self.n[0][2] * s,
            self.n[1][2] * s,
            self.n[2][2] * s,
        )
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3::new(
            self.n[0][0] * other.x + self.n[1][0] * other.y + self.n[2][0] * other.z,
            self.n[0][1] * other.x + self.n[1][1] * other.y + self.n[2][1] * other.z,
            self.n[0][2] * other.x + self.n[1][2] * other.y + self.n[2][2] * other.z,
        )
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Self;

    fn mul(self, other: Matrix3) -> Self::Output {
        Self::new(
            self.n[0][0] * other[(0, 0)]
                + self.n[1][0] * other[(1, 0)]
                + self.n[2][0] * other[(2, 0)],
            self.n[0][0] * other[(0, 1)]
                + self.n[1][0] * other[(1, 1)]
                + self.n[2][0] * other[(2, 1)],
            self.n[0][0] * other[(0, 2)]
                + self.n[1][0] * other[(1, 2)]
                + self.n[2][0] * other[(2, 2)],
            self.n[0][1] * other[(0, 0)]
                + self.n[1][1] * other[(1, 0)]
                + self.n[2][1] * other[(2, 0)],
            self.n[0][1] * other[(0, 1)]
                + self.n[1][1] * other[(1, 1)]
                + self.n[2][1] * other[(2, 1)],
            self.n[0][1] * other[(0, 2)]
                + self.n[1][1] * other[(1, 2)]
                + self.n[2][1] * other[(2, 2)],
            self.n[0][2] * other[(0, 0)]
                + self.n[1][2] * other[(1, 0)]
                + self.n[2][2] * other[(2, 0)],
            self.n[1][2] * other[(0, 1)]
                + self.n[1][2] * other[(1, 1)]
                + self.n[2][2] * other[(2, 1)],
            self.n[1][2] * other[(0, 2)]
                + self.n[1][2] * other[(1, 2)]
                + self.n[2][2] * other[(2, 2)],
        )
    }
}

impl MulAssign<Matrix3> for Matrix3 {
    fn mul_assign(&mut self, other: Matrix3) {
        self.n[0][0] = self.n[0][0] * other[(0, 0)]
            + self.n[1][0] * other[(1, 0)]
            + self.n[2][0] * other[(2, 0)];
        self.n[1][0] = self.n[0][0] * other[(0, 1)]
            + self.n[1][0] * other[(1, 1)]
            + self.n[2][0] * other[(2, 1)];
        self.n[2][0] = self.n[0][0] * other[(0, 2)]
            + self.n[1][0] * other[(1, 2)]
            + self.n[2][0] * other[(2, 2)];
        self.n[1][0] = self.n[0][1] * other[(0, 0)]
            + self.n[1][1] * other[(1, 0)]
            + self.n[2][1] * other[(2, 0)];
        self.n[1][1] = self.n[0][1] * other[(0, 1)]
            + self.n[1][1] * other[(1, 1)]
            + self.n[2][1] * other[(2, 1)];
        self.n[1][2] = self.n[0][1] * other[(0, 2)]
            + self.n[1][1] * other[(1, 2)]
            + self.n[2][1] * other[(2, 2)];
        self.n[2][0] = self.n[0][2] * other[(0, 0)]
            + self.n[1][2] * other[(1, 0)]
            + self.n[2][2] * other[(2, 0)];
        self.n[2][1] = self.n[1][2] * other[(0, 1)]
            + self.n[1][2] * other[(1, 1)]
            + self.n[2][2] * other[(2, 1)];
        self.n[2][2] = self.n[1][2] * other[(0, 2)]
            + self.n[1][2] * other[(1, 2)]
            + self.n[2][2] * other[(2, 2)];
    }
}

impl MulAssign<f32> for Matrix3 {
    fn mul_assign(&mut self, other: f32) {
        self.n[0][0] *= other;
        self.n[1][0] *= other;
        self.n[2][0] *= other;
        self.n[0][1] *= other;
        self.n[1][1] *= other;
        self.n[2][1] *= other;
        self.n[0][2] *= other;
        self.n[1][2] *= other;
        self.n[2][2] *= other;
    }
}

impl Div<f32> for Matrix3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        let s = 1.0 / other;
        Self::new(
            self.n[0][0] * s,
            self.n[1][0] * s,
            self.n[2][0] * s,
            self.n[0][1] * s,
            self.n[1][1] * s,
            self.n[2][1] * s,
            self.n[0][2] * s,
            self.n[1][2] * s,
            self.n[2][2] * s,
        )
    }
}

impl DivAssign<f32> for Matrix3 {
    fn div_assign(&mut self, other: f32) {
        let other = 1.0 / other;
        self.n[0][0] *= other;
        self.n[1][0] *= other;
        self.n[2][0] *= other;
        self.n[0][1] *= other;
        self.n[1][1] *= other;
        self.n[2][1] *= other;
        self.n[0][2] *= other;
        self.n[1][2] *= other;
        self.n[2][2] *= other;
    }
}
