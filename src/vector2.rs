use crate::prelude::*;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn project(&self, other: &Vector2) -> Vector2 {
        *other * self.dot(other)
    }

    pub fn reject(&self, other: &Vector2) -> Vector2 {
        *self - *other * self.dot(other)
    }

    pub fn normalize(&self) -> Vector2 {
        *self / self.magnitude()
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Index out of bounds");
        }
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        if index > 2 {
            panic!("Index out of bounds");
        }
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<Matrix2> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        Vector2::new(
            rhs[(0, 0)] * self.x + rhs[(0, 1)] * self.y,
            rhs[(1, 0)] * self.x + rhs[(1, 1)] * self.y,
        )
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    fn new(x: f32, y: f32) -> Point2 {
        Point2 { x, y }
    }
}

impl Add<Vector2> for Point2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector2> for Point2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vector2;

    fn sub(self, rhs: Point2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

impl Default for Point2 {
    fn default() -> Point2 {
        Point2 { x: 0.0, y: 0.0 }
    }
}
