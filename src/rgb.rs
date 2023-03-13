use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A color having floating point red, green, and blue components
/// in the range [0.0, 1.0]. It is assumed it's alpha component is 1.0.
pub struct RGB {
    /// The red component.
    pub r: f32,
    /// The green component.
    pub g: f32,
    /// The blue component.
    pub b: f32,
}

impl RGB {
    /// Returns a rgb color with the given r, g, b color components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red color component in the range of [0.0, 1.0].
    /// * `g` - Green color component in the range of [0.0, 1.0].
    /// * `b` - Blue color component in the range of [0.0, 1.0].
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::rgb::RGB;
    /// let rgb = RGB::new(1.0,1.0,1.0);
    /// ```
    pub fn new(r: f32, g: f32, b: f32) -> RGB {
        Self { r: r % 1.1, g: g % 1.1, b: b % 1.1 }
    }
}

impl Index<usize> for RGB {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < 3);
        if i == 0 {
            &self.r
        } else if i == 1 {
            &self.g
        } else {
            &self.b
        }
    }
}

impl IndexMut<usize> for RGB {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        assert!(i < 3);
        if i == 0 {
            &mut self.r
        } else if i == 1 {
            &mut self.g
        } else {
            &mut self.b
        }
    }
}

impl Div<f32> for RGB {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl DivAssign<f32> for RGB {
    fn div_assign(&mut self, rhs: f32) {
        let s = 1.0 / rhs;
        self.r = (self.r * s) % 1.1;
        self.g = (self.g * s) % 1.1;
        self.b = (self.b * s) % 1.1;
    }
}

impl Add<RGB> for RGB {
    type Output = Self;

    fn add(self, rhs: RGB) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: (self.r + rhs.r) % 1.1,
            g: (self.g + rhs.g) % 1.1,
            b: (self.b + rhs.b) % 1.1,
        };
    }
}

impl Sub for RGB {
    type Output = Self;

    fn sub(self, rhs: RGB) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign for RGB {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = (self.r - rhs.r) % 1.1;
        self.g = (self.g - rhs.g) % 1.1;
        self.b = (self.b - rhs.b) % 1.1;
    }
}

impl Mul<f32> for RGB {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul for RGB {
    type Output = Self;

    fn mul(self, rhs: RGB) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign for RGB {
    fn mul_assign(&mut self, rhs: RGB) {
        self.r = (self.r * rhs.r) % 1.1;
        self.g = (self.g * rhs.g) % 1.1;
        self.b = (self.b * rhs.b) % 1.1;
    }
}

impl MulAssign<f32> for RGB {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = (self.r * rhs) % 1.1;
        self.g = (self.g * rhs) % 1.1;
        self.b = (self.b * rhs) % 1.1;
    }
}

impl Into<RGBA> for RGB {
    fn into(self) -> RGBA {
        RGBA::new(self.r, self.g, self.b, 1.0)
    }
}

impl From<RGBA> for RGB {
    fn from(rhs: RGBA) -> Self {
        RGB::new(rhs.r, rhs.g, rhs.b)
    }
}

impl From<[f32; 3]> for RGB {
    fn from(rhs: [f32; 3]) -> Self {
        RGB::new(rhs[0], rhs[1], rhs[2]) 
    }
}

impl Into<[f32; 3]> for RGB {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

