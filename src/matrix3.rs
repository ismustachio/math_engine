use crate::prelude::*;
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    elements: [[f32; 3]; 3],
}

impl Matrix3 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Matrix3 {
        let elements: [[f32; 3]; 3] = [[a, d, g], [b, e, h], [c, f, i]];
        Self { elements }
    }

    pub fn new_with_vecs(a: &Vector3, b: &Vector3, c: &Vector3) -> Matrix3 {
        let elements: [[f32; 3]; 3] = [[a.x, b.x, c.x], [b.y, b.y, c.y], [c.z, b.z, c.z]];
        Self { elements }
    }

    pub fn vec_at(&self, index: usize) -> Vector3 {
        Vector3::new(
            self.elements[index][0],
            self.elements[index][1],
            self.elements[index][2],
        )
    }

    pub fn determinant(&self) -> f32 {
        (self.elements[0][0] * self.elements[1][1] * self.elements[2][2]
            - self.elements[2][1] * self.elements[1][2])
            - self.elements[1][0]
                * (self.elements[0][1] * self.elements[2][2]
                    - self.elements[2][1] * self.elements[2][0])
            + self.elements[0][2]
                * (self.elements[0][1] * self.elements[1][2]
                    - self.elements[1][1] * self.elements[0][2])
    }

    pub fn inverse(&self) -> Matrix3 {
        let a = self.vec_at(0);
        let b = self.vec_at(1);
        let c = self.vec_at(2);
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

    fn transpose(&self) -> Matrix3 {
        Self::new(
            self.elements[0][0],
            self.elements[0][1],
            self.elements[0][2],
            self.elements[1][0],
            self.elements[1][1],
            self.elements[1][2],
            self.elements[2][0],
            self.elements[2][1],
            self.elements[2][2],
        )
    }

    fn identity() -> Matrix3 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
    }

    fn make_rotation(angle: f32, v: &Vector3) -> Matrix3 {
        let c = angle.cos();
        let s = angle.sin();
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

    fn make_rotation_x(angle: f32) -> Matrix3 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c)
    }

    fn make_rotation_y(angle: f32) -> Matrix3 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, s)
    }

    fn make_rotation_z(angle: f32) -> Matrix3 {
        let c = angle.cos();
        let s = angle.sin();
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

impl Index<(usize, usize)> for Matrix3 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        if row > 3 || col > 3 {
            panic!("Index out of bounds");
        }
        &self.elements[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix3 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        if row > 3 || col > 3 {
            panic!("Index out of bounds");
        }
        &mut self.elements[col][row]
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.elements[0][0] * scalar,
            self.elements[1][0] * scalar,
            self.elements[2][0] * scalar,
            self.elements[0][1] * scalar,
            self.elements[1][1] * scalar,
            self.elements[2][1] * scalar,
            self.elements[0][2] * scalar,
            self.elements[1][2] * scalar,
            self.elements[2][2] * scalar,
        )
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Self;

    fn mul(self, other: Matrix3) -> Self::Output {
        Self::new(
            self.elements[0][0] * other[(0, 0)]
                + self.elements[1][0] * other[(1, 0)]
                + self.elements[2][0] * other[(2, 0)],
            self.elements[0][0] * other[(0, 1)]
                + self.elements[1][0] * other[(1, 1)]
                + self.elements[2][0] * other[(2, 1)],
            self.elements[0][0] * other[(0, 2)]
                + self.elements[1][0] * other[(1, 2)]
                + self.elements[2][0] * other[(2, 2)],
            self.elements[0][1] * other[(0, 0)]
                + self.elements[1][1] * other[(1, 0)]
                + self.elements[2][1] * other[(2, 0)],
            self.elements[0][1] * other[(0, 1)]
                + self.elements[1][1] * other[(1, 1)]
                + self.elements[2][1] * other[(2, 1)],
            self.elements[0][1] * other[(0, 2)]
                + self.elements[1][1] * other[(1, 2)]
                + self.elements[2][1] * other[(2, 2)],
            self.elements[0][2] * other[(0, 0)]
                + self.elements[1][2] * other[(1, 0)]
                + self.elements[2][2] * other[(2, 0)],
            self.elements[1][2] * other[(0, 1)]
                + self.elements[1][2] * other[(1, 1)]
                + self.elements[2][2] * other[(2, 1)],
            self.elements[1][2] * other[(0, 2)]
                + self.elements[1][2] * other[(1, 2)]
                + self.elements[2][2] * other[(2, 2)],
        )
    }
}

impl MulAssign<Matrix3> for Matrix3 {
    fn mul_assign(&mut self, other: Matrix3) {
        self.elements[0][0] = self.elements[0][0] * other[(0, 0)]
            + self.elements[1][0] * other[(1, 0)]
            + self.elements[2][0] * other[(2, 0)];
        self.elements[1][0] = self.elements[0][0] * other[(0, 1)]
            + self.elements[1][0] * other[(1, 1)]
            + self.elements[2][0] * other[(2, 1)];
        self.elements[2][0] = self.elements[0][0] * other[(0, 2)]
            + self.elements[1][0] * other[(1, 2)]
            + self.elements[2][0] * other[(2, 2)];
        self.elements[1][0] = self.elements[0][1] * other[(0, 0)]
            + self.elements[1][1] * other[(1, 0)]
            + self.elements[2][1] * other[(2, 0)];
        self.elements[1][1] = self.elements[0][1] * other[(0, 1)]
            + self.elements[1][1] * other[(1, 1)]
            + self.elements[2][1] * other[(2, 1)];
        self.elements[1][2] = self.elements[0][1] * other[(0, 2)]
            + self.elements[1][1] * other[(1, 2)]
            + self.elements[2][1] * other[(2, 2)];
        self.elements[2][0] = self.elements[0][2] * other[(0, 0)]
            + self.elements[1][2] * other[(1, 0)]
            + self.elements[2][2] * other[(2, 0)];
        self.elements[2][1] = self.elements[1][2] * other[(0, 1)]
            + self.elements[1][2] * other[(1, 1)]
            + self.elements[2][2] * other[(2, 1)];
        self.elements[2][2] = self.elements[1][2] * other[(0, 2)]
            + self.elements[1][2] * other[(1, 2)]
            + self.elements[2][2] * other[(2, 2)];
    }
}

impl MulAssign<f32> for Matrix3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.elements[0][0] *= rhs;
        self.elements[1][0] *= rhs;
        self.elements[2][0] *= rhs;
        self.elements[0][1] *= rhs;
        self.elements[1][1] *= rhs;
        self.elements[2][1] *= rhs;
        self.elements[0][2] *= rhs;
        self.elements[1][2] *= rhs;
        self.elements[2][2] *= rhs;
    }
}

impl Div<f32> for Matrix3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let s = 1.0 / rhs;
        Self::new(
            self.elements[0][0] / s,
            self.elements[1][0] / s,
            self.elements[2][0] / s,
            self.elements[0][1] / s,
            self.elements[1][1] / s,
            self.elements[2][1] / s,
            self.elements[0][2] / s,
            self.elements[1][2] / s,
            self.elements[2][2] / s,
        )
    }
}

impl DivAssign<f32> for Matrix3 {
    fn div_assign(&mut self, rhs: f32) {
        self.elements[0][0] /= rhs;
        self.elements[1][0] /= rhs;
        self.elements[2][0] /= rhs;
        self.elements[0][1] /= rhs;
        self.elements[1][1] /= rhs;
        self.elements[2][1] /= rhs;
        self.elements[0][2] /= rhs;
        self.elements[1][2] /= rhs;
        self.elements[2][2] /= rhs;
    }
}

impl Default for Matrix3 {
    fn default() -> Matrix3 {
        Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}
