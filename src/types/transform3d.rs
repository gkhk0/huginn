use crate::int;
use crate::types::vectors::Vector3;
use crate::types::Basis;
use crate::utils::float;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

/// A 3×4 matrix representing a 3D transformation.
///
/// **Transform3D** is a 3×4 matrix representing a transformation in 3D space. It contains a [`Basis`], which on its own can represent rotation, scale, and shear. Additionally, combined with its own `origin`, the transform can also represent a translation.
#[derive(Copy, Clone, Debug, Default)]
pub struct Transform3D {
    /// The [`Basis`] of this transform. It is composed by 3 axes ([`Basis::x`], [`Basis::y`], and [`Basis::z`]). Together, these represent the transform's rotation, scale, and shear.
    pub basis: Basis,
    /// The translation offset of this transform. In 3D space, this can be seen as the position.
    pub origin: Vector3,
}

impl Transform3D {
    /// A transform with no translation, no rotation, and its scale being `1`. Its `basis` is equal to [`Basis::IDENTITY`].
    ///
    /// When multiplied by another struct such as [`AABB`] or another [`Transform3D`], no transformation occurs.
    pub const IDENTITY: Self = Self::new(Basis::IDENTITY, Vector3::ZERO);

    /// **Transform3D** with mirroring applied perpendicular to the YZ plane. Its `basis` is equal to [`Basis::FLIP_X`].
    pub const FLIP_X: Self = Self::new(Basis::FLIP_X, Vector3::ZERO);

    /// **Transform3D** with mirroring applied perpendicular to the XZ plane. Its `basis` is equal to [`Basis::FLIP_Y`].
    pub const FLIP_Y: Self = Self::new(Basis::FLIP_Y, Vector3::ZERO);

    /// **Transform3D** with mirroring applied perpendicular to the XY plane. Its `basis` is equal to [`Basis::FLIP_Z`].
    pub const FLIP_Z: Self = Self::new(Basis::FLIP_Z, Vector3::ZERO);

    /// Constructs a **Transform3D** from a [`Basis`] and [`Vector3`].
    pub const fn new(basis: Basis, origin: Vector3) -> Self {
        Self { basis, origin }
    }

    fn affine_invert(&mut self) {
        self.basis.invert();
        self.origin = self.basis.xform(&-self.origin);
    }

    /// Returns the inverted version of this transform. Unlike [`inverse`](Transform3D::inverse), this method works with almost any `basis`, including non-uniform ones, but is slower. See also [`Basis::inverse`].
    ///
    /// **Note:** For this method to return correctly, the transform's `basis` needs to have a determinant that is *__not__* exactly `0` (see [`Basis::determinant`]).
    pub fn affine_inverse(&self) -> Self {
        let mut ret = *self;
        ret.affine_invert();
        ret
    }

    /// Returns the result of the linear interpolation between this transform and `xform` by the given `weight`.
    ///
    /// The `weight` should be between `0.0` and `1.0` (inclusive). Values outside this range are allowed and can be used to perform *extrapolation* instead.
    pub fn interpolate_with(&self, xform: &Self, weight: float!()) -> Self {
        let mut interp = Self::default();
        let src_scale = self.basis.get_scale();
        let src_rot = self.basis.get_rotation_quaternion();
        let src_loc = self.origin;

        let dst_scale = xform.basis.get_scale();
        let dst_rot = xform.basis.get_rotation_quaternion();
        let dst_loc = xform.origin;

        interp.basis.set_quaternion_scale(
            &src_rot.slerp(&dst_rot, weight).normalized(),
            &src_scale.lerp(&dst_scale, weight),
        );
        interp.origin = src_loc.lerp(&dst_loc, weight);
        interp
    }

    fn invert(&mut self) {
        self.basis = self.basis.transposed();
        self.origin = self.basis.xform(&-self.origin);
    }
    /// Returns the inverted version of this transform. See also [`Basis::inverse`].
    ///
    /// **Note:** For this method to return correctly, the transform's `basis` needs to be *orthonormal* (see [`Basis::orthonormalized`]). That means, the basis should only represent a rotation. If it does not, use [`affine_inverse`](Transform3D::affine_inverse) instead.
    pub fn inverse(&self) -> Self {
        // FIXME: this function assumes the basis is a rotation matrix, with no scaling.
        // Transform3D::affine_inverse can handle matrices with scaling, so that should eventually be used.
        let mut ret = *self;
        ret.invert();
        ret
    }

    /// Returns `true` if this transform and `xform` are approximately equal, by running [`is_equal_approx`](crate::utils::is_equal_approx) on each component.
    pub fn is_equal_approx(&self, xform: &Self) -> bool {
        self.basis.is_equal_approx(&xform.basis) && self.origin.is_equal_approx(&xform.origin)
    }

    /// Returns `true` if this transform is finite, by calling `is_finite` on each component.
    pub fn is_finite(&self) -> bool {
        self.basis.is_finite() && self.origin.is_finite()
    }

    /// Returns a copy of this transform rotated so that the forward axis (-Z) points `towards` the target position.
    ///
    /// The up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The resulting transform is orthonormalized. The existing rotation, scale, and skew information from the original transform is discarded. The `target` and `up` vectors cannot be zero, cannot be parallel to each other, and are defined in global/parent space.
    ///
    /// If `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position. By default, the -Z axis (camera forward) is treated as forward (implies +X is right).
    fn looking_at(&self, target: &Vector3, up: Option<&Vector3>, use_model_front: bool) -> Self {
        let mut t = *self;
        t.basis = Basis::looking_at(&(target - self.origin), up, use_model_front);
        t
    }

    fn orthonormalize(&mut self) {
        self.basis.orthonormalize();
    }

    /// Returns a copy of this transform with its `basis` orthonormalized. An orthonormal basis is both *orthogonal* (the axes are perpendicular to each other) and *normalized* (the axes have a length of `1`), which also means it can only represent rotation. See also [`Basis::orthonormalized`].
    pub fn orthonormalized(&self) -> Self {
        let mut ret = *self;
        ret.orthonormalize();
        ret
    }

    /// Returns a copy of this transform rotated around the given `axis` by the given `angle` (in radians).
    ///
    /// The `axis` must be a normalized vector.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the left, i.e., `R * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn rotated(&self, axis: &Vector3, angle: float!()) -> Self {
        // Equivalent to left multiplication
        let basis = Basis::from((axis, angle));
        Self::new(basis * self.basis, basis.xform(&self.origin))
    }

    /// Returns a copy of this transform rotated around the given `axis` by the given `angle` (in radians).
    ///
    /// The `axis` must be a normalized vector.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the right, i.e., `X * R`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn rotated_local(&self, axis: &Vector3, angle: float!()) -> Self {
        // Equivalent to right multiplication
        let basis = Basis::from((axis, angle));
        Self::new(self.basis * basis, self.origin)
    }

    /// Returns a copy of this transform scaled by the given `scale` factor.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the left, i.e., `S * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn scaled(&self, scale: &Vector3) -> Self {
        // Equivalent to left multiplication
        Self::new(self.basis.scaled(scale), self.origin * scale)
    }

    /// Returns a copy of this transform scaled by the given `scale` factor.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the right, i.e., `X * S`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn scaled_local(&self, scale: &Vector3) -> Self {
        // Equivalent to right multiplication
        Self::new(self.basis.scaled_local(scale), self.origin)
    }

    /// Returns a copy of this transform translated by the given `offset`.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the left, i.e., `T * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn translated(&self, offset: &Vector3) -> Self {
        // Equivalent to left multiplication
        Self::new(self.basis, self.origin + offset)
    }

    /// Returns a copy of this transform translated by the given `offset`.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the right, i.e., `X * T`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn translated_local(&self, offset: &Vector3) -> Self {
        // Equivalent to right multiplication
        Self::new(self.basis, self.origin + self.basis.xform(&offset))
    }

    pub fn xform(&self, vec: &Vector3) -> Vector3 {
        Vector3::new(
            self.basis.x.dot(&vec) + self.origin.x,
            self.basis.y.dot(&vec) + self.origin.y,
            self.basis.z.dot(&vec) + self.origin.z,
        )
    }
}

impl PartialEq for Transform3D {
    fn eq(&self, other: &Self) -> bool {
        self.basis == other.basis && self.origin == other.origin
    }
}

impl_op_ex!(*= |a: &mut Transform3D, b: &Transform3D| {
    a.origin = a.xform(&b.origin);
    a.basis *= b.basis;
});
impl_op_ex!(*|a: &Transform3D, b: &Transform3D| -> Transform3D {
    let mut t = *a;
    t *= b;
    t
});

impl_op_ex_commutative!(*|a: &Transform3D, b: &Vector3| -> Vector3 {
    let mut ret = *b;
    ret.x = b.x * a.basis.x.x + b.y * a.basis.y.x + b.z * a.basis.z.x;
    ret.y = b.x * a.basis.x.y + b.y * a.basis.y.y + b.z * a.basis.z.y;
    ret.z = b.x * a.basis.x.z + b.y * a.basis.y.z + b.z * a.basis.z.z;
    ret += a.origin;
    ret
});

impl_op_ex_commutative!(*|a: &Transform3D, b: &Vec<Vector3>| -> Vec<Vector3> {
    b.iter().map(|&i| i * a).collect()
});

impl_op_ex!(*= |a: &mut Transform3D, b: &float!()|{
    a.basis *= b;
    a.origin *= b;
});
impl_op_ex_commutative!(*|a: &Transform3D, b: &float!()| -> Transform3D {
    let mut ret = *a;
    ret *= b;
    ret
});
impl_op_ex!(*= |a: &mut Transform3D, b: int!()| {
    *a *= b as float!();
});
impl_op_ex_commutative!(*|a: &Transform3D, b: int!()| -> Transform3D {
    let mut ret = *a;
    ret *= b;
    ret
});

impl_op_ex!(/= |a: &mut Transform3D, b: &float!()|{
    a.basis /= b;
    a.origin /= b;
});
impl_op_ex_commutative!(/ |a: &Transform3D, b: &float!()| -> Transform3D {
    let mut ret = *a;
    ret /= b;
    ret
});
impl_op_ex!(/= |a: &mut Transform3D, b: int!()| {
    *a /= b as float!();
});
impl_op_ex_commutative!(/ |a: &Transform3D, b: int!()| -> Transform3D {
    let mut ret = *a;
    ret /= b;
    ret
});
