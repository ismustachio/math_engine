use crate::prelude::*;
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    elements: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, l: f32, m: f32, n: f32, o: f32, p: f32) -> Matrix4 {
        let elements: [[f32; 4]; 4] = [[a, e, i, m], [b, f, j, n], [c, g, k, o], [d,h,l,p]];
        Self { elements }
    }

    fn new_with_vecs(a: &Vector4, b: &Vector4, c: &Vector4, d: &Vector4) -> Matrix4 {
        let elements: [[f32; 4]; 4] = [[a.x, b.x, c.x, d.x], [b.y, b.y, c.y, d.y], [c.z, b.z, c.z, c.z], [a.w, b.w, c.w, d.w]];
        Self { elements }
    }

    fn vec_at(&self, index: usize) -> Vector3 {
        Vector3::new(
            self.elements[index][0],
            self.elements[index][1],
            self.elements[index][2],
        )
    }

    fn determinant(&self) -> f32 {
        (self.elements[0][0] * self.elements[1][1] * self.elements[2][2]
            - self.elements[2][1] * self.elements[1][2])
            - self.elements[1][0]
                * (self.elements[0][1] * self.elements[2][2]
                    - self.elements[2][1] * self.elements[2][0])
            + self.elements[0][2]
                * (self.elements[0][1] * self.elements[1][2]
                    - self.elements[1][1] * self.elements[0][2])
    }

    fn inverse(&self) -> Matrix4 {
        let a = self.vec_at(0);
        let b = self.vec_at(1);
        let c = self.vec_at(2);
        let d = self.vec_at(3);
        
        let x = self.elements[0][3];
        let y = self.elements[1][3];
        let z = self.elements[2][3];
        let w = self.elements[3][3];

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);
        let mut u = a * y - b * x;
        let mut v = c * w - d * z;

        let inv = 1.0 / (s.dot(&v) + t.dot(&u));
        s *= inv;
        t *= inv;
        u *= inv;
        v *= inv;
        let r0 = b.cross(&v) + t * y;
        let r1 = v.cross(&a) - t * x;
        let r2 = d.cross(&u) + s * w;
        let r3 = u.cross(&c) - s * z;
        Self::new(r0.x,r0.y,r0.z,-b.dot(&t),
        r1.x, r1.y, r1.z, a.dot(&t),
        r2.x, r2.y, r2.z, -d.dot(&s),
        r3.x, r3.y, r3.z, c.dot(&s))
    }

    fn transpose(&self) -> Matrix4 {
        Self::new(
            self.elements[0][0],
            self.elements[0][1],
            self.elements[0][2],
            self.elements[0][3],
            self.elements[1][0],
            self.elements[1][1],
            self.elements[1][2],
            self.elements[1][3],
            self.elements[2][0],
            self.elements[2][1],
            self.elements[2][2],
            self.elements[2][3],
            self.elements[3][0],
            self.elements[3][1],
            self.elements[3][2],
            self.elements[3][3],
        )
    }

    fn identity() -> Matrix4 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        if row > 4 || col > 4 {
            panic!("Index out of bounds");
        }
        &self.elements[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        if row > 4 || col > 4 {
            panic!("Index out of bounds");
        }
        &mut self.elements[col][row]
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector4::new(self.elements[0][0] * rhs.x + self.elements[1][0] * rhs.y + self.elements[2][0] * rhs.z,
                     self.elements[0][1] * rhs.x + self.elements[1][1] * rhs.y + self.elements[2][1] * rhs.z,
                     self.elements[0][2] * rhs.x + self.elements[1][2] * rhs.y + self.elements[2][2] * rhs.z,
                     self.elements[0][3] * rhs.x + self.elements[1][3] * rhs.y + self.elements[2][3] * rhs.z)
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Point3) -> Self::Output {
        Vector4::new(self.elements[0][0] * rhs.x + self.elements[1][0] * rhs.y + self.elements[2][0] * rhs.z + self.elements[3][0],
                     self.elements[0][1] * rhs.x + self.elements[1][1] * rhs.y + self.elements[2][1] * rhs.z + self.elements[3][1],
                     self.elements[0][2] * rhs.x + self.elements[1][2] * rhs.y + self.elements[2][2] * rhs.z + self.elements[3][2],
                     self.elements[0][3] * rhs.x + self.elements[1][3] * rhs.y + self.elements[2][3] * rhs.z + self.elements[3][3])
    }
}

impl Mul<Point2> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Point3) -> Self::Output {
        Vector4::new(self.elements[0][0] * rhs.x + self.elements[1][0] * rhs.y + self.elements[3][0],
                     self.elements[0][1] * rhs.x + self.elements[1][1] * rhs.y + self.elements[3][1],
                     self.elements[0][2] * rhs.x + self.elements[1][2] * rhs.y + self.elements[3][2],
                     self.elements[0][3] * rhs.x + self.elements[1][3] * rhs.y + self.elements[3][3])
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.elements[0][0] * scalar,
            self.elements[1][0] * scalar,
            self.elements[2][0] * scalar,
            self.elements[3][0] * scalar,
            self.elements[0][1] * scalar,
            self.elements[1][1] * scalar,
            self.elements[2][1] * scalar,
            self.elements[3][1] * scalar,
            self.elements[0][2] * scalar,
            self.elements[1][2] * scalar,
            self.elements[2][2] * scalar,
            self.elements[3][2] * scalar,
            self.elements[0][3] * scalar,
            self.elements[1][3] * scalar,
            self.elements[2][3] * scalar,
            self.elements[3][3] * scalar,
        )
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;
    fn mul(self, other: Matrix4) -> Self::Output {
        Self::new(
            self.elements[0][0] * other[(0, 0)] 
            + self.elements[1][0] * other[(1, 0)]
            + self.elements[2][0] * other[(2, 0)]
            + self.elements[3][0] * other[(3, 0)],
            self.elements[0][0] * other[(0, 1)]
            + self.elements[1][0] * other[(1, 1)]
            + self.elements[2][0] * other[(2, 1)]
            + self.elements[3][0] * other[(3, 1)],
            self.elements[0][0] * other[(0, 2)]
            + self.elements[1][0] * other[(1, 2)]
            + self.elements[2][0] * other[(2, 2)]
            + self.elements[3][0] * other[(3, 2)],
            self.elements[0][0] * other[(0, 3)]
            + self.elements[1][0] * other[(1, 3)]
            + self.elements[2][0] * other[(2, 3)]
            + self.elements[3][0] * other[(3, 3)],
            self.elements[0][1] * other[(0, 0)]
            + self.elements[1][1] * other[(1, 0)]
            + self.elements[2][1] * other[(2, 0)]
            + self.elements[3][1] * other[(3, 0)],
            self.elements[0][1] * other[(0, 1)]
            + self.elements[1][1] * other[(1, 1)]
            + self.elements[2][1] * other[(2, 1)]
            + self.elements[3][1] * other[(3, 1)],
            self.elements[0][1] * other[(0, 2)]
            + self.elements[1][1] * other[(1, 2)]
            + self.elements[2][1] * other[(2, 2)]
            + self.elements[3][1] * other[(3, 2)],
            self.elements[0][1] * other[(0, 3)]
            + self.elements[1][1] * other[(1, 3)]
            + self.elements[2][1] * other[(2, 3)]
            + self.elements[3][1] * other[(3, 3)],
            self.elements[0][2] * other[(0, 0)]
            + self.elements[1][2] * other[(1, 0)]
            + self.elements[2][2] * other[(2, 0)]
            + self.elements[3][2] * other[(3, 0)],
            self.elements[0][2] * other[(0, 1)]
            + self.elements[1][2] * other[(1, 1)]
            + self.elements[2][2] * other[(2, 1)]
            + self.elements[3][2] * other[(3, 1)],
            self.elements[0][2] * other[(0, 2)]
            + self.elements[1][2] * other[(1, 2)]
            + self.elements[2][2] * other[(2, 2)]
            + self.elements[3][2] * other[(3, 2)],
            self.elements[0][2] * other[(0, 3)]
            + self.elements[1][2] * other[(1, 3)]
            + self.elements[2][2] * other[(2, 3)]
            + self.elements[3][2] * other[(3, 3)],
            self.elements[0][3] * other[(0, 0)]
            + self.elements[1][3] * other[(1, 0)]
            + self.elements[2][3] * other[(2, 0)]
            + self.elements[3][3] * other[(3, 0)],
            self.elements[0][3] * other[(0, 1)]
            + self.elements[1][3] * other[(1, 1)]
            + self.elements[2][3] * other[(2, 1)]
            + self.elements[3][3] * other[(3, 1)],
            self.elements[0][3] * other[(0, 2)]
            + self.elements[1][3] * other[(1, 2)]
            + self.elements[2][3] * other[(2, 2)]
            + self.elements[3][3] * other[(3, 2)],
            self.elements[0][3] * other[(0, 3)]
            + self.elements[1][3] * other[(1, 3)]
            + self.elements[2][3] * other[(2, 3)]
            + self.elements[3][3] * other[(3, 3)]
        )
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, other: Matrix4) {
        self.elements[0][0] = self.elements[0][0] * other[(0, 0)] 
            + self.elements[1][0] * other[(1, 0)]
            + self.elements[2][0] * other[(2, 0)]
            + self.elements[3][0] * other[(3, 0)];
        self.elements[1][0] = self.elements[0][0] * other[(0, 1)]
            + self.elements[1][0] * other[(1, 1)]
            + self.elements[2][0] * other[(2, 1)]
            + self.elements[3][0] * other[(3, 1)];
        self.elements[2][0] = self.elements[0][0] * other[(0, 2)]
            + self.elements[1][0] * other[(1, 2)]
            + self.elements[2][0] * other[(2, 2)]
            + self.elements[3][0] * other[(3, 2)];
        self.elements[3][0] = self.elements[0][0] * other[(0, 3)]
            + self.elements[1][0] * other[(1, 3)]
            + self.elements[2][0] * other[(2, 3)]
            + self.elements[3][0] * other[(3, 3)];
        self.elements[0][1] = self.elements[0][1] * other[(0, 0)]
            + self.elements[1][1] * other[(1, 0)]
            + self.elements[2][1] * other[(2, 0)]
            + self.elements[3][1] * other[(3, 0)];
        self.elements[1][1] = self.elements[0][1] * other[(0, 1)]
            + self.elements[1][1] * other[(1, 1)]
            + self.elements[2][1] * other[(2, 1)]
            + self.elements[3][1] * other[(3, 1)];
        self.elements[2][1] = self.elements[0][1] * other[(0, 2)]
            + self.elements[1][1] * other[(1, 2)]
            + self.elements[2][1] * other[(2, 2)]
            + self.elements[3][1] * other[(3, 2)];
        self.elements[3][1] = self.elements[0][1] * other[(0, 3)]
            + self.elements[1][1] * other[(1, 3)]
            + self.elements[2][1] * other[(2, 3)]
            + self.elements[3][1] * other[(3, 3)];
        self.elements[0][2] = self.elements[0][2] * other[(0, 0)]
            + self.elements[1][2] * other[(1, 0)]
            + self.elements[2][2] * other[(2, 0)]
            + self.elements[3][2] * other[(3, 0)];
        self.elements[1][2] = self.elements[0][2] * other[(0, 1)]
            + self.elements[1][2] * other[(1, 1)]
            + self.elements[2][2] * other[(2, 1)]
            + self.elements[3][2] * other[(3, 1)];
        self.elements[2][2] = self.elements[0][2] * other[(0, 2)]
            + self.elements[1][2] * other[(1, 2)]
            + self.elements[2][2] * other[(2, 2)]
            + self.elements[3][2] * other[(3, 2)];
        self.elements[3][2] = self.elements[0][2] * other[(0, 3)]
            + self.elements[1][2] * other[(1, 3)]
            + self.elements[2][2] * other[(2, 3)]
            + self.elements[3][2] * other[(3, 3)];
        self.elements[0][3] = self.elements[0][3] * other[(0, 0)]
            + self.elements[1][3] * other[(1, 0)]
            + self.elements[2][3] * other[(2, 0)]
            + self.elements[3][3] * other[(3, 0)];
        self.elements[1][3] = self.elements[0][3] * other[(0, 1)]
            + self.elements[1][3] * other[(1, 1)]
            + self.elements[2][3] * other[(2, 1)]
            + self.elements[3][3] * other[(3, 1)];
        self.elements[2][3] = self.elements[0][3] * other[(0, 2)]
            + self.elements[1][3] * other[(1, 2)]
            + self.elements[2][3] * other[(2, 2)]
            + self.elements[3][3] * other[(3, 2)];
        self.elements[3][3] = self.elements[0][3] * other[(0, 3)]
            + self.elements[1][3] * other[(1, 3)]
            + self.elements[2][3] * other[(2, 3)]
            + self.elements[3][3] * other[(3, 3)];
    }
}

impl MulAssign<f32> for Matrix4 {
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
        self.elements[0][3] *= rhs;
        self.elements[1][3] *= rhs;
        self.elements[2][3] *= rhs;
        self.elements[3][3] *= rhs;
    }
}

impl Div<f32> for Matrix4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let s = 1.0 / rhs;
        Self::new(
            self.elements[0][0] / s,
            self.elements[1][0] / s,
            self.elements[2][0] / s,
            self.elements[3][0] / s,
            self.elements[0][1] / s,
            self.elements[1][1] / s,
            self.elements[2][1] / s,
            self.elements[3][1] / s,
            self.elements[0][2] / s,
            self.elements[1][2] / s,
            self.elements[2][2] / s,
            self.elements[3][2] / s,
            self.elements[0][3] / s,
            self.elements[1][3] / s,
            self.elements[2][3] / s,
            self.elements[3][3] / s,
        )
    }
}

impl DivAssign<f32> for Matrix4 {
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
        self.elements[0][3] /= rhs;
        self.elements[1][3] /= rhs;
        self.elements[2][3] /= rhs;
        self.elements[3][3] /= rhs;
    }
}

impl Default for Matrix4 {
    fn default() -> Matrix4 {
        Matrix4::new(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0)
    }
}
