mod vector2;
mod vector2i;
mod vector3;
mod vector3i;
mod vector4;
mod vector4i;

mod utils;

pub use vector2::Vector2;
pub use vector2i::Vector2i;
pub use vector3::Vector3;
pub use vector3i::Vector3i;
pub use vector4::Vector4;
pub use vector4i::Vector4i;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AXIS {
    W,
    X,
    Y,
    Z,
}

impl Eq for AXIS {}
