use std::ops::{Div, Index, IndexMut, Mul};

use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    pub elements: [[f32; 2]; 2],
}

impl Matrix2 {
    fn new(a: f32, b: f32, c: f32, d: f32) -> Matrix2 {
        let elements: [[f32; 2]; 2] = [[a, c], [b, d]];
        Self { elements }
    }

    fn new_with_vecs(a: &Vector2, b: &Vector2) -> Matrix2 {
        let elements: [[f32; 2]; 2] = [[a.x, b.x], [a.y, b.y]];
        Self { elements }
    }

    fn vec_at(&self, index: usize) -> Vector2 {
        if index > 2 {
            panic!("Index out of bounds");
        }
        Vector2::new(self.elements[index][0], self.elements[index][1])
    }

    fn determinant(&self) -> f32 {
        self.elements[0][0] * self.elements[1][1] - self.elements[1][0] * self.elements[0][1]
    }

    fn inverse(&self) -> Matrix2 {
        let inv = 1.0 / self.determinant();
        Self::new(
            self.elements[1][1] * inv,
            -self.elements[0][1] * inv,
            -self.elements[1][0] * inv,
            self.elements[0][0] * inv,
        )
    }

    fn transpose(&self) -> Matrix2 {
        Self::new(
            self.elements[0][0],
            self.elements[0][1],
            self.elements[1][0],
            self.elements[1][1],
        )
    }

    fn identity() -> Matrix2 {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    fn make_rotation(angle: f32) -> Matrix2 {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(c, -s, s, -c)
    }

    fn make_scale_x(sx: f32) -> Matrix2 {
        Self::new(sx, 0.0, 0.0, 1.0)
    }

    fn make_scale_y(sy: f32) -> Matrix2 {
        Self::new(1.0, 0.0, 0.0, sy)
    }

    fn make_scale(scale: f32) -> Matrix2 {
        Self::new(scale, 0.0, 0.0, scale)
    }

    fn make_scale_xy(sx: f32, sy: f32) -> Matrix2 {
        Self::new(sx, 0.0, 0.0, sy)
    }
}

impl Index<(usize, usize)> for Matrix2 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        if row > 2 || col > 2 {
            panic!("Index out of bounds")
        }
        &self.elements[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix2 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        if row > 2 || col > 2 {
            panic!("Index out of bounds")
        }
        &mut self.elements[col][row]
    }
}

impl Mul<f32> for Matrix2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.elements[0][0] * scalar,
            self.elements[1][0] * scalar,
            self.elements[0][1] * scalar,
            self.elements[1][1] * scalar,
        )
    }
}

impl Mul<Matrix2> for Matrix2 {
    type Output = Self;

    fn mul(self, other: Matrix2) -> Self::Output {
        Self::new(
            self.elements[0][0] * other[(0, 0)] + self.elements[0][1] * other[(1, 0)],
            self.elements[0][0] * other[(0, 1)] + self.elements[0][1] * other[(1, 1)],
            self.elements[1][0] * other[(0, 0)] + self.elements[1][1] * other[(1, 0)],
            self.elements[1][0] * other[(0, 1)] + self.elements[0][1] * other[(1, 1)],
        )
    }
}

impl Div<f32> for Matrix2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Matrix2::new(
            self.elements[0][0] / rhs,
            self.elements[1][0] / rhs,
            self.elements[0][1] / rhs,
            self.elements[1][1] / rhs,
        )
    }
}
