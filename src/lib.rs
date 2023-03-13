pub mod line;
pub mod matrix2;
pub mod matrix3;
pub mod matrix4;
pub mod plane;
pub mod quarternion;
pub mod transform4;
pub mod vector2;
pub mod vector3;
pub mod vector4;
pub mod point2;
pub mod point3;
pub mod rgb;
pub mod rgba;
pub mod rgb_u8;
pub mod rgb_u32;

pub mod prelude {
    pub use crate::line::*;
    pub use crate::matrix2::*;
    pub use crate::matrix3::*;
    pub use crate::matrix4::*;
    pub use crate::plane::*;
    pub use crate::quarternion::*;
    pub use crate::transform4::*;
    pub use crate::vector2::*;
    pub use crate::vector3::*;
    pub use crate::vector4::*;
    pub use crate::point2::*;
    pub use crate::point3::*;
    pub use crate::rgb::*;
    pub use crate::rgba::*;
    pub use crate::rgb_u8::*;
    pub use crate::rgb_u32::*;
}
