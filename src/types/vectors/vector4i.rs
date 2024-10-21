use crate::utils::int;

#[derive(Copy, Clone, Default)]
pub struct Vector4i {
    pub w: int!(),
    pub x: int!(),
    pub y: int!(),
    pub z: int!(),
}
