use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Self { x, y, z, w }
    }

    pub fn dot(&self, rhs: &Vector4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn project(&self, rhs: &Vector4) -> Vector4 {
        *rhs * self.dot(rhs)
    }

    pub fn reject(&self, rhs: &Vector4) -> Vector4 {
        *self - *rhs * self.dot(rhs)
    }

    pub fn normalize(&self) -> Vector4 {
        *self / self.magnitude()
    }

    pub fn normalize_mut(&mut self) {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self.z /= m;
        self.w /= m;
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < 4);
        if i == 0 {
            return &self.x;
        } else if i == 1 {
            return &self.y;
        } else if i == 2 {
            return &self.z;
        }
        &self.w
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        assert!(i < 4);
        if i == 0 {
            return &mut self.x;
        } else if i == 1 {
            return &mut self.y;
        } else if i == 2 {
            return &mut self.z;
        }
        &mut self.w
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, rhs: &Vector4) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z && self.w == rhs.w
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Self;

    fn add(self, rhs: Vector4) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl MulAssign<Vector4> for Vector4 {
    fn mul_assign(&mut self, rhs: Vector4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
