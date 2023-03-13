use crate::prelude::*;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Default, Copy, Clone, Debug)]
/// A three dimensional positional vector having float components
/// x, and y. It's w coordinated it's assumed to be 0.
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    /// Returns a positional vector initialized with the floating point components x, and y.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinates.
    /// * `y` - The y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::point2::Point2;
    /// let p = Point2::new(1.0,0.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Point2 {
        Point2 { x, y }
    }
}

impl Add<Vector2> for Point2 {
    type Output = Self;

    fn add(self, other: Vector2) -> Self::Output {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2> for Point2 {
    type Output = Self;

    fn sub(self, other: Vector2) -> Self::Output {
        Point2::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vector2;

    fn sub(self, other: Point2) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<Matrix4> for Vector2 {
    type Output = Vector4;

    fn mul(self, other: Matrix4) -> Self::Output {
        Vector4::new(
            other[(0, 0)] * self.x + other[(0, 1)] * self.y,
            other[(0, 1)] * self.x + other[(1, 1)] * self.y,
            other[(0, 2)] * self.x + other[(1, 2)] * self.y,
            other[(0, 3)] * self.x + other[(1, 3)] * self.y,
        )
    }
}

impl Mul<Matrix4> for Point2 {
    type Output = Vector4;

    fn mul(self, other: Matrix4) -> Self::Output {
        Vector4::new(
            other[(0, 0)] * self.x + other[(0, 1)] * self.y + other[(3, 0)],
            other[(0, 1)] * self.x + other[(1, 1)] * self.y + other[(3, 1)],
            other[(0, 2)] * self.x + other[(1, 2)] * self.y + other[(3, 2)],
            other[(0, 3)] * self.x + other[(1, 3)] * self.y + other[(3, 3)],
        )
    }
}
