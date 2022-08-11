use crate::prelude::*;
use std::ops::{Div, Index, IndexMut, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Transform4 {
    elements: [[f32; 4]; 4],
}

impl Transform4 {
    fn new(
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

    fn new_with_vecs(a: &Vector3, b: &Vector3, c: &Vector3, p: &Point3) -> Transform4 {
        let elements: [[f32; 4]; 4] = [
            [a.x, a.y, a.z, 0.0],
            [b.x, b.y, b.z, 0.0],
            [c.x, c.y, c.z, 0.0],
            [p.x, p.y, p.z, 0.0],
        ];
        Self { elements }
    }

    fn vec_at(&self, index: usize) -> Vector3 {
        if index > 4 {
            panic!("index out of bounds");
        }
        Vector3::new(
            self.elements[index][0],
            self.elements[index][1],
            self.elements[index][2],
        )
    }

    fn get_translation(&self) -> Point3 {
        Point3::new(
            self.elements[0][3],
            self.elements[1][3],
            self.elements[2][3],
        )
    }

    fn set_translation(&mut self, p: &Point3) {
        self.elements[3][0] = p.x;
        self.elements[3][1] = p.y;
        self.elements[3][2] = p.z;
    }

    fn inverse(&self) -> Transform4 {
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

    fn identity() -> Transform4 {
        Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
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
