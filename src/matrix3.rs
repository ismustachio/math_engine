use std::ops::{Div, Index, IndexMut, Mul};
use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
	m00: f32,
	m01: f32,
	m02: f32,
	m10: f32,
	m11: f32,
	m12: f32,
	m20: f32,
	m21: f32,
	m22: f32,
}

impl Matrix3 {
	fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Matrix3 {
		Self {
			m00: a,
			m10: b,
			m20: c,
			m01: d,
			m11: e,
			m21: f,
			m02: g,
			m12: h,
			m22: i,
		}
	}

	fn new_with_vecs(a: &Vector3, b: &Vector3, c: &Vector3) -> Matrix3 {
		Self {
			m00: a.x,
			m10: a.y,
			m20: a.x,
			m01: b.x,
			m11: b.y,
			m21: b.z,
			m02: c.x,
			m12: c.y,
			m22: c.y,
		}
	}

	fn vec_at(&self, index: usize) -> Vector3 {
    Vector3::new(self[(index, 0)], self[(index, 1)], self[(index, 2)])
	}

	fn determinant(&self) -> f32 {
		(self.m00 * self.m11 * self.m22 - self.m21 * self.m12) - self.m10 * (self.m01 * self.m22 - self.m21 * self.m20) + self.m02 * (self.m01 * self.m12 - self.m11 * self.m02)
	}

	fn inverse(&self) -> Matrix3 {
		let a = self.vec_at(0);
		let b = self.vec_at(1);
		let c = self.vec_at(2);
		let r0 = b.cross(&c);
		let r1 = c.cross(&a);
		let r2 = a.cross(&b);
		let inv = 1.0 / r2.dot(&c);
		Self::new(r0.x * inv,
		r0.y * inv,
		r0.z * inv,
		r1.x * inv,
		r1.y * inv,
		r1.z * inv,
		r2.x * inv,
		r2.y * inv,
		r2.z * inv)
	}

	fn transpose(&self) -> Matrix3 {
		Self::new(self.m00,self.m01,self.m02,
		self.m10,self.m11, self.m12,
		self.m20,self.m21,self.m22)
	}

	fn identity() -> Matrix3 {
		Self::new(1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0)
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
		Self::new(c + x * v.x,
		axay - s * v.z,
		axaz + s * v.y,
		axay + s * v.z,
		c + y * v.y,
		ayaz - s * v.x,
		axaz - s * v.y,
		ayaz + s * v.x,
		c + z * v.z)
	}

	fn make_rotation_x(angle: f32) -> Matrix3 {
		let c = angle.cos();
		let s = angle.sin();
		Self::new(1.0,0.0,0.0,0.0,c,-s,0.0,s,c)
	}

	fn make_rotation_y(angle: f32) -> Matrix3 {
		let c = angle.cos();
		let s = angle.sin();
		Self::new(c,0.0,s,0.0,1.0,0.0,-s,0.0,s)
	}

	fn make_rotation_z(angle: f32) -> Matrix3 {
		let c = angle.cos();
		let s = angle.sin();
		Self::new(c,0.0,s,0.0,1.0,0.0,-s,0.0,s)
	}

	fn make_skew(angle: f32, a: &Vector3, b: &Vector3) -> Matrix3 {
		let t = angle.tan();
		let x = a.x * t;
		let y = a.y * t;
		let z = a.z * t;
		Self::new(x * b.x + 1.0, x * b.y, x * b.z,
		y * b.x + 1.0, y * b.y, y * b.z,
		z * b.x + 1.0, z * b.y, z * b.z)
	}

	fn make_scale(sx: f32, sy: f32, sz: f32) -> Matrix3 {
		Self::new(sx,0.0,0.0,0.0,sy,0.0,0.0,0.0,sz)
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

   	Self::new(x * a.x + 1.0, axay, axaz, axay, y * a.y + 1.0, ayaz, axaz, ayaz, z * a.z + 1.0)
  }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let index = 3*col+row;
        match index {
        	0 => &self.m00,
        	1 => &self.m10,
        	2 => &self.m20,
        	3 => &self.m01,
        	4 => &self.m11,
        	5 => &self.m21,
        	6 => &self.m02,
        	7 => &self.m12,
        	8 => &self.m22,
        	_ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix3 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        let index = 3*col+row;
        match index {
        	0 => &mut self.m00,
        	1 => &mut self.m10,
        	2 => &mut self.m20,
        	3 => &mut self.m01,
        	4 => &mut self.m11,
        	5 => &mut self.m21,
        	6 => &mut self.m02,
        	7 => &mut self.m12,
        	8 => &mut self.m22,
        	_ => panic!("Index out of bounds"),
        }
    }
}

impl Mul<f32> for Matrix3 {
	type Output = Self;

	fn mul(self, scalar: f32) -> Self::Output {
		Self::new(self.m00 * scalar,
		self.m10 * scalar,
		self.m20 * scalar,
		self.m01 * scalar,
		self.m11 * scalar,
		self.m21 * scalar,
		self.m02 * scalar,
		self.m12 * scalar,
		self.m22 * scalar)
	}
}

impl Mul<Matrix3> for Matrix3 {
	type Output = Self;

	fn mul(self, other: Matrix3) -> Self::Output {
		Self::new(self.m00 * other[(0,0)] + self.m10 * other[(1,0)] + self.m20 * other[(2,0)],
		self.m00 * other[(0,1)] + self.m10 * other[(1,1)] + self.m20 * other[(2,1)],
		self.m00 * other[(0,2)] + self.m10 * other[(1,2)] + self.m20 * other[(2,2)],
		self.m01 * other[(0,0)] + self.m11 * other[(1,0)] + self.m21 * other[(2,0)],
		self.m01 * other[(0,1)] + self.m11 * other[(1,1)] + self.m21 * other[(2,1)],
		self.m01 * other[(0,2)] + self.m11 * other[(1,2)] + self.m21 * other[(2,2)],
		self.m02 * other[(0,0)] + self.m12 * other[(1,0)] + self.m22 * other[(2,0)],
		self.m12 * other[(0,1)] + self.m12 * other[(1,1)] + self.m22 * other[(2,1)],
		self.m12 * other[(0,2)] + self.m12 * other[(1,2)] + self.m22 * other[(2,2)])
	}
}

impl Div<f32> for Matrix3 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		let s = 1.0 / rhs;
		Self::new(self.m00 / s,
		self.m01 / s,
		self.m02 / s,
		self.m10 / s,
		self.m11 / s,
		self.m12 / s,
		self.m20 / s,
		self.m21 / s,
		self.m22 / s)
	}
}