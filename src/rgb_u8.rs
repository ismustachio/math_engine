use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A RGBU encapsulates a color having unsigned byte red, green, and blue.
pub struct RGBu8 {
    /// The red component.
    pub r: u8,
    /// The green component.
    pub g: u8,
    /// The blue component.
    pub b: u8,
}

impl RGBu8 {
    pub fn new(r: u8, g: u8, b: u8) -> RGBu8 {
        Self { r, g, b }
    }
}

impl From<RGB> for RGBu8 {
    fn from(rhs: RGB) -> Self {
        let r = ((rhs.r * 255.00)) as u8;
        let g = ((rhs.g * 255.00)) as u8;
        let b = ((rhs.b * 255.00)) as u8;
        RGBu8::new(r, g, b)
    }
}
