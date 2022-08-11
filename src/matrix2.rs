use std::ops::{Div, Index, IndexMut, Mul};

use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
	m00: f32,
	m01: f32,
	m10: f32,
	m11: f32,
}

impl Matrix2 {
	fn new(a: f32, b: f32, c: f32, d: f32) -> Matrix2 {
		Self {
			m00: a,
			m10: b,
			m01: c,
			m11: d,
		}
	}

	fn new_with_vecs(a: &Vector2, b: &Vector2) -> Matrix2 {
		Self {
			m00: a.x,
			m10: a.y,
			m01: b.x,
			m11: b.y,
		}
	}

	fn vec_at(&self, index: usize) -> Vector2 {
		if index > 2 || index > 2 {
    	panic!("Index out of bounds");
   	}
    Vector2::new(self[(index, 0)], self[(index, 1)])
	}

	fn determinant(&self) -> f32 {
		self.m00 * self.m11 - self.m10 * self.m01
	}

	fn inverse(&self) -> Matrix2 {
		let inv = 1.0 / self.determinant();
		Self::new(self.m11 * inv,
		-self.m01 * inv,
		-self.m10 * inv,
		self.m00 * inv)
	}

	fn transpose(&self) -> Matrix2 {
		Self::new(self.m00, self.m01, self.m10, self.m11)
	}

	fn identity() -> Matrix2 {
		Self::new(1.0,0.0,0.0,1.0)
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
        let index = 2*row+col;
        match index {
        	0 => &self.m00,
        	1 => &self.m10,
        	2 => &self.m01,
        	3 => &self.m11,
        	_ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix2 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        let index = 2*row+col;
        match index {
        	0 => &mut self.m00,
        	1 => &mut self.m10,
        	2 => &mut self.m01,
        	3 => &mut self.m11,
        	_ => panic!("Index out of bounds"),
        }
    }
}

impl Mul<f32> for Matrix2 {
	type Output = Self;

	fn mul(self, scalar: f32) -> Self::Output {
		Self::new(self.m00 * scalar,
		self.m01 * scalar,
		self.m10 * scalar,
		self.m11 * scalar)
	}
}

impl Mul<Matrix2> for Matrix2 {
	type Output = Self;

	fn mul(self, other: Matrix2) -> Self::Output {
		Self::new(self.m00 * other[(0,0)] + self.m01 * other[(1,0)],
		self.m00 * other[(0,1)] + self.m01 * other[(1,1)],
		self.m10 * other[(0,0)] + self.m11 * other[(1,0)],
		self.m10 * other[(0,1)] + self.m01 * other[(1,1)])
	}
}


impl Div<f32> for Matrix2 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Matrix2::new(self.m00 / rhs,
		self.m01 / rhs,
		self.m10 / rhs,
		self.m11 / rhs)
	}
}