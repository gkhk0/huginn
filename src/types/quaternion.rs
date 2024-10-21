use crate::types::vectors::Vector3;
use crate::types::{Basis, EulerOrder};
use crate::utils::{
    cubic_interpolate, cubic_interpolate_in_time, float, int, is_equal_approx,
    is_equal_approx_with_tolerance, CMP_EPSILON, UNIT_EPSILON,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::ops::Neg;

/// A unit quaternion used for representing 3D rotations.
///
/// The **Quaternion** type is a 4D data structure that represents rotation in the form of a [Hamilton convention quaternion](https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation). Compared to the [`Basis`] type which can store both rotation and scale, quaternions can *only* store rotation.
///
/// A **Quaternion** is composed by 4 floating-point components: `w`, `x`, `y`, and `z`. These components are very compact in memory, and because of this some operations are more efficient and less likely to cause floating-point errors. Methods such as [`Quaternion::get_angle`], [`Quaternion::get_axis`], and [`Quaternion::slerp`] are faster than their Basis counterparts.
///
/// For a great introduction to quaternions, see [this video by 3Blue1Brown](https://www.youtube.com/watch?v=d4EgbgTm0Bg). You do not need to know the math behind quaternions, as Grimm provides several helper methods that handle it for you. These include [`Quaternion::slerp`] and [`Quaternion::spherical_cubic_interpolate`], as well as the `*` operator.
///
/// **Note:** Quaternions must be normalized before being used for rotation (see [`Quaternion::normalized`]).
///
/// **Note:** Similarly to [`Vector2`] and [`Vector3`], the components of a quaternion use 32-bit precision by default. If double precision is needed, use the feature flag `double-precision`.
#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    /// W component of the quaternion. This is the "real" part.
    ///
    /// **Note:** Quaternion components should usually not be manipulated directly.
    pub w: float!(),
    /// X component of the quaternion. This is the value along the "imaginary" `i` axis.
    ///
    /// **Note:** Quaternion components should usually not be manipulated directly.
    pub x: float!(),
    /// Y component of the quaternion. This is the value along the "imaginary" `j` axis.
    ///
    /// **Note:** Quaternion components should usually not be manipulated directly.
    pub y: float!(),
    /// Z component of the quaternion. This is the value along the "imaginary" `k` axis.
    ///
    /// **Note:** Quaternion components should usually not be manipulated directly.
    pub z: float!(),
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quaternion {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Constructs a **Quaternion** defined by the given values.
    ///
    /// **Note:** Only normalized quaternions represent rotation; if these values are not normalized, the new **Quaternion** will not be a valid rotation.
    pub const fn new(x: float!(), y: float!(), z: float!(), w: float!()) -> Self {
        Self { x, y, z, w }
    }

    /// Returns the angle between this quaternion and `to`. This is the magnitude of the angle you would need to rotate by to get from one to the other.
    ///
    /// **Note:** The magnitude of the floating-point error for this method is abnormally high, so methods such as `is_zero_approx` will not work reliably.
    pub fn angle_to(&self, to: &Self) -> float!() {
        let d = self.dot(to);
        // acos does clamping.
        (d * d * 2.0 - 1.0).acos()
    }

    /// Returns the dot product between this quaternion and `with`.
    ///
    /// This is equivalent to `(quat.x * with.x) + (quat.y * with.y) + (quat.z * with.z) + (quat.w * with.w)`.
    pub const fn dot(&self, with: &Self) -> float!() {
        self.x * with.x + self.y * with.y + self.z * with.z + self.w * with.w
    }

    /// Returns the exponential of this quaternion. The rotation axis of the result is the normalized rotation axis of this quaternion, the angle of the result is the length of the vector part of this quaternion.
    pub fn exp(&self) -> Self {
        let src = *self;
        let mut src_v = Vector3::new(src.x, src.y, src.z);
        let theta = src_v.length();
        src_v = src_v.normalized();
        if theta < CMP_EPSILON || !src_v.is_normalized() {
            Quaternion::new(0.0, 0.0, 0.0, 1.0)
        } else {
            Quaternion::from((&src_v, theta))
        }
    }

    /// Constructs a new **Quaternion** from the given [`Vector3`] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians. This method always uses the YXZ convention ([`EulerOrder::YXZ`]).
    pub fn from_euler(euler: &Vector3) -> Self {
        let half_a1 = euler.y / 2.0;
        let half_a2 = euler.x / 2.0;
        let half_a3 = euler.z / 2.0;

        // R = Y(a1).X(a2).Z(a3) convention for Euler angles.
        // Conversion to quaternion as listed in https://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf (page A-6)
        // a3 is the angle of the first rotation, following the notation in this reference.

        let cos_a1 = half_a1.cos();
        let sin_a1 = half_a1.sin();
        let cos_a2 = half_a2.cos();
        let sin_a2 = half_a2.sin();
        let cos_a3 = half_a3.cos();
        let sin_a3 = half_a3.sin();

        Quaternion::new(
            sin_a1 * cos_a2 * sin_a3 + cos_a1 * sin_a2 * cos_a3,
            sin_a1 * cos_a2 * cos_a3 - cos_a1 * sin_a2 * sin_a3,
            -sin_a1 * sin_a2 * cos_a3 + cos_a1 * cos_a2 * sin_a3,
            sin_a1 * sin_a2 * sin_a3 + cos_a1 * cos_a2 * cos_a3,
        )
    }

    /// Returns the angle of the rotation represented by this quaternion.
    ///
    /// **Note:** The quaternion must be normalized.
    pub fn get_angle(&self) -> float!() {
        2.0 * self.w.acos()
    }

    /// Returns the rotation axis of the rotation represented by this quaternion.
    pub fn get_axis(&self) -> Vector3 {
        if self.w.abs() > 1.0 - CMP_EPSILON {
            Vector3::new(self.x, self.y, self.z)
        } else {
            let r = 1.0 / (1.0 - self.w * self.w).sqrt();
            Vector3::new(self.x * r, self.y * r, self.z * r)
        }
    }

    /// Returns this quaternion's rotation as a [`Vector3`] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians.
    ///
    /// The order of each consecutive rotation can be changed with `order` (see [`EulerOrder`]). By default, the YXZ convention is used ([`EulerOrder::YXZ`]): Z (roll) is calculated first, then X (pitch), and lastly Y (yaw). When using the opposite method [`Quaternion::from_euler`], this order is reversed.
    pub fn get_euler(&self, order: Option<EulerOrder>) -> Vector3 {
        let order = if let Some(order) = order {
            order
        } else {
            EulerOrder::YXZ
        };

        Basis::from(self).get_euler(Some(order))
    }

    /// Returns the inverse version of this quaternion, inverting the sign of every component except `w`.
    pub const fn inverse(&self) -> Self {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns `true` if this quaternion and `to` are approximately equal, by running `is_equal_approx` on each component.
    pub fn is_equal_approx(&self, to: &Self) -> bool {
        is_equal_approx(self.x, to.x)
            && is_equal_approx(self.y, to.y)
            && is_equal_approx(self.z, to.z)
            && is_equal_approx(self.w, to.w)
    }

    /// Returns `true` if this quaternion is finite, by calling `is_finite` on each component.
    pub const fn is_finite(&self) -> bool {
        self.w.is_finite() && self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns `true` if this quaternion is normalized. See also [`Quaternion::normalized`].
    pub fn is_normalized(&self) -> bool {
        is_equal_approx_with_tolerance(self.length_squared(), 1.0, UNIT_EPSILON)
        // use less epsilon
    }

    /// Returns this quaternion's length, also called magnitude.
    pub fn length(&self) -> float!() {
        self.length_squared().sqrt()
    }

    /// Returns this quaternion's length, squared.
    ///
    /// **Note:** This method is faster than [`Quaternion::length`], so prefer it if you only need to compare quaternion lengths.
    pub const fn length_squared(&self) -> float!() {
        self.dot(self)
    }

    /// Returns the logarithm of this quaternion. Multiplies this quaternion's rotation axis by its rotation angle, and stores the result in the returned quaternion's vector part (`x`, `y`, and `z`). The returned quaternion's real part (`w`) is always `0.0`.
    pub fn log(&self) -> Self {
        let src = *self;
        let src_v = src.get_axis() * src.get_angle();
        Quaternion::new(src_v.x, src_v.y, src_v.z, 0.0)
    }

    /// Returns a copy of this quaternion, normalized so that its length is `1.0`. See also [`Quaternion::is_normalized`].
    pub fn normalized(&self) -> Self {
        self / self.length()
    }

    /// Performs a spherical-linear interpolation with the `to` quaternion, given a `weight` and returns the result. Both this quaternion and `to` must be normalized.
    pub fn slerp(&self, to: &Self, weight: float!()) -> Self {
        // calc cosine
        let mut cosom = self.dot(to);

        // adjust signs (if necessary)
        let to1 = if cosom < 0.0 {
            cosom = -cosom;
            -to
        } else {
            *to
        };

        // calculate coefficients

        let (scale0, scale1) = if (1.0 - cosom) > CMP_EPSILON {
            // standard case (slerp)
            let omega = cosom.acos();
            let sinom = omega.sin();
            (
                ((1.0 - weight) * omega).sin() / sinom,
                (weight * omega).sin() / sinom,
            )
        } else {
            // "from" and "to" quaternions are very close
            //  ... so we can do a linear interpolation
            (1.0 - weight, weight)
        };
        // calculate final values
        Self::new(
            scale0 * self.x + scale1 * to1.x,
            scale0 * self.y + scale1 * to1.y,
            scale0 * self.z + scale1 * to1.z,
            scale0 * self.w + scale1 * to1.w,
        )
    }

    /// Performs a spherical-linear interpolation with the `to` quaternion, given a `weight` and returns the result. Unlike [`Quaternion::slerp`], this method does not check if the rotation path is smaller than 90 degrees. Both this quaternion and `to` must be normalized.
    pub fn slerp_ni(&self, to: &Self, weight: float!()) -> Self {
        let from = *self;

        let dot = from.dot(to);

        if dot.abs() > 1.0 - CMP_EPSILON / 10.0 {
            from
        } else {
            let theta = dot.acos();
            let sin_t = 1.0 / theta.sin();
            let new_factor = (weight * theta).sin() * sin_t;
            let inv_factor = ((1.0 - weight) * theta).sin() * sin_t;

            Self::new(
                inv_factor * from.x + new_factor * to.x,
                inv_factor * from.y + new_factor * to.y,
                inv_factor * from.z + new_factor * to.z,
                inv_factor * from.w + new_factor * to.w,
            )
        }
    }

    /// Performs a spherical cubic interpolation between quaternions `pre_a`, this vector, `b`, and `post_b`, by the given amount `weight`.
    pub fn spherical_cubic_interpolate(
        &self,
        b: &Self,
        pre_a: &Self,
        post_b: &Self,
        weight: float!(),
    ) -> Self {
        let mut from_q = *self;
        let mut pre_q = *pre_a;
        let mut to_q = *b;
        let mut post_q = *post_b;

        // Align flip phases.
        from_q = Basis::from(&from_q).get_rotation_quaternion();
        pre_q = Basis::from(&pre_q).get_rotation_quaternion();
        to_q = Basis::from(&to_q).get_rotation_quaternion();
        post_q = Basis::from(&post_q).get_rotation_quaternion();

        // Flip quaternions to the shortest path if necessary.
        let flip1 = from_q.dot(&pre_q).is_sign_negative();
        pre_q = if flip1 { -pre_q } else { pre_q };
        let flip2 = from_q.dot(&to_q).is_sign_negative();
        to_q = if flip2 { -to_q } else { to_q };
        let flip3 = if flip2 {
            to_q.dot(&post_q) <= 0.0
        } else {
            to_q.dot(&post_q).is_sign_negative()
        };
        post_q = if flip3 { -post_q } else { post_q };

        // Calc by Exp map in from_q space.
        let mut ln_from = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        let mut ln_to = (from_q.inverse() * to_q).log();
        let mut ln_pre = (from_q.inverse() * pre_q).log();
        let mut ln_post = (from_q.inverse() * post_q).log();
        let mut ln = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln.x = cubic_interpolate(ln_from.x, ln_to.x, ln_pre.x, ln_post.x, weight);
        ln.y = cubic_interpolate(ln_from.y, ln_to.y, ln_pre.y, ln_post.y, weight);
        ln.z = cubic_interpolate(ln_from.z, ln_to.z, ln_pre.z, ln_post.z, weight);
        let q1 = from_q * ln.exp();

        // Calc by Exp map in to_q space.
        ln_from = (to_q.inverse() * from_q).log();
        ln_to = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln_pre = (to_q.inverse() * pre_q).log();
        ln_post = (to_q.inverse() * post_q).log();
        ln = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln.x = cubic_interpolate(ln_from.x, ln_to.x, ln_pre.x, ln_post.x, weight);
        ln.y = cubic_interpolate(ln_from.y, ln_to.y, ln_pre.y, ln_post.y, weight);
        ln.z = cubic_interpolate(ln_from.z, ln_to.z, ln_pre.z, ln_post.z, weight);
        let q2 = to_q * ln.exp();

        // To cancel error made by Exp map ambiguity, do blending.
        q1.slerp(&q2, weight)
    }

    /// Performs a spherical cubic interpolation between quaternions `pre_a`, this vector, `b`, and `post_b`, by the given amount `weight`.
    ///
    /// It can perform smoother interpolation than [`Quaternion::spherical_cubic_interpolate`] by the time values.
    pub fn spherical_cubic_interpolate_in_time(
        &self,
        b: &Self,
        pre_a: &Self,
        post_b: &Self,
        weight: float!(),
        b_t: float!(),
        pre_a_t: float!(),
        post_b_t: float!(),
    ) -> Self {
        let mut from_q = *self;
        let mut pre_q = *pre_a;
        let mut to_q = *b;
        let mut post_q = *post_b;

        // Align flip phases.
        from_q = Basis::from(&from_q).get_rotation_quaternion();
        pre_q = Basis::from(&pre_q).get_rotation_quaternion();
        to_q = Basis::from(&to_q).get_rotation_quaternion();
        post_q = Basis::from(&post_q).get_rotation_quaternion();

        // Flip quaternions to the shortest path if necessary.
        let flip1 = from_q.dot(&pre_q).is_sign_negative();
        pre_q = if flip1 { -pre_q } else { pre_q };
        let flip2 = from_q.dot(&to_q).is_sign_negative();
        to_q = if flip2 { -to_q } else { to_q };
        let flip3 = if flip2 {
            to_q.dot(&post_q) <= 0.0
        } else {
            to_q.dot(&post_q).is_sign_negative()
        };
        post_q = if flip3 { -post_q } else { post_q };

        // Calc by Exp map in from_q space.
        let mut ln_from = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        let mut ln_to = (from_q.inverse() * to_q).log();
        let mut ln_pre = (from_q.inverse() * pre_q).log();
        let mut ln_post = (from_q.inverse() * post_q).log();
        let mut ln = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln.x = cubic_interpolate_in_time(
            ln_from.x, ln_to.x, ln_pre.x, ln_post.x, weight, b_t, pre_a_t, post_b_t,
        );
        ln.y = cubic_interpolate_in_time(
            ln_from.y, ln_to.y, ln_pre.y, ln_post.y, weight, b_t, pre_a_t, post_b_t,
        );
        ln.z = cubic_interpolate_in_time(
            ln_from.z, ln_to.z, ln_pre.z, ln_post.z, weight, b_t, pre_a_t, post_b_t,
        );
        let q1 = from_q * ln.exp();

        // Calc by Exp map in to_q space.
        ln_from = (to_q.inverse() * from_q).log();
        ln_to = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln_pre = (to_q.inverse() * pre_q).log();
        ln_post = (to_q.inverse() * post_q).log();
        ln = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        ln.x = cubic_interpolate_in_time(
            ln_from.x, ln_to.x, ln_pre.x, ln_post.x, weight, b_t, pre_a_t, post_b_t,
        );
        ln.y = cubic_interpolate_in_time(
            ln_from.y, ln_to.y, ln_pre.y, ln_post.y, weight, b_t, pre_a_t, post_b_t,
        );
        ln.z = cubic_interpolate_in_time(
            ln_from.z, ln_to.z, ln_pre.z, ln_post.z, weight, b_t, pre_a_t, post_b_t,
        );
        let q2 = to_q * ln.exp();

        // To cancel error made by Exp map ambiguity, do blending.
        q1.slerp(&q2, weight)
    }

    pub fn xform(&self, v: &Vector3) -> Vector3 {
        let u = Vector3::new(self.x, self.y, self.z);
        let uv = u.cross(v);
        v + ((uv * self.w) + u.cross(&uv)) * 2.0
    }
}

impl From<(&Vector3, &Vector3)> for Quaternion {
    /// Constructs a **Quaternion** representing the shortest arc between `arc_from` and `arc_to`. These can be imagined as two points intersecting a sphere's surface, with a radius of `1.0`.
    fn from(v: (&Vector3, &Vector3)) -> Self {
        let c = v.0.cross(v.1);
        let d = v.0.dot(v.1);

        if d < -1.0 + CMP_EPSILON {
            Quaternion::new(0.0, 1.0, 0.0, 0.0)
        } else {
            let s = ((1.0 + d) * 2.0).sqrt();
            let rs = 1.0 / s;
            Quaternion::new(c.x * rs, c.y * rs, c.z * rs, s * 0.5)
        }
    }
}

impl From<(&Vector3, float!())> for Quaternion {
    /// Constructs a **Quaternion** representing rotation around the `axis` by the given `angle`, in radians. The axis must be a normalized vector.
    fn from(v: (&Vector3, float!())) -> Self {
        let d = v.0.length();
        if d == 0.0 {
            Quaternion::new(0.0, 0.0, 0.0, 0.0)
        } else {
            let sin_angle = (v.1 * 0.5).sin();
            let cos_angle = (v.1 * 0.5).cos();
            let s = sin_angle / d;
            Quaternion::new(v.0.x * s, v.0.y * s, v.0.z * s, cos_angle)
        }
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Eq for Quaternion {}

impl_op_ex!(*|lhs: &Quaternion, rhs: &Quaternion| -> Quaternion {
    Quaternion::new(
        lhs.w * rhs.x + lhs.x * rhs.w + lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.w * rhs.y + lhs.y * rhs.w + lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.w * rhs.z + lhs.z * rhs.w + lhs.x * rhs.y - lhs.y * rhs.x,
        lhs.w * rhs.w - lhs.x * rhs.x - lhs.y * rhs.y - lhs.z * rhs.z,
    )
});
impl_op_ex!(*= |lhs: &mut Quaternion, rhs: &Quaternion| {
    lhs.x = lhs.w * rhs.x + lhs.x * rhs.w + lhs.y * rhs.z - lhs.z * rhs.y;
    lhs.y = lhs.w * rhs.y + lhs.y * rhs.w + lhs.z * rhs.x - lhs.x * rhs.z;
    lhs.z = lhs.w * rhs.z + lhs.z * rhs.w + lhs.x * rhs.y - lhs.y * rhs.x;
    lhs.w = lhs.w * rhs.w - lhs.x * rhs.x - lhs.y * rhs.y - lhs.z * rhs.z;
});

impl_op_ex_commutative!(*|lhs: &Quaternion, rhs: &float!()| -> Quaternion {
    Quaternion::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs, lhs.w * rhs)
});
impl_op_ex!(*= |lhs: &mut Quaternion, rhs: &float!()| {
    lhs.x = lhs.x * rhs;
    lhs.y = lhs.y * rhs;
    lhs.z = lhs.z * rhs;
    lhs.w = lhs.w * rhs;
});
impl_op_ex_commutative!(*|lhs: &Quaternion, rhs: int!()| -> Quaternion { lhs * rhs as float!() });
impl_op_ex!(*= |lhs: &mut Quaternion, rhs: int!()| {
    lhs.x = lhs.x * rhs as float!();
    lhs.y = lhs.y * rhs as float!();
    lhs.z = lhs.z * rhs as float!();
    lhs.w = lhs.w * rhs as float!();
});

impl_op_ex!(+ |lhs: &Quaternion, rhs: &Quaternion| -> Quaternion {
    Quaternion::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w)
});
impl_op_ex!(+= |lhs: &mut Quaternion, rhs: &Quaternion| {
    lhs.x = lhs.x + rhs.x;
    lhs.y = lhs.y + rhs.y;
    lhs.z = lhs.z + rhs.z;
    lhs.w = lhs.w + rhs.w;
});
impl_op_ex!(-|lhs: &Quaternion, rhs: &Quaternion| -> Quaternion {
    Quaternion::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w)
});
impl_op_ex!(-= |lhs: &mut Quaternion, rhs: &Quaternion| {
    lhs.x = lhs.x - rhs.x;
    lhs.y = lhs.y - rhs.y;
    lhs.z = lhs.z - rhs.z;
    lhs.w = lhs.w - rhs.w;
});
impl_op_ex!(/ |lhs: &Quaternion, rhs: &float!()| -> Quaternion {
    Quaternion::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs, lhs.w / rhs)
});
impl_op_ex!(/= |lhs: &mut Quaternion, rhs: &float!()| {
    lhs.x = lhs.x / rhs;
    lhs.y = lhs.y / rhs;
    lhs.z = lhs.z / rhs;
    lhs.w = lhs.w / rhs;
});
impl_op_ex!(/ |lhs: &Quaternion, rhs: int!()| -> Quaternion {
    lhs / rhs as float!()
});
impl_op_ex!(/= |lhs: &mut Quaternion, rhs: int!()| {
    lhs.x = lhs.x / rhs as float!();
    lhs.y = lhs.y / rhs as float!();
    lhs.z = lhs.z / rhs as float!();
    lhs.w = lhs.w / rhs as float!();
});

impl Neg for Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}
impl Neg for &Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}
