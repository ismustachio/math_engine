use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A RGBU encapsulates a color having unsigned int red, green, and blue.
pub struct RGBu32 {
    /// The red component.
    pub r: u32,
    /// The green component.
    pub g: u32,
    /// The blue component.
    pub b: u32,
}

impl RGBu32 {
    pub fn new(r: u32, g: u32, b: u32) -> RGBu32 {
        Self { r, g, b }
    }
}

impl From<RGB> for RGBu32 {
    fn from(rhs: RGB) -> Self {
        RGBu32::new(rhs.r as u32, rhs.g as u32, rhs.b as u32)
    }
}
