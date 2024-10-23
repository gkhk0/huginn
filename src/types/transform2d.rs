use crate::types::vectors::Vector2;
use crate::utils::{float, float_consts, int, is_equal_approx, FloatExt};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::mem::swap;
use std::ops::Not;

/// A 2×3 matrix representing a 2D transformation.
///
/// Transform2D is a 2×3 [matrix](https://en.wikipedia.org/wiki/Matrix_(mathematics)) representing a transformation in 2D space. It contains three [`Vector2`] values: `x`, `y`, and `origin`. Together, they can represent translation, rotation, scale, and skew.
///
/// The `x` and `y` axes form a 2×2 matrix, known as the transform's **basis**. The length of each axis ([`Vector2::length`]) influences the transform's scale, while the direction of all axes influence the rotation. Usually, both axes are perpendicular to one another. However, when you rotate one axis individually, the transform becomes skewed. Applying a skewed transform to a 2D sprite will make the sprite appear distorted.
///
/// **Note:** Unlike [`Transform3D`], there is no 2D equivalent to the [`Basis`] type. All mentions of "basis" refer to the `x` and `y` components of **Transform2D**.
#[derive(Copy, Clone, Debug)]
pub struct Transform2D {
    /// The translation offset of this transform, and the column `2` of the matrix. In 2D space, this can be seen as the position.
    pub origin: Vector2,
    /// The transform basis's X axis, and the column `0` of the matrix. Combined with `y`, this represents the transform's rotation, scale, and skew.
    ///
    /// On the identity transform, this vector points right ([`Vector2::RIGHT`]).
    pub x: Vector2,
    /// The transform basis's Y axis, and the column `1` of the matrix. Combined with `x`, this represents the transform's rotation, scale, and skew.
    ///
    /// On the identity transform, this vector points down ([`Vector2::DOWN`]).
    pub y: Vector2,
}

impl Default for Transform2D {
    /// Constructs a **Transform2D** identical to [`Transform2D::IDENTITY`].
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform2D {
    /// The identity Transform2D. A transform with no translation, no rotation, and its scale being `1`. When multiplied by another struct such as [`Rect2`] or another [`Transform2D`], no transformation occurs. This means that:
    ///
    /// - The x points right ([`Vector2::RIGHT`]);
    /// - The y points down ([`Vector2::DOWN`]).
    pub const IDENTITY: Self = Self::new(Vector2::RIGHT, Vector2::DOWN, Vector2::ZERO);

    /// When any transform is multiplied by `FLIP_X`, it negates all components of the `x` axis (the X column).
    ///
    /// When `FLIP_X` is multiplied by any basis, it negates the [`Vector2::x`] component of all axes (the X row).
    pub const FLIP_X: Self = Self::new(Vector2::LEFT, Vector2::DOWN, Vector2::ZERO);

    /// When any transform is multiplied by `FLIP_Y`, it negates all components of the `y` axis (the Y column).
    ///
    /// When `FLIP_Y` is multiplied by any basis, it negates the [`Vector2::y`] component of all axes (the Y row).
    pub const FLIP_Y: Self = Self::new(Vector2::RIGHT, Vector2::UP, Vector2::ZERO);

    /// Constructs a **Transform2D** from 3 [`Vector2`] values representing `x`, `y`, and the `origin` (the three matrix columns).
    pub const fn new(x: Vector2, y: Vector2, origin: Vector2) -> Self {
        Self { origin, x, y }
    }

    pub const fn new_from_floats(
        xx: float!(),
        xy: float!(),
        yx: float!(),
        yy: float!(),
        origin_x: float!(),
        origin_y: float!(),
    ) -> Self {
        Self::new(
            Vector2::new(xx, xy),
            Vector2::new(yx, yy),
            Vector2::new(origin_x, origin_y),
        )
    }

    pub fn basis_xform(&self, vec: &Vector2) -> Vector2 {
        Vector2::new(self.tdotx(vec), self.tdoty(vec))
    }

    fn affine_invert(&mut self) {
        let det = self.determinant();

        let idet = 1.0 / det;

        swap(&mut self.x.x, &mut self.y.y);
        self.x *= Vector2::new(idet, -idet);
        self.y *= Vector2::new(-idet, idet);

        self.origin = self.basis_xfrom(&-self.origin);
    }

    /// Returns the inverted version of this transform. Unlike [`Transform2D::inverse`], this method works with almost any basis, including non-uniform ones, but is slower. See also [`Transform2D::inverse`].
    ///
    /// **Note:** For this method to return correctly, the transform's basis needs to have a determinant that is not exactly `0` (see [`Transform2D::determinant`]).
    pub fn affine_inverse(&self) -> Self {
        let mut inv = *self;
        inv.affine_invert();
        inv
    }

    /// Returns a copy of the `v` vector, transformed (multiplied) by the transform basis's matrix. Unlike the multiplication operator (`*`), this method ignores the origin.
    pub fn basis_xfrom(&self, v: &Vector2) -> Vector2 {
        Vector2::new(self.tdotx(v), self.tdoty(v))
    }

    /// Returns a copy of the `v` vector, transformed (multiplied) by the inverse transform basis's matrix (see [`Transform2D::inverse`]). This method ignores the `origin`.
    ///
    /// **Note:** This method assumes that this transform's basis is *orthonormal* (see [`Transform2D::orthonormalized`]). If the basis is not orthonormal, `transform.affine_inverse().basis_xform(vector)` should be used instead (see [`Transform2D::affine_inverse`]).
    pub fn basis_xform_inv(&self, v: &Vector2) -> Vector2 {
        Vector2::new(self.x.dot(v), self.y.dot(v))
    }

    /// Returns the [determinant](https://en.wikipedia.org/wiki/Determinant) of this transform basis's matrix. For advanced math, this number can be used to determine a few attributes:
    ///
    /// - If the determinant is exactly `0`, the basis is not invertible (see [`Transform2D::inverse`]).
    /// - If the determinant is a negative number, the basis represents a negative scale.
    ///
    /// **Note:** If the basis's scale is the same for every axis, its determinant is always that scale by the power of 2.
    pub fn determinant(&self) -> float!() {
        self.x.x * self.y.y - self.x.y * self.y.x
    }

    /// Returns this transform's translation. Equivalent to `origin`.
    pub fn get_origin(&self) -> Vector2 {
        self.origin
    }

    /// Returns this transform's rotation (in radians). This is equivalent to `x`'s angle (see [`Vector2::angle`]).
    pub fn get_rotation(&self) -> float!() {
        self.x.angle()
    }

    fn set_rotation(&mut self, rot: float!()) {
        let scale = self.get_scale();
        let cr = rot.cos();
        let sr = rot.sin();
        self.x.x = cr;
        self.x.y = sr;
        self.y.x = -sr;
        self.y.y = cr;
        self.set_scale(&scale);
    }

    /// Returns the length of both `x` and `y`, as a [`Vector2`]. If this transform's basis is not skewed, this value is the scaling factor. It is not affected by rotation.
    ///
    /// **Note:** If the value returned by [`Transform2D::determinant`] is negative, the scale is also negative.
    pub fn get_scale(&self) -> Vector2 {
        let det_sign = self.determinant().sign();
        Vector2::new(self.x.length(), det_sign * self.y.length())
    }

    fn set_scale(&mut self, scale: &Vector2) {
        self.x = self.x.normalized();
        self.y = self.y.normalized();
        self.x *= scale.x;
        self.y *= scale.y;
    }

    /// Returns this transform's skew (in radians).
    pub fn get_skew(&self) -> float!() {
        let det = self.determinant();
        self.x
            .normalized()
            .dot(&(det.sign() * self.y.normalized()))
            .acos()
            - float_consts::PI * 0.5
    }

    pub fn set_skew(&mut self, angle: float!()) {
        let det = self.determinant();
        self.y = det.sign() * self.x.rotated(float_consts::PI * 0.5 + angle).normalized() * self.y.length();
    }

    /// Returns the result of the linear interpolation between this transform and `xform` by the given `weight`.
    ///
    /// The `weight` should be between `0.0` and `1.0` (inclusive). Values outside this range are allowed and can be used to perform *extrapolation* instead.
    pub fn interpolate_with(&self, xform: &Transform2D, weight: float!()) -> Self {
        Transform2D::from((
            self.get_rotation().lerp(xform.get_rotation(), weight),
            self.get_scale().lerp(&xform.get_scale(), weight),
            self.get_skew().lerp(xform.get_skew(), weight),
            self.get_origin().lerp(&xform.get_origin(), weight),
        ))
    }

    fn invert(&mut self) {
        swap(&mut self.x.y, &mut self.y.x);
        self.origin = self.basis_xfrom(&-self.origin);
    }

    /// Returns the [inverted version of this transform](https://en.wikipedia.org/wiki/Invertible_matrix).
    ///
    /// **Note:** For this method to return correctly, the transform's basis needs to be *orthonormal* (see [`Transform2D::orthonormalized`]). That means, the basis should only represent a rotation. If it does not, use [`Transform2D::affine_inverse`] instead.
    pub fn inverse(&self) -> Self {
        let mut inv = *self;
        inv.invert();
        inv
    }

    /// Returns `true` if this transform's basis is conformal. A conformal basis is both *orthogonal* (the axes are perpendicular to each other) and *uniform* (the axes share the same length). This method can be especially useful during physics calculations.
    pub fn is_conformal(&self) -> bool {
        // Non-flipped case.
        if is_equal_approx(self.x.x, self.y.y) && is_equal_approx(self.x.y, -self.y.x) {
            true
        }
        // Flipped case.
        else if is_equal_approx(self.x.x, -self.y.y) && is_equal_approx(self.x.y, self.y.x) {
            true
        } else {
            false
        }
    }

    /// Returns `true` if this transform and `xform` are approximately equal, by running [`Vector2::is_equal_approx`] on each component.
    pub fn is_equal_approx(&self, xform: &Transform2D) -> bool {
        self.x.is_equal_approx(&xform.x)
            && self.y.is_equal_approx(&xform.y)
            && self.origin.is_equal_approx(&xform.origin)
    }

    /// Returns `true` if this transform is finite, by calling [`Vector2::is_finite`] on each component.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.origin.is_finite()
    }

    pub fn tdotx(&self, v: &Vector2) -> float!() {
        self.x.x * v.x + self.y.x * v.y
    }

    pub fn tdoty(&self, v: &Vector2) -> float!() {
        self.x.y * v.x + self.y.y * v.y
    }

    pub fn xform(&self, vec: &Vector2) -> Vector2 {
        Vector2::new(self.tdotx(vec), self.tdoty(vec)) + self.origin
    }

    /// Returns a copy of the transform rotated such that the rotated X-axis points towards the `target` position, in global space.
    pub fn looking_at(&self, target: &Vector2) -> Self {
        let mut return_trans = Self::from((self.get_rotation(), self.get_origin()));
        let target_position = self.affine_inverse().xform(target);
        return_trans.set_rotation(
            return_trans.get_rotation() + (target_position * self.get_scale()).angle(),
        );
        return_trans
    }

    fn orthonormalize(&mut self) {
        // Gram-Schmidt Process

        let mut x = self.x;
        let mut y = self.y;

        x = x.normalized();
        y = y - x * x.dot(&y);
        y = y.normalized();

        self.x = x;
        self.y = y;
    }

    /// Returns a copy of this transform with its basis orthonormalized. An orthonormal basis is both *orthogonal* (the axes are perpendicular to each other) and *normalized* (the axes have a length of `1`), which also means it can only represent rotation.
    pub fn orthonormalized(&self) -> Self {
        let mut ortho = *self;
        ortho.orthonormalize();
        ortho
    }

    /// Returns a copy of the transform rotated by the given `angle` (in radians).
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the left, i.e., `R * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn rotated(&self, angle: float!()) -> Self {
        // Equivalent to left multiplication
        Self::from((angle, Vector2::default())) * self
    }

    /// Returns a copy of the transform rotated by the given `angle` (in radians).
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the right, i.e., `X * R`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn rotated_local(&self, angle: float!()) -> Self {
        // Equivalent to right multiplication
        self * Self::from((angle, Vector2::default())) // Could be optimized, because origin transform can be skipped.
    }

    fn scale(&mut self, scale: &Vector2) {
        self.scale_basis(scale);
        self.origin *= scale;
    }

    fn scale_basis(&mut self, scale: &Vector2) {
        self.x.x *= scale.x;
        self.x.y *= scale.y;
        self.y.x *= scale.x;
        self.y.y *= scale.y;
    }

    /// Returns a copy of the transform scaled by the given `scale` factor.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the left, i.e., `S * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn scaled(&self, scale: &Vector2) -> Self {
        // Equivalent to left multiplication
        let mut copy = *self;
        copy.scale(scale);
        copy
    }

    /// Returns a copy of the transform scaled by the given `scale` factor.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the right, i.e., `X * S`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn scaled_local(&self, scale: &Vector2) -> Self {
        // Equivalent to right multiplication
        Self::new(self.x * scale.x, self.y * scale.y, self.origin)
    }

    /// Returns a copy of the transform translated by the given `offset`.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the left, i.e., `T * X`.
    ///
    /// This can be seen as transforming with respect to the global/parent frame.
    pub fn translated(&self, offset: &Vector2) -> Self {
        // Equivalent to left multiplication
        Self::new(self.x, self.y, self.origin + offset)
    }

    /// Returns a copy of the transform translated by the given `offset`.
    ///
    /// This method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the right, i.e., `X * T`.
    ///
    /// This can be seen as transforming with respect to the local frame.
    pub fn translated_local(&self, offset: &Vector2) -> Self {
        Self::new(self.x, self.y, self.origin + self.basis_xfrom(offset))
    }

    pub fn get(&self, index: usize) -> Vector2 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.origin,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, index: usize, value: Vector2) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.origin = value,
            _ => panic!("Invalid index"),
        }
    }
}

impl From<(float!(), &Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians) and position.
    fn from(value: (float!(), &Vector2)) -> Self {
        let cr = value.0.cos();
        let sr = value.0.sin();
        Self::new_from_floats(cr, sr, -sr, cr, value.1.x, value.1.y)
    }
}

impl From<(float!(), Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians) and position.
    fn from(value: (float!(), Vector2)) -> Self {
        Self::from((value.0, &value.1))
    }
}

impl From<(float!(), &Vector2, float!(), &Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians), scale, skew (in radians), and position.
    fn from(value: (float!(), &Vector2, float!(), &Vector2)) -> Self {
        let xx = value.0.cos() * value.1.x;
        let yy = (value.0 + value.2).cos() * value.1.y;
        let yx = -(value.0 + value.2).sin() * value.1.y;
        let xy = value.0.sin() * value.1.x;
        Self::new_from_floats(xx, xy, yx, yy, value.3.x, value.3.y)
    }
}

impl From<(float!(), Vector2, float!(), Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians), scale, skew (in radians), and position.
    fn from(value: (float!(), Vector2, float!(), Vector2)) -> Self {
        Self::from((value.0, &value.1, value.2, &value.3))
    }
}

impl From<(float!(), &Vector2, float!(), Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians), scale, skew (in radians), and position.
    fn from(value: (float!(), &Vector2, float!(), Vector2)) -> Self {
        Self::from((value.0, value.1, value.2, &value.3))
    }
}

impl From<(float!(), Vector2, float!(), &Vector2)> for Transform2D {
    /// Constructs a **Transform2D** from a given angle (in radians), scale, skew (in radians), and position.
    fn from(value: (float!(), Vector2, float!(), &Vector2)) -> Self {
        Self::from((value.0, &value.1, value.2, value.3))
    }
}

impl PartialEq for Transform2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.origin == other.origin
    }
}

impl Eq for Transform2D {}

impl_op_ex_commutative!(
    *|a: &Transform2D, b: &Vec<Transform2D>| -> Vec<Transform2D> {
        b.iter().map(|(&i)| i * a).collect()
    }
);

impl_op_ex!(*= |a: &mut Transform2D, b: &Transform2D| {
    a.origin = a.xform(&b.origin);
    let x0 = a.tdotx(&b.x);
    let x1 = a.tdoty(&b.x);
    let y0 = a.tdotx(&b.y);
    let y1 = a.tdoty(&b.y);

    a.x.x = x0;
    a.x.y = x1;
    a.y.x = y0;
    a.y.y = y1;
});

impl_op_ex!(*|a: &Transform2D, b: &Transform2D| -> Transform2D {
    let mut t = *a;
    t *= b;
    t
});

// TODO: impl Rect2 * Transform2D

impl_op_ex!(*= |a: &mut Transform2D, b: &Vector2| {
    a.x *= b;
    a.y *= b;
    a.origin *= b;
});

impl_op_ex_commutative!(*|a: &Transform2D, b: &Vector2| -> Transform2D {
    let mut t = *a;
    t *= b;
    t
});

impl_op_ex!(*= |a: &mut Transform2D, b: &float!()| {
    a.x *= b;
    a.y *= b;
    a.origin *= b;
});

impl_op_ex_commutative!(*|a: &Transform2D, b: &float!()| -> Transform2D {
    let mut t = *a;
    t *= b;
    t
});

impl_op_ex!(*= |a: &mut Transform2D, b: int!()| {
    a.x *= b;
    a.y *= b;
    a.origin *= b;
});

impl_op_ex_commutative!(*|a: &Transform2D, b: int!()| -> Transform2D {
    let mut t = *a;
    t *= b;
    t
});

impl_op_ex!(/= |a: &mut Transform2D, b: &float!()| {
    a.x /= b;
    a.y /= b;
    a.origin /= b;
});

impl_op_ex_commutative!(/ |a: &Transform2D, b: &float!()| -> Transform2D {
    let mut t = *a;
    t /= b;
    t
});

impl_op_ex!(/= |a: &mut Transform2D, b: int!()| {
    a.x /= b;
    a.y /= b;
    a.origin /= b;
});

impl_op_ex_commutative!(/ |a: &Transform2D, b: int!()| -> Transform2D {
    let mut t = *a;
    t /= b;
    t
});
