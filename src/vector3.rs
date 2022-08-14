use crate::prelude::*;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.z,
        )
    }

    pub fn project(&self, other: &Vector3) -> Vector3 {
        *other * self.dot(other)
    }

    pub fn reject(&self, other: &Vector3) -> Vector3 {
        *self - *other * self.dot(other)
    }

    pub fn normalize(&self) -> Vector3 {
        *self / self.magnitude()
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Matrix3> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Self::new(
            rhs[(0, 0)] * self.x + rhs[(0, 1)] * self.y + rhs[(0, 2)] * self.z,
            rhs[(1, 0)] * self.x + rhs[(1, 1)] * self.y + rhs[(1, 2)] * self.z,
            rhs[(2, 0)] * self.x + rhs[(2, 1)] * self.y + rhs[(2, 2)] * self.z,
        )
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Default for Vector3 {
    fn default() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Default for Point3 {
    fn default() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Mul<Matrix4> for Vector3 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Vector4 {
        Vector4::new(
            rhs[(0, 0)] * self.x + rhs[(0, 1)] * self.y + rhs[(2, 0)] * self.z,
            rhs[(0, 1)] * self.x + rhs[(1, 1)] * self.y + rhs[(2, 1)] * self.z,
            rhs[(0, 2)] * self.x + rhs[(1, 2)] * self.y + rhs[(2, 2)] * self.z,
            rhs[(0, 3)] * self.x + rhs[(1, 3)] * self.y + rhs[(2, 3)] * self.z,
        )
    }
}

impl Mul<Matrix4> for Point3 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Vector4::new(
            rhs[(0, 0)] * self.x + rhs[(0, 1)] * self.y + rhs[(2, 0)] * self.z + rhs[(3, 0)],
            rhs[(0, 1)] * self.x + rhs[(1, 1)] * self.y + rhs[(2, 1)] * self.z + rhs[(3, 1)],
            rhs[(0, 2)] * self.x + rhs[(1, 2)] * self.y + rhs[(2, 2)] * self.z + rhs[(3, 2)],
            rhs[(0, 3)] * self.x + rhs[(1, 3)] * self.y + rhs[(2, 3)] * self.z + rhs[(3, 3)],
        )
    }
}

// Returns the distance between the point q and the line determined by the point
// p and the direction v.
pub fn point_line_distance(q: &Point3, p: &Point3, v: &Vector3) -> f32 {
    let a = (*q - *p).cross(&v);
    f32::sqrt(a.dot(&a) / v.dot(&v))
}

// Returns the distance between two lines determined by the points p1 and p2 and the
// directions v1 and v2.
pub fn line_line_distance(p1: &Point3, v1: &Vector3, p2: &Point3, v2: &Vector3) -> f32 {
    let dp = *p2 - *p1;

    let v12 = v1.dot(&v1);
    let v22 = v2.dot(&v2);
    let v1v2 = v1.dot(&v2);

    let mut det = v1v2 * v1v2 - v12 * v22;

    if det.abs() > f32::MIN {
        det = 1.0 / det;

        let dpv1 = dp.dot(&v1);
        let dpv2 = dp.dot(&v2);
        let t1 = (v1v2 * dpv2 - v22 * dpv1) * det;
        let t2 = (v12 * dpv2 - v1v2 * dpv1) * det;
        (dp + *v2 * t2 - *v1 * t1).magnitude()
    } else {
        let a = dp.cross(&v1);
        f32::sqrt(a.dot(&a) / v12)
    }
}

// Calculates the point q at which the line determined by p and v intersects
// the plane f and returns true if such a point exists and false if v is parallel
// to the plane.
pub fn plane_line_intersect(p: &Point3, v: &Vector3, f: &Plane, mut q: Point3) -> bool {
    let fv = f.vec_dot(v);
    if fv.abs() > f32::MIN {
        q = *p - *v * (f.point_dot(p) / fv);
        true
    } else {
        false
    }
}
