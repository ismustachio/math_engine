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
