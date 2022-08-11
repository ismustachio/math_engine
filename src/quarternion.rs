use crate::prelude::*;

pub struct Quarternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quarternion {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Quarternion {
        Quarternion { x, y, z, w }
    }

    fn new_with_vec(a: &Vector3, s: f32) -> Quarternion {
        Quarternion {
            x: a.x,
            y: a.y,
            z: a.z,
            w: s,
        }
    }

    fn get_vector_part(&self) -> Vector3 {
        Vector3 {
            self.x,
            self.x,
            self.x,
        }
    }

    fn get_rotation_matrix(&self) -> Matrix3 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
    }

    fn set_rotation_matrix(&mut self, matrix: &Matrix3) {

    }

    fn transform(&self, v: &Vector3) -> Vector3 {
        let b = self.get_vector_part();
        let b2 = b.x * b.x + b.y * b.y + b.z * b.z;
        (v * (self.w * self.w - b2) + b * (v.dot(&b) * 2.0) + b.cross(&v) * (self.w * 2.0))
    }
}

impl Mul<Quarternion> for Quarternion {
    type Output = Self;

    fn mul(self, rhs: Quarternion) -> Self::Output {
        Quarternion::new(self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
                         self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x, 
                         self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w, 
                         self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z)
    }
}
