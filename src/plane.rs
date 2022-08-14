use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};
use crate::prelude::*;

pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Plane {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Plane {
        Plane { x, y, z, w }
    }

    pub fn new_with_vec(v: &Vector3, d: f32) -> Plane {
        Plane {
            x: v.x,
            y: v.y,
            z: v.z,
            w: d,
        }
    }

    pub fn get_normal(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.x)
    }

    pub fn vec_dot(&self, v: &Vector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn point_dot(&self, v: &Point3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w
    }
}

impl Mul<Transform4> for Plane {
    type Output = Self;

    fn mul(self, rhs: Transform4) -> Self::Output {
        Plane::new(
            self.x * rhs[(0, 0)] + self.y * rhs[(1, 0)] + self.z * rhs[(2, 0)],
            self.x * rhs[(0, 1)] + self.y * rhs[(1, 1)] + self.z * rhs[(2, 1)],
            self.x * rhs[(0, 2)] + self.y * rhs[(1, 2)] + self.z * rhs[(2, 2)],
            self.x * rhs[(0, 3)] + self.y * rhs[(1, 3)] + self.z * rhs[(2, 3)] + self.w,
        )
    }
}

// calculates the point p at which three planes f1, f2, and f3 intersect and returns
// true if such a point exists. Returns false if the normal vectors are not linearly independent
pub fn three_planes_intersect(f1: &Plane, f2: &Plane, f3: &Plane, &mut p: Point3) -> bool {
    let n1 = f1.get_normal();
    let n2 = f2.get_normal();
    let n3 = f3.get_normal();
    let n1xn2 = n1.cross(&n2);
    let det = n1xn2.dot(&n3);
    if det.fabs() > f32::MIN {
        p = (n3.cross(&n2) * f1.w + n1.cross(&n3) * f2.w - n1xn2 * f3.w) / det;
        true
    } else {
        false
    }
}

// calculates the line determined by the point p and the direction v at which
// two planes f1 and f2 intersect and returns true if it exists. Returns false
// if the normal vectors are parallel.
pub fn two_planes_intersect(f1: &Plane, f2: &Plane, &mut p: Point3, &mut v: Vector3) -> bool {
    let n1 = f1.get_normal();
    let n2 = f2.get_normal();

    v = n1.cross(&n2);
    let det = v.dot(v);
    if det.fabs() > f32::MIN {
        p = (v.cross(&n2) * f1.w + n1.cross(v) * f2.w) / det;
        true
    } else {
        false
    }
}
