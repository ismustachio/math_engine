use crate::prelude::*;
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Default, Copy, Clone, Debug)]
pub struct Matrix4 {
    n: [Vector4; 4],
}

impl Matrix4 {
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
        m: f32,
        n: f32,
        o: f32,
        p: f32,
    ) -> Matrix4 {
        let n: [Vector4; 4] = [
            Vector4::new(a, e, i, m),
            Vector4::new(b, f, j, n),
            Vector4::new(c, g, k, o),
            Vector4::new(d, h, l, p),
        ];
        Self { n }
    }

    fn new_with_vecs(a: Vector4, b: Vector4, c: Vector4, d: Vector4) -> Matrix4 {
        let n: [Vector4; 4] = [a, b, c, d];
        Self { n }
    }

    fn at(&self, i: usize, j: usize) -> f32 {
        self[j][i]
    }

    fn vec3_at(&self, i: usize) -> Vector3 {
        Vector3::new(self[i].x, self[i].y, self[i].z)
    }

    fn vec_at(&self, i: usize) -> Vector4 {
        self[i]
    }

    fn determinant(&self) -> f32 {
        (self.n[0][0] * self.n[1][1] * self.n[2][2] - self.n[2][1] * self.n[1][2])
            - self.n[1][0] * (self.n[0][1] * self.n[2][2] - self.n[2][1] * self.n[2][0])
            + self.n[0][2] * (self.n[0][1] * self.n[1][2] - self.n[1][1] * self.n[0][2])
    }

    fn inverse(&self) -> Matrix4 {
        let a = self.vec3_at(0);
        let b = self.vec3_at(1);
        let c = self.vec3_at(2);
        let d = self.vec3_at(3);

        let x = self.n[0][3];
        let y = self.n[1][3];
        let z = self.n[2][3];
        let w = self.n[3][3];

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
        Self::new(
            r0.x,
            r0.y,
            r0.z,
            -b.dot(&t),
            r1.x,
            r1.y,
            r1.z,
            a.dot(&t),
            r2.x,
            r2.y,
            r2.z,
            -d.dot(&s),
            r3.x,
            r3.y,
            r3.z,
            c.dot(&s),
        )
    }

    fn transpose(&self) -> Matrix4 {
        Self::new(
            self.n[0][0],
            self.n[0][1],
            self.n[0][2],
            self.n[0][3],
            self.n[1][0],
            self.n[1][1],
            self.n[1][2],
            self.n[1][3],
            self.n[2][0],
            self.n[2][1],
            self.n[2][2],
            self.n[2][3],
            self.n[3][0],
            self.n[3][1],
            self.n[3][2],
            self.n[3][3],
        )
    }

    fn identity() -> Matrix4 {
        Self::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(row < 4 && col < 4);
        &self.n[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        assert!(row < 4 && col < 4);
        &mut self.n[col][row]
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4;
    fn index(&self, col: usize) -> &Self::Output {
        assert!(col < 4);
        &self.n[col]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, col: usize) -> &mut Vector4 {
        assert!(col < 4);
        &mut self.n[col]
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector4::new(
            self.n[0][0] * rhs.x + self.n[1][0] * rhs.y + self.n[2][0] * rhs.z,
            self.n[0][1] * rhs.x + self.n[1][1] * rhs.y + self.n[2][1] * rhs.z,
            self.n[0][2] * rhs.x + self.n[1][2] * rhs.y + self.n[2][2] * rhs.z,
            self.n[0][3] * rhs.x + self.n[1][3] * rhs.y + self.n[2][3] * rhs.z,
        )
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Point3) -> Self::Output {
        Vector4::new(
            self.n[0][0] * rhs.x + self.n[1][0] * rhs.y + self.n[2][0] * rhs.z + self.n[3][0],
            self.n[0][1] * rhs.x + self.n[1][1] * rhs.y + self.n[2][1] * rhs.z + self.n[3][1],
            self.n[0][2] * rhs.x + self.n[1][2] * rhs.y + self.n[2][2] * rhs.z + self.n[3][2],
            self.n[0][3] * rhs.x + self.n[1][3] * rhs.y + self.n[2][3] * rhs.z + self.n[3][3],
        )
    }
}

impl Mul<Point2> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Point2) -> Self::Output {
        Vector4::new(
            self.n[0][0] * rhs.x + self.n[1][0] * rhs.y + self.n[3][0],
            self.n[0][1] * rhs.x + self.n[1][1] * rhs.y + self.n[3][1],
            self.n[0][2] * rhs.x + self.n[1][2] * rhs.y + self.n[3][2],
            self.n[0][3] * rhs.x + self.n[1][3] * rhs.y + self.n[3][3],
        )
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::new(
            self.n[0][0] * s,
            self.n[1][0] * s,
            self.n[2][0] * s,
            self.n[3][0] * s,
            self.n[0][1] * s,
            self.n[1][1] * s,
            self.n[2][1] * s,
            self.n[3][1] * s,
            self.n[0][2] * s,
            self.n[1][2] * s,
            self.n[2][2] * s,
            self.n[3][2] * s,
            self.n[0][3] * s,
            self.n[1][3] * s,
            self.n[2][3] * s,
            self.n[3][3] * s,
        )
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        Self::new(
            self.n[0][0] * rhs[(0, 0)]
                + self.n[1][0] * rhs[(1, 0)]
                + self.n[2][0] * rhs[(2, 0)]
                + self.n[3][0] * rhs[(3, 0)],
            self.n[0][0] * rhs[(0, 1)]
                + self.n[1][0] * rhs[(1, 1)]
                + self.n[2][0] * rhs[(2, 1)]
                + self.n[3][0] * rhs[(3, 1)],
            self.n[0][0] * rhs[(0, 2)]
                + self.n[1][0] * rhs[(1, 2)]
                + self.n[2][0] * rhs[(2, 2)]
                + self.n[3][0] * rhs[(3, 2)],
            self.n[0][0] * rhs[(0, 3)]
                + self.n[1][0] * rhs[(1, 3)]
                + self.n[2][0] * rhs[(2, 3)]
                + self.n[3][0] * rhs[(3, 3)],
            self.n[0][1] * rhs[(0, 0)]
                + self.n[1][1] * rhs[(1, 0)]
                + self.n[2][1] * rhs[(2, 0)]
                + self.n[3][1] * rhs[(3, 0)],
            self.n[0][1] * rhs[(0, 1)]
                + self.n[1][1] * rhs[(1, 1)]
                + self.n[2][1] * rhs[(2, 1)]
                + self.n[3][1] * rhs[(3, 1)],
            self.n[0][1] * rhs[(0, 2)]
                + self.n[1][1] * rhs[(1, 2)]
                + self.n[2][1] * rhs[(2, 2)]
                + self.n[3][1] * rhs[(3, 2)],
            self.n[0][1] * rhs[(0, 3)]
                + self.n[1][1] * rhs[(1, 3)]
                + self.n[2][1] * rhs[(2, 3)]
                + self.n[3][1] * rhs[(3, 3)],
            self.n[0][2] * rhs[(0, 0)]
                + self.n[1][2] * rhs[(1, 0)]
                + self.n[2][2] * rhs[(2, 0)]
                + self.n[3][2] * rhs[(3, 0)],
            self.n[0][2] * rhs[(0, 1)]
                + self.n[1][2] * rhs[(1, 1)]
                + self.n[2][2] * rhs[(2, 1)]
                + self.n[3][2] * rhs[(3, 1)],
            self.n[0][2] * rhs[(0, 2)]
                + self.n[1][2] * rhs[(1, 2)]
                + self.n[2][2] * rhs[(2, 2)]
                + self.n[3][2] * rhs[(3, 2)],
            self.n[0][2] * rhs[(0, 3)]
                + self.n[1][2] * rhs[(1, 3)]
                + self.n[2][2] * rhs[(2, 3)]
                + self.n[3][2] * rhs[(3, 3)],
            self.n[0][3] * rhs[(0, 0)]
                + self.n[1][3] * rhs[(1, 0)]
                + self.n[2][3] * rhs[(2, 0)]
                + self.n[3][3] * rhs[(3, 0)],
            self.n[0][3] * rhs[(0, 1)]
                + self.n[1][3] * rhs[(1, 1)]
                + self.n[2][3] * rhs[(2, 1)]
                + self.n[3][3] * rhs[(3, 1)],
            self.n[0][3] * rhs[(0, 2)]
                + self.n[1][3] * rhs[(1, 2)]
                + self.n[2][3] * rhs[(2, 2)]
                + self.n[3][3] * rhs[(3, 2)],
            self.n[0][3] * rhs[(0, 3)]
                + self.n[1][3] * rhs[(1, 3)]
                + self.n[2][3] * rhs[(2, 3)]
                + self.n[3][3] * rhs[(3, 3)],
        )
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        self.n[0][0] = self.n[0][0] * rhs[(0, 0)]
            + self.n[1][0] * rhs[(1, 0)]
            + self.n[2][0] * rhs[(2, 0)]
            + self.n[3][0] * rhs[(3, 0)];
        self.n[1][0] = self.n[0][0] * rhs[(0, 1)]
            + self.n[1][0] * rhs[(1, 1)]
            + self.n[2][0] * rhs[(2, 1)]
            + self.n[3][0] * rhs[(3, 1)];
        self.n[2][0] = self.n[0][0] * rhs[(0, 2)]
            + self.n[1][0] * rhs[(1, 2)]
            + self.n[2][0] * rhs[(2, 2)]
            + self.n[3][0] * rhs[(3, 2)];
        self.n[3][0] = self.n[0][0] * rhs[(0, 3)]
            + self.n[1][0] * rhs[(1, 3)]
            + self.n[2][0] * rhs[(2, 3)]
            + self.n[3][0] * rhs[(3, 3)];
        self.n[0][1] = self.n[0][1] * rhs[(0, 0)]
            + self.n[1][1] * rhs[(1, 0)]
            + self.n[2][1] * rhs[(2, 0)]
            + self.n[3][1] * rhs[(3, 0)];
        self.n[1][1] = self.n[0][1] * rhs[(0, 1)]
            + self.n[1][1] * rhs[(1, 1)]
            + self.n[2][1] * rhs[(2, 1)]
            + self.n[3][1] * rhs[(3, 1)];
        self.n[2][1] = self.n[0][1] * rhs[(0, 2)]
            + self.n[1][1] * rhs[(1, 2)]
            + self.n[2][1] * rhs[(2, 2)]
            + self.n[3][1] * rhs[(3, 2)];
        self.n[3][1] = self.n[0][1] * rhs[(0, 3)]
            + self.n[1][1] * rhs[(1, 3)]
            + self.n[2][1] * rhs[(2, 3)]
            + self.n[3][1] * rhs[(3, 3)];
        self.n[0][2] = self.n[0][2] * rhs[(0, 0)]
            + self.n[1][2] * rhs[(1, 0)]
            + self.n[2][2] * rhs[(2, 0)]
            + self.n[3][2] * rhs[(3, 0)];
        self.n[1][2] = self.n[0][2] * rhs[(0, 1)]
            + self.n[1][2] * rhs[(1, 1)]
            + self.n[2][2] * rhs[(2, 1)]
            + self.n[3][2] * rhs[(3, 1)];
        self.n[2][2] = self.n[0][2] * rhs[(0, 2)]
            + self.n[1][2] * rhs[(1, 2)]
            + self.n[2][2] * rhs[(2, 2)]
            + self.n[3][2] * rhs[(3, 2)];
        self.n[3][2] = self.n[0][2] * rhs[(0, 3)]
            + self.n[1][2] * rhs[(1, 3)]
            + self.n[2][2] * rhs[(2, 3)]
            + self.n[3][2] * rhs[(3, 3)];
        self.n[0][3] = self.n[0][3] * rhs[(0, 0)]
            + self.n[1][3] * rhs[(1, 0)]
            + self.n[2][3] * rhs[(2, 0)]
            + self.n[3][3] * rhs[(3, 0)];
        self.n[1][3] = self.n[0][3] * rhs[(0, 1)]
            + self.n[1][3] * rhs[(1, 1)]
            + self.n[2][3] * rhs[(2, 1)]
            + self.n[3][3] * rhs[(3, 1)];
        self.n[2][3] = self.n[0][3] * rhs[(0, 2)]
            + self.n[1][3] * rhs[(1, 2)]
            + self.n[2][3] * rhs[(2, 2)]
            + self.n[3][3] * rhs[(3, 2)];
        self.n[3][3] = self.n[0][3] * rhs[(0, 3)]
            + self.n[1][3] * rhs[(1, 3)]
            + self.n[2][3] * rhs[(2, 3)]
            + self.n[3][3] * rhs[(3, 3)];
    }
}

impl MulAssign<f32> for Matrix4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.n[0][0] *= rhs;
        self.n[1][0] *= rhs;
        self.n[2][0] *= rhs;
        self.n[3][0] *= rhs;
        self.n[0][1] *= rhs;
        self.n[1][1] *= rhs;
        self.n[2][1] *= rhs;
        self.n[3][1] *= rhs;
        self.n[0][2] *= rhs;
        self.n[1][2] *= rhs;
        self.n[2][2] *= rhs;
        self.n[3][2] *= rhs;
        self.n[0][3] *= rhs;
        self.n[1][3] *= rhs;
        self.n[2][3] *= rhs;
        self.n[3][3] *= rhs;
    }
}

impl Div<f32> for Matrix4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let s = 1.0 / rhs;
        Self::new(
            self.n[0][0] / s,
            self.n[1][0] / s,
            self.n[2][0] / s,
            self.n[3][0] / s,
            self.n[0][1] / s,
            self.n[1][1] / s,
            self.n[2][1] / s,
            self.n[3][1] / s,
            self.n[0][2] / s,
            self.n[1][2] / s,
            self.n[2][2] / s,
            self.n[3][2] / s,
            self.n[0][3] / s,
            self.n[1][3] / s,
            self.n[2][3] / s,
            self.n[3][3] / s,
        )
    }
}

impl DivAssign<f32> for Matrix4 {
    fn div_assign(&mut self, rhs: f32) {
        self.n[0][0] /= rhs;
        self.n[1][0] /= rhs;
        self.n[2][0] /= rhs;
        self.n[3][0] /= rhs;
        self.n[0][1] /= rhs;
        self.n[1][1] /= rhs;
        self.n[2][1] /= rhs;
        self.n[3][1] /= rhs;
        self.n[0][2] /= rhs;
        self.n[1][2] /= rhs;
        self.n[2][2] /= rhs;
        self.n[3][2] /= rhs;
        self.n[0][3] /= rhs;
        self.n[1][3] /= rhs;
        self.n[2][3] /= rhs;
        self.n[3][3] /= rhs;
    }
}
