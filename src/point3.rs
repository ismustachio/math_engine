use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A three dimensional positional vector having float components
/// x, y, and z. It's w coordinated it's assumed to be 0.
pub struct Point3 {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
    /// The z component.
    pub z: f32,
}

impl Point3 {
    /// Returns a positional vector initialized with the floating point components x, y, and z.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinates.
    /// * `y` - The y coordinates.
    /// * `z` - The z coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::point3::Point3;
    /// let point3 = Point3::new(1.0,0.0,0.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self::Output {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Point3) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Matrix4> for Vector3 {
    type Output = Vector4;

    fn mul(self, other: Matrix4) -> Vector4 {
        Vector4::new(
            other[(0, 0)] * self.x + other[(0, 1)] * self.y + other[(2, 0)] * self.z,
            other[(0, 1)] * self.x + other[(1, 1)] * self.y + other[(2, 1)] * self.z,
            other[(0, 2)] * self.x + other[(1, 2)] * self.y + other[(2, 2)] * self.z,
            other[(0, 3)] * self.x + other[(1, 3)] * self.y + other[(2, 3)] * self.z,
        )
    }
}

impl Mul<Matrix4> for Point3 {
    type Output = Vector4;

    fn mul(self, other: Matrix4) -> Self::Output {
        Vector4::new(
            other[(0, 0)] * self.x
                + other[(0, 1)] * self.y
                + other[(2, 0)] * self.z
                + other[(3, 0)],
            other[(0, 1)] * self.x
                + other[(1, 1)] * self.y
                + other[(2, 1)] * self.z
                + other[(3, 1)],
            other[(0, 2)] * self.x
                + other[(1, 2)] * self.y
                + other[(2, 2)] * self.z
                + other[(3, 2)],
            other[(0, 3)] * self.x
                + other[(1, 3)] * self.y
                + other[(2, 3)] * self.z
                + other[(3, 3)],
        )
    }
}

impl From<Vector3> for Point3 {
    fn from(p: Vector3) -> Self {
        Point3 {
            x: p.x,
            y: p.y,
            z: p.z,
        }
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
pub fn plane_line_intersect(p: &Point3, v: &Vector3, f: &Plane) -> Option<Point3> {
    let fv = f.vec_dot(v);
    if fv.abs() > f32::MIN {
        Some(*p - *v * (f.point_dot(p) / fv))
    } else {
        None
    }
}
