mod basis;
mod quaternion;
mod transform2d;
mod transform3d;
/// A module containing different vector structs.
pub mod vectors;

pub use basis::Basis;
pub use quaternion::Quaternion;
pub use transform2d::Transform2D;
pub use transform3d::Transform3D;

#[derive(Clone, Copy)]
pub enum EulerOrder {
    /// Specifies that Euler angles should be in XYZ order. When composing, the order is X, Y, Z. When decomposing, the order is reversed, first Z, then Y, and X last.
    XYZ,
    /// Specifies that Euler angles should be in XZY order. When composing, the order is X, Z, Y. When decomposing, the order is reversed, first Y, then Z, and X last.
    XZY,
    /// Specifies that Euler angles should be in YXZ order. When composing, the order is Y, X, Z. When decomposing, the order is reversed, first Z, then X, and Y last.
    YXZ,
    /// Specifies that Euler angles should be in YZX order. When composing, the order is Y, Z, X. When decomposing, the order is reversed, first X, then Z, and Y last.
    YZX,
    /// Specifies that Euler angles should be in ZXY order. When composing, the order is Z, X, Y. When decomposing, the order is reversed, first Y, then X, and Z last.
    ZXY,
    /// Specifies that Euler angles should be in ZYX order. When composing, the order is Z, Y, X. When decomposing, the order is reversed, first X, then Y, and Z last.
    ZYX,
}
