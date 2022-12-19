mod line;
mod matrix2;
mod matrix3;
mod matrix4;
mod plane;
mod quarternion;
mod transform4;
mod vector2;
mod vector3;
mod vector4;

mod prelude {
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
}

use prelude::*;
