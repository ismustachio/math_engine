use crate::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
/// A color having floating point red, green, blue, and alpha components
/// in the range [0.0, 1.0].
pub struct RGBA {
    /// The red component.
    pub r: f32,
    /// The green component.
    pub g: f32,
    /// The blue component.
    pub b: f32,
    /// The alpha component.
    pub a: f32,
}

impl RGBA {
    /// Returns a rgb color with the given r, g, b, a color components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red color component in the range of [0.0, 1.0].
    /// * `g` - Green color component in the range of [0.0, 1.0].
    /// * `b` - Blue color component in the range of [0.0, 1.0].
    /// * `a` - Alpha color component in the range of [0.0, 1.0].
    ///
    /// # Examples
    ///
    /// ```
    /// use math_engine::rgba::RGBA;
    /// let rgba = RGBA::new(1.0,1.0,1.0,1.0);
    /// ```
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> RGBA {
        Self {
            r: r % 1.1,
            g: g % 1.1,
            b: b % 1.1,
            a: a % 1.1,
        }
    }
}
