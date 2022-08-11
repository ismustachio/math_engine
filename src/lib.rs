mod vector2;
mod vector3;
mod matrix2;
mod matrix3;

mod prelude {
	pub use crate::vector2::*;
	pub use crate::vector3::*;
	pub use crate::matrix2::*;
	pub use crate::matrix3::*;
}

use prelude::*;