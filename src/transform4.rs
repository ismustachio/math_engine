use crate::prelude::*;
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Copy, Clone, Debug)]
pub struct Transform4 {
    elements: [[f32; 4]; 4],
}

impl Transform4 {
    pub fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
        i: f32,
        j: f32,
        k: f32,
        l: f32,
    ) -> Transform4 {
        let elements: [[f32; 4]; 4] = [
            [a, e, i, 0.0],
            [b, f, j, 0.0],
            [c, g, k, 0.0],
            [d, h, l, 1.0],
        ];
        Self { elements }
    }

    pub fn new_with_vecs(a: &Vector3, b: &Vector3, c: &Vector3, p: &Point3) -> Transform4 {
        let elements: [[f32; 4]; 4] = [
            [a.x, a.y, a.z, 0.0],
            [b.x, b.y, b.z, 0.0],
            [c.x, c.y, c.z, 0.0],
            [p.x, p.y, p.z, 0.0],
        ];
        Self { elements }
    }

    pub fn vec_at(&self, index: usize) -> Vector3 {
        if index > 4 {
            panic!("index out of bounds");
        }
        Vector3::new(
            self.elements[index][0],
            self.elements[index][1],
            self.elements[index][2],
        )
    }

    pub fn get_translation(&self) -> Point3 {
        Point3::new(
            self.elements[0][3],
            self.elements[1][3],
            self.elements[2][3],
        )
    }

    pub fn set_translation(&mut self, p: &Point3) {
        self.elements[3][0] = p.x;
        self.elements[3][1] = p.y;
        self.elements[3][2] = p.z;
    }

    pub fn determinant(&self) -> f32 {
        self.elements[0][0]
            * (self.elements[1][1] * self.elements[2][2]
                - self.elements[2][1] * self.elements[1][2])
            - self.elements[1][0]
                * (self.elements[0][1] * self.elements[2][2]
                    - self.elements[2][1] * self.elements[0][2])
            + self.elements[2][0]
                * (self.elements[0][1] * self.elements[1][2]
                    - self.elements[1][1] * self.elements[0][2])
    }

    pub fn inverse(&self) -> Transform4 {
        let a = self.vec_at(0);
        let b = self.vec_at(1);
        let c = self.vec_at(2);
        let d = self.vec_at(3);

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);

        let inv_det = 1.0 / s.dot(&c);
        s *= inv_det;
        t *= inv_det;
        let v = c * inv_det;
        let r0 = b.cross(&v);
        let r1 = v.cross(&a);
        Self::new(
            r0.x,
            r0.y,
            r0.z,
            -b.dot(&t),
            r1.x,
            r1.y,
            r1.z,
            a.dot(&t),
            s.x,
            s.y,
            s.z,
            -d.dot(&s),
        )
    }

    pub fn identity() -> Transform4 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    }

    // Returns 4x4 transformation matrix that represents a reflection
    // through the plane f. The plane f is assumed to be normalized.
    pub fn make_plane_reflection(f: &Plane) -> Transform4 {
        let x = f.x * -2.0;
        let y = f.y * -2.0;
        let z = f.z * -2.0;
        let nxny = x * f.y;
        let nxnz = x * f.z;
        let nynz = y * f.z;
        Self::new(
            x * f.x + 1.0,
            nxny,
            nxnz,
            x * f.w,
            nxny,
            y * f.y + 1.0,
            nynz,
            y * f.w,
            nxnz,
            nynz,
            z * f.z + 1.0,
            z * f.w,
        )
    }

    pub fn make_vec_reflection(v: &Vector3) -> Transform4 {
        let x = v.x * -2.0;
        let y = v.y * -2.0;
        let z = v.z * -2.0;
        let axay = x * v.y;
        let axaz = x * v.z;
        let ayaz = y * v.z;
        Self::new(
            x * v.x + 1.0,
            axay,
            axaz,
            0.0,
            axay,
            y * v.y + 1.0,
            ayaz,
            0.0,
            axaz,
            ayaz,
            z * v.z + 1.0,
            0.0,
        )
    }

    pub fn make_scale_x(sx: f32) -> Transform4 {
        Self::new(sx, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    }

    pub fn make_scale_y(sy: f32) -> Transform4 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    }

    pub fn make_scale_z(sz: f32) -> Transform4 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, sz, 0.0)
    }

    pub fn make_scale_xyz(sx: f32, sy: f32, sz: f32) -> Transform4 {
        Self::new(sx, 0.0, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 0.0, sz, 0.0)
    }

    pub fn make_scale_vec(s: f32, v: &Vector3) -> Transform4 {
        let scale = s - 1.0;
        let x = v.x * scale;
        let y = v.y * scale;
        let z = v.z * scale;
        let axay = x * v.y;
        let axaz = x * v.z;
        let ayaz = y * v.z;
        Self::new(
            x * v.x + 1.0,
            axay,
            axaz,
            0.0,
            axay,
            y * v.y + 1.0,
            ayaz,
            0.0,
            axaz,
            ayaz,
            z * v.z + 1.0,
            0.0,
        )
    }

    pub fn make_translation(v: &Vector3) -> Transform4 {
        Self::new(1.0, 0.0, 0.0, v.x, 0.0, 1.0, 0.0, v.y, 0.0, 0.0, 1.0, v.z)
    }

    pub fn make_involution(v: &Vector3) -> Transform4 {
        let x = v.x * 2.0;
        let y = v.y * 2.0;
        let z = v.z * 2.0;
        let axay = x * v.y;
        let axaz = x * v.z;
        let ayaz = y * v.z;
        Self::new(
            x * v.x - 1.0,
            axay,
            axaz,
            0.0,
            axay,
            y * v.y - 1.0,
            ayaz,
            0.0,
            axaz,
            ayaz,
            z * v.z - 1.0,
            0.0,
        )
    }

    pub fn make_skew(angle: f32, a: &Vector3, b: &Vector3) -> Transform4 {
        let t = angle.tan();
        let x = a.x * t;
        let y = a.y * t;
        let z = a.z * t;
        Self::new(
            x * b.x + 1.0,
            x * b.y,
            x * b.z,
            0.0,
            y * b.x,
            y * b.y + 1.0,
            y * b.z,
            0.0,
            z * b.x,
            z * b.y,
            z * b.z + 1.0,
            0.0,
        )
    }

    pub fn make_rotation_x(angle: f32) -> Transform4 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, c, -s, 0.0, 0.0, s, c, 0.0)
    }

    pub fn make_rotation_y(angle: f32) -> Transform4 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(c, 0.0, s, 0.0, 0.0, 1.0, 0.0, 0.0, -s, 0.0, c, 0.0)
    }

    pub fn make_rotation_z(angle: f32) -> Transform4 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    }

    pub fn make_rotation(angle: f32, v: &Vector3) -> Transform4 {
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
            0.0,
            axay + s * v.z,
            c + y * v.y,
            ayaz - s * v.x,
            0.0,
            axaz - s * v.y,
            ayaz + s * v.x,
            c + z * v.z,
            0.0,
        )
    }
}

impl Mul<Vector3> for Transform4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3::new(
            self.elements[0][0] * other.x
                + self.elements[1][0] * other.y
                + self.elements[2][0] * other.z,
            self.elements[0][1] * other.x
                + self.elements[1][1] * other.y
                + self.elements[2][1] * other.z,
            self.elements[0][2] * other.x
                + self.elements[1][2] * other.y
                + self.elements[2][2] * other.z,
        )
    }
}

impl Mul<Point3> for Transform4 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Self::Output {
        Point3::new(
            self.elements[0][0] * other.x
                + self.elements[1][0] * other.y
                + self.elements[2][0] * other.z
                + self.elements[3][0],
            self.elements[0][1] * other.x
                + self.elements[1][1] * other.y
                + self.elements[2][1] * other.z
                + self.elements[3][1],
            self.elements[0][2] * other.x
                + self.elements[1][2] * other.y
                + self.elements[2][2] * other.z
                + self.elements[3][2],
        )
    }
}

impl Mul<Matrix3> for Transform4 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3::new(
            self.elements[0][0] * rhs[(0, 0)]
                + self.elements[1][0] * rhs[(1, 0)]
                + self.elements[2][0] * rhs[(2, 0)],
            self.elements[0][0] * rhs[(0, 1)]
                + self.elements[1][0] * rhs[(1, 1)]
                + self.elements[2][0] * rhs[(2, 1)],
            self.elements[0][0] * rhs[(0, 2)]
                + self.elements[1][0] * rhs[(1, 2)]
                + self.elements[2][0] * rhs[(2, 2)],
            self.elements[0][0] * rhs[(0, 0)]
                + self.elements[1][1] * rhs[(1, 0)]
                + self.elements[2][1] * rhs[(2, 0)],
            self.elements[0][0] * rhs[(0, 1)]
                + self.elements[1][1] * rhs[(1, 1)]
                + self.elements[2][1] * rhs[(2, 1)],
            self.elements[0][0] * rhs[(0, 2)]
                + self.elements[1][1] * rhs[(1, 2)]
                + self.elements[2][1] * rhs[(2, 2)],
            self.elements[0][0] * rhs[(0, 0)]
                + self.elements[1][2] * rhs[(1, 0)]
                + self.elements[2][2] * rhs[(2, 0)],
            self.elements[0][0] * rhs[(0, 1)]
                + self.elements[1][2] * rhs[(1, 1)]
                + self.elements[2][2] * rhs[(2, 1)],
            self.elements[0][0] * rhs[(0, 2)]
                + self.elements[1][2] * rhs[(1, 2)]
                + self.elements[2][2] * rhs[(2, 2)],
        )
    }
}

impl Mul<Vector2> for Transform4 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(
            self.elements[0][0] * rhs.x + self.elements[1][0] * rhs.y,
            self.elements[0][1] * rhs.x + self.elements[1][1] * rhs.y,
        )
    }
}

impl Mul<Point2> for Transform4 {
    type Output = Vector2;

    fn mul(self, rhs: Point2) -> Self::Output {
        Vector2::new(
            self.elements[0][0] * rhs.x + self.elements[1][0] * rhs.y + self.elements[3][0],
            self.elements[0][1] * rhs.x + self.elements[1][1] * rhs.y + self.elements[3][1],
        )
    }
}

impl Mul<Transform4> for Transform4 {
    type Output = Self;

    fn mul(self, other: Transform4) -> Self::Output {
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
            self.elements[0][0] * other[(0, 3)]
                + self.elements[1][0] * other[(1, 3)]
                + self.elements[2][0] * other[(2, 3)]
                + self.elements[3][0],
            self.elements[0][1] * other[(0, 0)]
                + self.elements[1][1] * other[(1, 0)]
                + self.elements[2][1] * other[(2, 0)],
            self.elements[0][1] * other[(0, 1)]
                + self.elements[1][1] * other[(1, 1)]
                + self.elements[2][1] * other[(2, 1)],
            self.elements[0][1] * other[(0, 2)]
                + self.elements[1][1] * other[(1, 2)]
                + self.elements[2][1] * other[(2, 2)],
            self.elements[0][1] * other[(0, 3)]
                + self.elements[1][0] * other[(1, 3)]
                + self.elements[2][0] * other[(2, 3)]
                + self.elements[3][1],
            self.elements[0][2] * other[(0, 0)]
                + self.elements[1][2] * other[(1, 0)]
                + self.elements[2][2] * other[(2, 0)],
            self.elements[0][2] * other[(0, 1)]
                + self.elements[1][2] * other[(1, 1)]
                + self.elements[2][2] * other[(2, 1)],
            self.elements[0][2] * other[(0, 2)]
                + self.elements[1][2] * other[(1, 2)]
                + self.elements[2][2] * other[(2, 2)],
            self.elements[0][2] * other[(0, 3)]
                + self.elements[1][0] * other[(1, 3)]
                + self.elements[2][0] * other[(2, 3)]
                + self.elements[3][2],
        )
    }
}

impl Index<(usize, usize)> for Transform4 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        if row > 4 || col > 4 {
            panic!("Index out of bounds");
        }
        &self.elements[col][row]
    }
}

impl IndexMut<(usize, usize)> for Transform4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        if row > 4 || col > 4 {
            panic!("Index out of bounds");
        }
        &mut self.elements[col][row]
    }
}

impl Div<f32> for Transform4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(
            self.elements[0][0] / rhs,
            self.elements[1][0] / rhs,
            self.elements[2][0] / rhs,
            self.elements[3][0] / rhs,
            self.elements[0][1] / rhs,
            self.elements[1][1] / rhs,
            self.elements[2][1] / rhs,
            self.elements[3][1] / rhs,
            self.elements[0][2] / rhs,
            self.elements[1][2] / rhs,
            self.elements[2][2] / rhs,
            self.elements[3][2] / rhs,
        )
    }
}

impl DivAssign<f32> for Transform4 {
    fn div_assign(&mut self, rhs: f32) {
        self.elements[0][0] /= rhs;
        self.elements[1][0] /= rhs;
        self.elements[2][0] /= rhs;
        self.elements[3][0] /= rhs;
        self.elements[0][1] /= rhs;
        self.elements[1][1] /= rhs;
        self.elements[2][1] /= rhs;
        self.elements[3][1] /= rhs;
        self.elements[0][2] /= rhs;
        self.elements[1][2] /= rhs;
        self.elements[2][2] /= rhs;
        self.elements[3][2] /= rhs;
    }
}

impl MulAssign<Transform4> for Transform4 {
    fn mul_assign(&mut self, other: Transform4) {
        self.elements[0][0] = self.elements[0][0] * other[(0, 0)]
            + self.elements[1][0] * other[(1, 0)]
            + self.elements[2][0] * other[(2, 0)];
        self.elements[1][0] = self.elements[0][0] * other[(0, 1)]
            + self.elements[1][0] * other[(1, 1)]
            + self.elements[2][0] * other[(2, 1)];
        self.elements[2][0] = self.elements[0][0] * other[(0, 2)]
            + self.elements[1][0] * other[(1, 2)]
            + self.elements[2][0] * other[(2, 2)];
        self.elements[3][0] = self.elements[0][0] * other[(0, 3)]
            + self.elements[1][0] * other[(1, 3)]
            + self.elements[2][0] * other[(2, 3)]
            + self.elements[3][0];
        self.elements[0][1] = self.elements[0][1] * other[(0, 0)]
            + self.elements[1][1] * other[(1, 0)]
            + self.elements[2][1] * other[(2, 0)];
        self.elements[1][1] = self.elements[0][1] * other[(0, 1)]
            + self.elements[1][1] * other[(1, 1)]
            + self.elements[2][1] * other[(2, 1)];
        self.elements[2][1] = self.elements[0][1] * other[(0, 2)]
            + self.elements[1][1] * other[(1, 2)]
            + self.elements[2][1] * other[(2, 2)];
        self.elements[3][1] = self.elements[0][1] * other[(0, 3)]
            + self.elements[1][0] * other[(1, 3)]
            + self.elements[2][0] * other[(2, 3)]
            + self.elements[3][1];
        self.elements[0][2] = self.elements[0][2] * other[(0, 0)]
            + self.elements[1][2] * other[(1, 0)]
            + self.elements[2][2] * other[(2, 0)];
        self.elements[1][2] = self.elements[0][2] * other[(0, 1)]
            + self.elements[1][2] * other[(1, 1)]
            + self.elements[2][2] * other[(2, 1)];
        self.elements[2][2] = self.elements[0][2] * other[(0, 2)]
            + self.elements[1][2] * other[(1, 2)]
            + self.elements[2][2] * other[(2, 2)];
        self.elements[3][2] = self.elements[0][2] * other[(0, 3)]
            + self.elements[1][0] * other[(1, 3)]
            + self.elements[2][0] * other[(2, 3)]
            + self.elements[3][2];
    }
}

impl MulAssign<f32> for Transform4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.elements[0][0] *= rhs;
        self.elements[1][0] *= rhs;
        self.elements[2][0] *= rhs;
        self.elements[3][0] *= rhs;
        self.elements[0][1] *= rhs;
        self.elements[1][1] *= rhs;
        self.elements[2][1] *= rhs;
        self.elements[3][1] *= rhs;
        self.elements[0][2] *= rhs;
        self.elements[1][2] *= rhs;
        self.elements[2][2] *= rhs;
        self.elements[3][2] *= rhs;
    }
}
