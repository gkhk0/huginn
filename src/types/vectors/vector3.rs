use crate::types::vectors::{Vector2, Vector3i, AXIS};
use crate::types::Basis;
use crate::utils::{
    bezier_derivative, bezier_interpolate, cubic_interpolate, cubic_interpolate_in_time, float,
    int, is_equal_approx, is_equal_approx_with_tolerance, is_zero_approx, posmod_f, snapped,
    FloatExt, CMP_EPSILON, UNIT_EPSILON,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Neg, Not};

/// A 3D vector using floating-point coordinates.
///
/// A 3-element structure that can be used to represent 3D coordinates or any other triplet of numeric values.
///
/// It uses floating-point coordinates. By default, these floating-point values use 32-bit precision. If double precision is needed, use the feature flag `double-precision`.
///
/// See [`Vector3i`] for its integer counterpart.
///
/// **Note:** In a boolean context, a Vector3 will evaluate to `false` if it's equal to `Vector3::new(0.0, 0.0, 0.0)`. Otherwise, a Vector3 will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector3 {
    /// The vector's X component. Also, accessible by using the index position `v.get(0)`.
    pub x: float!(),
    /// The vector's Y component. Also, accessible by using the index position `v.get(1)`.
    pub y: float!(),
    /// The vector's Z component. Also, accessible by using the index position `v.get(2)`.
    pub z: float!(),
}

impl Vector3 {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    /// Infinity vector, a vector with all components set to [`f32::INFINITY`].
    pub const INF: Self = Self::new(
        <float!()>::INFINITY,
        <float!()>::INFINITY,
        <float!()>::INFINITY,
    );

    /// Left unit vector. Represents the local direction of left, and the global direction of west.
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);

    /// Right unit vector. Represents the local direction of right, and the global direction of east.
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);

    /// Up unit vector.
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);

    /// Down unit vector.
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);

    /// Forward unit vector. Represents the local direction of forward, and the global direction of north. Keep in mind that the forward direction for lights, cameras, etc. is different from 3D assets like characters, which face towards the camera by convention. Use [`Vector3::MODEL_FRONT`] and similar constants when working in 3D asset space.
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0);

    /// Back unit vector. Represents the local direction of back, and the global direction of south.
    pub const BACK: Self = Self::new(0.0, 0.0, 1.0);

    /// Unit vector pointing towards the left side of imported 3D assets.
    pub const MODEL_LEFT: Self = Self::new(1.0, 0.0, 0.0);

    /// Unit vector pointing towards the right side of imported 3D assets.
    pub const MODEL_RIGHT: Self = Self::new(1.0, 0.0, 1.0);

    /// Unit vector pointing towards the top side (up) of imported 3D assets.
    pub const MODEL_TOP: Self = Self::new(0.0, 1.0, 0.0);

    /// Unit vector pointing towards the bottom side (down) of imported 3D assets.
    pub const MODEL_BOTTOM: Self = Self::new(0.0, -1.0, 0.0);

    /// Unit vector pointing towards the front side (facing forward) of imported 3D assets.
    pub const MODEL_FRONT: Self = Self::new(0.0, 0.0, 1.0);

    /// Unit vector pointing towards the rear side (back) of imported 3D assets.
    pub const MODEL_REAR: Self = Self::new(0.0, 0.0, -1.0);

    /// Returns a **Vector3** with the given components.
    pub const fn new(x: float!(), y: float!(), z: float!()) -> Self {
        Self { x, y, z }
    }

    /// Access vector components using their index. `v.get(0)` is equivalent to `v.x`, `v.get(1)` is equivalent to `v.y`, and `v.get(2)` is equivalent to `v.z`.
    pub const fn get(&self, index: usize) -> float!() {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, index: usize, value: float!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> float!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            AXIS::Z => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: float!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            AXIS::Z => self.z = value,
            _ => panic!("Invalid axis"),
        }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Returns the unsigned minimum angle to the given vector, in radians.
    pub fn angle_to(&self, to: &Self) -> float!() {
        self.cross(to).length().atan2(self.dot(to))
    }

    /// Returns the derivative at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points.
    pub fn bezier_derivative(
        &self,
        control_1: &Self,
        control_2: &Self,
        end: &Self,
        t: float!(),
    ) -> Self {
        Self::new(
            bezier_derivative(self.x, control_1.x, control_2.x, end.x, t),
            bezier_derivative(self.y, control_1.y, control_2.y, end.y, t),
            bezier_derivative(self.z, control_1.z, control_2.z, end.z, t),
        )
    }

    /// Returns the point at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points.
    pub fn bezier_interpolate(
        &self,
        control_1: &Self,
        control_2: &Self,
        end: &Self,
        t: float!(),
    ) -> Self {
        Self::new(
            bezier_interpolate(self.x, control_1.x, control_2.x, end.x, t),
            bezier_interpolate(self.y, control_1.y, control_2.y, end.y, t),
            bezier_interpolate(self.z, control_1.z, control_2.z, end.z, t),
        )
    }

    /// Returns the vector "bounced off" from a plane defined by the given normal `n`.
    ///
    /// **Note:** [`Vector3::bounce`] performs the operation that most engines and frameworks call `reflect()`.
    pub fn bounce(&self, n: &Self) -> Self {
        -self.reflect(n)
    }

    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    /// Returns a new vector with all components clamped between the components of `min` and `max`, by running `clamp` on each component.
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    /// Returns a new vector with all components clamped between `min` and `max`, by running `clamp` on each component.
    pub fn clamp_f(&self, min: float!(), max: float!()) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    /// Returns the cross product of this vector and `with`.
    ///
    /// This returns a vector perpendicular to both this and `with`, which would be the normal vector of the plane defined by the two vectors. As there are two such vectors, in opposite directions, this method returns the vector defined by a right-handed coordinate system. If the two vectors are parallel this returns an empty vector, making it useful for testing if two vectors are parallel.
    pub fn cross(&self, with: &Self) -> Self {
        Self::new(
            self.y * with.z - self.z * with.y,
            self.z * with.x - self.x * with.z,
            self.x * with.y - self.y * with.x,
        )
    }

    /// Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    pub fn cubic_interpolate(
        &self,
        b: &Self,
        pre_a: &Self,
        post_b: &Self,
        weight: float!(),
    ) -> Self {
        Self::new(
            cubic_interpolate(self.x, b.x, pre_a.x, post_b.x, weight),
            cubic_interpolate(self.y, b.y, pre_a.y, post_b.y, weight),
            cubic_interpolate(self.z, b.z, pre_a.z, post_b.z, weight),
        )
    }

    /// Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    ///
    /// It can perform smoother interpolation than [`Vector3::cubic_interpolate`] by the time values.
    pub fn cubic_interpolate_in_time(
        &self,
        b: &Self,
        pre_a: &Self,
        post_b: &Self,
        weight: float!(),
        b_t: float!(),
        pre_a_t: float!(),
        post_b_t: float!(),
    ) -> Self {
        Self::new(
            cubic_interpolate_in_time(
                self.x, b.x, pre_a.x, post_b.x, weight, b_t, pre_a_t, post_b_t,
            ),
            cubic_interpolate_in_time(
                self.y, b.y, pre_a.y, post_b.y, weight, b_t, pre_a_t, post_b_t,
            ),
            cubic_interpolate_in_time(
                self.z, b.z, pre_a.z, post_b.z, weight, b_t, pre_a_t, post_b_t,
            ),
        )
    }

    /// Returns the normalized vector pointing from this vector to `to`. This is equivalent to using `(b - a).normalized()`.
    pub fn direction_to(&self, to: &Self) -> Self {
        (to - self).normalized()
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector3::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: &Self) -> float!() {
        (to - self).length_squared()
    }

    /// Returns the distance between this vector and `to`.
    pub fn distance_to(&self, to: &Self) -> float!() {
        (to - self).length()
    }

    /// Returns the dot product of this vector and `with`. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.
    ///
    /// The dot product will be `0` for a right angle (90-degrees), greater than 0 for angles narrower than 90-degrees and lower than 0 for angles wider than 90-degrees.
    ///
    /// When using unit (normalized) vectors, the result will always be between `-1.0` (180-degree angle) when the vectors are facing opposite directions, and `1.0` (0-degree angle) when the vectors are aligned.
    ///
    /// **Note:** a.dot(b) *is* equivalent to b.dot(a).
    pub fn dot(&self, with: &Self) -> float!() {
        self.x * with.x + self.y * with.y + self.z * with.z
    }

    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    /// Returns the inverse of the vector. This is the same as `Vector3::new(1.0 / v.x, 1.0 / v.y, 1.0 / v.z)`.
    pub fn inverse(&self) -> Self {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    /// Returns `true` if this vector and `to` are approximately equal, by running [`is_equal_approx`] on each component.
    pub fn is_equal_approx(&self, to: &Self) -> bool {
        is_equal_approx(self.x, to.x)
            && is_equal_approx(self.y, to.y)
            && is_equal_approx(self.z, to.z)
    }

    /// Returns `true` if this vector is finite, by calling `is_finite` on each component.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns `true` if the vector is normalized, i.e. its length is approximately equal to 1.
    pub fn is_normalized(&self) -> bool {
        is_equal_approx_with_tolerance(self.length_squared(), 1.0, UNIT_EPSILON)
    }

    /// Returns `true` if this vector's values are approximately zero, by running [`is_zero_approx`] on each component.
    ///
    /// This method is faster than using [`Vector3::is_equal_approx`] with one value as a zero vector.
    pub fn is_zero_approx(&self) -> bool {
        is_zero_approx(self.x) && is_zero_approx(self.y) && is_zero_approx(self.z)
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> float!() {
        self.length_squared().sqrt()
    }

    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector3::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn length_squared(&self) -> float!() {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        x2 + y2 + z2
    }

    /// Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    pub fn lerp(&self, to: &Self, weight: float!()) -> Self {
        Self::new(
            self.x.lerp(to.x, weight),
            self.y.lerp(to.y, weight),
            self.z.lerp(to.z, weight),
        )
    }

    /// Returns the vector with a maximum length by limiting its length to `length`.
    pub fn limit_length(&self, length: float!()) -> Self {
        let l = self.length();
        let mut v = *self;
        if l > 0.0 && length < l {
            v /= l;
            v *= length;
        }
        v
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector3::new(x.max(with.x), y.max(with.y), z.max(with.z))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(self.x.max(with.x), self.y.max(with.y), self.z.max(with.z))
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub fn max_axis_index(&self) -> AXIS {
        if self.x < self.y {
            if self.y < self.z {
                AXIS::Z
            } else {
                AXIS::Y
            }
        } else if self.x < self.z {
            AXIS::Z
        } else {
            AXIS::X
        }
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector3::new(x.max(with), y.max(with), z.max(with))`.
    pub fn max_f(&self, with: float!()) -> Self {
        Self::new(self.x.max(with), self.y.max(with), self.z.max(with))
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector3::new(x.min(with.x), y.min(with.y), z.min(with.z))`.
    pub fn min(&self, with: &Self) -> Self {
        Self::new(self.x.min(with.x), self.y.min(with.y), self.z.min(with.z))
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::Z`].
    pub fn min_axis_index(&self) -> AXIS {
        if self.x < self.y {
            if self.x < self.z {
                AXIS::X
            } else {
                AXIS::Z
            }
        } else if self.y < self.z {
            AXIS::Y
        } else {
            AXIS::Z
        }
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector3::new(x.min(with), y.min(with), z.min(with))`.
    pub fn min_f(&self, with: float!()) -> Self {
        Self::new(self.x.min(with), self.y.min(with), self.z.min(with))
    }

    /// Returns a new vector moved toward `to` by the fixed `delta` amount. Will not go past the final value.
    pub fn move_toward(&self, to: &Self, delta: float!()) -> Self {
        let vd = to - self;
        let len = vd.length();
        if len <= delta || len < CMP_EPSILON {
            *to
        } else {
            self + vd / len * delta
        }
    }

    fn normalize(&mut self) {
        let length_sq = self.length_squared();
        if length_sq == 0.0 {
            self.x = 0.0;
            self.z = 0.0;
            self.y = 0.0;
        } else {
            let length = length_sq.sqrt();
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }

    /// Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `(0, 0, 0)` if `v.length() == 0`. See also [`Vector3::is_normalized`].
    ///
    /// **Note:** This function may return incorrect values if the input vector length is near zero.
    pub fn normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    /// Returns the **Vector3** from an octahedral-compressed form created using [`Vector3::octahedron_encode`] (stored as a [`Vector2`]).
    pub fn octahedron_decode(uv: &Vector2) -> Self {
        let f = Vector2::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
        let mut n = Self::new(f.x, f.y, 1.0 - f.x.abs() - f.y.abs());
        let t = (-n.z).clamp(0.0, 1.0);
        n.x += if n.x >= 0.0 { -t } else { t };
        n.y += if n.y >= 0.0 { -t } else { t };
        n.normalized()
    }

    /// Returns the octahedral-encoded (oct32) form of this **Vector3** as a [`Vector2`]. Since a [`Vector2`] occupies 1/3 less memory compared to **Vector3**, this form of compression can be used to pass greater amounts of `normalized` **Vector3**s without increasing storage or memory requirements. See also [`Vector3::octahedron_decode`].
    ///
    /// **Note:** [`Vector3::octahedron_encode`] can only be used for `normalized` vectors. `octahedron_encode` does *not* check whether this **Vector3** is normalized, and will return a value that does not decompress to the original value if the **Vector3** is not normalized.
    ///
    /// **Note:** Octahedral compression is lossy, although visual differences are rarely perceptible in real world scenarios.
    pub fn octahedron_encode(&self) -> Vector2 {
        let mut n = *self;
        n /= n.x.abs() + n.y.abs() + n.z.abs();
        let mut o = if n.z >= 0.0 {
            Vector2::new(n.x, n.y)
        } else {
            Vector2::new(
                (1.0 - n.y.abs()) * (if n.x >= 0.0 { 1.0 } else { -1.0 }),
                (1.0 - n.x.abs()) * (if n.y >= 0.0 { 1.0 } else { -1.0 }),
            )
        };
        o.x = o.x * 0.5 + 0.5;
        o.y = o.y * 0.5 + 0.5;
        o
    }

    /// Returns the outer product with `with`.
    pub fn outer(&self, with: &Self) -> Basis {
        Basis::new(
            Self::new(self.x * with.x, self.x * with.y, self.x * with.z),
            Self::new(self.y * with.x, self.y * with.y, self.y * with.z),
            Self::new(self.z * with.x, self.z * with.y, self.z * with.z),
        )
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module`.
    pub fn posmod(&self, module: float!()) -> Self {
        Self::new(
            posmod_f(self.x, module),
            posmod_f(self.y, module),
            posmod_f(self.z, module),
        )
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module_v`'s components.
    pub fn posmodv(&self, module_v: &Self) -> Self {
        Self::new(
            posmod_f(self.x, module_v.x),
            posmod_f(self.y, module_v.y),
            posmod_f(self.z, module_v.z),
        )
    }

    /// Returns a new vector resulting from projecting this vector onto the given vector `b`. The resulting new vector is parallel to `b`. See also [`Vector3::slide`].
    ///
    /// **Note:** If the vector `b` is a zero vector, the components of the resulting new vector will be `NaN`.
    pub fn project(&self, b: &Self) -> Self {
        b * (self.dot(b) / b.length_squared())
    }

    /// Returns the result of reflecting the vector through a plane defined by the given normal vector `n`.
    ///
    /// Note: [`Vector3::reflect`] differs from what other engines and frameworks call `reflect()`. In other engines, `reflect()` returns the result of the vector reflected by the given plane. The reflection thus passes through the given normal. While in Grimm the reflection passes through the plane and can be thought of as bouncing off the normal. See also [`Vector3::bounce`] which does what most engines call `reflect()`.
    pub fn reflect(&self, n: &Self) -> Self {
        2.0 * n * self.dot(n) - self
    }

    fn rotate(&mut self, axis: &Self, angle: float!()) {
        *self = Basis::from((axis, angle)).xform(self);
    }

    /// Returns the result of rotating this vector around a given axis by `angle` (in radians). The axis must be a normalized vector. See also [`f32::to_radians`].
    pub fn rotated(&self, axis: &Self, angle: float!()) -> Self {
        let mut r = *self;
        r.rotate(axis, angle);
        r
    }

    /// Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    /// Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling `sign` on each component.
    pub fn sign(&self) -> Self {
        Self::new(self.x.sign(), self.y.sign(), self.z.sign())
    }

    /// Returns the signed angle to the given vector, in radians. The sign of the angle is positive in a counter-clockwise direction and negative in a clockwise direction when viewed from the side specified by the `axis`.
    pub fn signed_angle_to(&self, to: &Self, axis: &Self) -> float!() {
        let cross_to = self.cross(to);
        let unsigned_angle = cross_to.length().atan2(self.dot(to));
        let sign = cross_to.dot(axis);
        if sign < 0.0 {
            -unsigned_angle
        } else {
            unsigned_angle
        }
    }

    /// Returns the result of spherical linear interpolation between this vector and `to`, by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    ///
    /// This method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like [`Vector3::lerp`].
    pub fn slerp(&self, to: &Self, weight: float!()) -> Self {
        // This method seems more complicated than it really is, since we write out
        // the internals of some methods for efficiency (mainly, checking length).
        let start_length_sq = self.length_squared();
        let end_length_sq = to.length_squared();
        if start_length_sq == 0.0 || end_length_sq == 0.0 {
            // Zero length vectors have no angle, so the best we can do is either lerp or throw an error.
            return self.lerp(to, weight);
        }
        let mut axis = self.cross(to);
        let axis_length_sq = axis.length_squared();
        if axis_length_sq == 0.0 {
            // Colinear vectors have no rotation axis or angle between them, so the best we can do is lerp.
            return self.lerp(to, weight);
        }
        axis /= axis_length_sq.sqrt();
        let start_length = start_length_sq.sqrt();
        let result_length = start_length.lerp(end_length_sq.sqrt(), weight);
        let angle = self.angle_to(to);
        self.rotated(&axis, angle * weight) * (result_length / start_length)
    }

    /// Returns a new vector resulting from sliding this vector along a plane with normal `n`. The resulting new vector is perpendicular to `n`, and is equivalent to this vector minus its projection on `n`. See also [`Vector3::project`].
    ///
    /// **Note:** The vector `n` must be normalized. See also [`Vector3::normalized`].
    pub fn slide(&self, n: &Self) -> Self {
        self - n * self.dot(n)
    }

    fn snap(&mut self, step: &Self) {
        self.x = snapped(self.x, step.x);
        self.y = snapped(self.y, step.y);
        self.z = snapped(self.z, step.z);
    }

    /// Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped(&self, step: &Self) -> Self {
        let mut v = *self;
        v.snap(step);
        v
    }

    fn snap_f(&mut self, step: float!()) {
        self.x = snapped(self.x, step);
        self.y = snapped(self.y, step);
        self.z = snapped(self.z, step);
    }

    /// Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped_f(&self, step: float!()) -> Self {
        let mut v = *self;
        v.snap_f(step);
        v
    }
}

impl From<Vector3i> for Vector3 {
    /// Constructs a new **Vector3** from [`Vector3i`].
    fn from(value: Vector3i) -> Self {
        Self::new(
            value.x as float!(),
            value.y as float!(),
            value.z as float!(),
        )
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3 {}

//TODO: impl_op_ex_commutative!(*|a: &Transform3D, b: &Vector3| -> Vector3 { todo!() });
//TODO: impl_op_ex_commutative!(*|a: &Quaternion, b: &Vector3| -> Vector3 { todo!() });

impl_op_ex!(*|a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
});

impl_op_ex_commutative!(*|a: &Vector3, b: &float!()| -> Vector3 {
    Vector3::new(a.x * b, a.y * b, a.z * b)
});

impl_op_ex_commutative!(*|a: &Vector3, b: int!()| -> Vector3 {
    Vector3::new(
        a.x * b as float!(),
        a.y * b as float!(),
        a.z * b as float!(),
    )
});

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z) });

impl_op_ex!(-|a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex!(/ |a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x / b.x, a.y / b.y, a.z / b.z) });

impl_op_ex!(/ |a: &Vector3, b: &float!()| -> Vector3 { Vector3::new(a.x/b, a.y/b, a.z/b) });

impl_op_ex!(/ |a: &Vector3, b: int!()| -> Vector3 { Vector3::new(a.x/b as float!(), a.y/b as float!(), a.z/b as float!()) });

impl_op_ex!(/= |a: &mut Vector3, b: &float!()| { a.x=a.x/b; a.y=a.y/b;a.z=a.z/b });

impl_op_ex!(/= |a: &mut Vector3, b: int!()| { a.x=a.x/b as float!(); a.y=a.y/b as float!(); a.z=a.z/b as float!() });

impl_op_ex!(/= |a: &mut Vector3, b: &Vector3| { a.x=a.x/b.x; a.y=a.y/b.y; a.z=a.z/b.z });

impl_op_ex!(*= |a: &mut Vector3, b: &float!()| { a.x=a.x*b; a.y=a.y*b; a.z=a.z*b});

impl_op_ex!(*= |a: &mut Vector3, b: int!()| { a.x=a.x*b as float!(); a.y=a.y*b as float!(); a.z=a.z*b as float!()});

impl_op_ex!(*= |a: &mut Vector3, b: &Vector3| { a.x=a.x*b.x; a.y=a.y*b.y; a.z=a.z*b.z});

//TODO: impl_op_ex!(*= |a: &mut Vector3, b: &Transform3D| { todo!() });
//TODO: impl_op_ex!(*= |a: &mut Vector3, b: &Basis| { todo!() });
//TODO: impl_op_ex!(*= |a: &mut Vector3, b: &Quaternion| { todo!() });

impl_op_ex!(+= |a: &mut Vector3, b: &float!()| { a.x=a.x+b; a.y=a.y+b; a.z=a.z+b });

impl_op_ex!(+= |a: &mut Vector3, b: int!()| { a.x=a.x+b as float!(); a.y=a.y+b as float!(); a.z=a.z+b as float!() });

impl_op_ex!(+= |a: &mut Vector3, b: &Vector3| { a.x=a.x+b.x; a.y=a.y+b.y; a.z=a.z+b.z });

impl_op_ex!(-= |a: &mut Vector3, b: &float!()| { a.x=a.x-b; a.y=a.y-b; a.z=a.z-b; });

impl_op_ex!(-= |a: &mut Vector3, b: int!()| { a.x=a.x-b as float!(); a.y=a.y-b as float!(); a.z=a.z-b as float!() });

impl_op_ex!(-= |a: &mut Vector3, b: &Vector3| { a.x=a.x-b.x; a.y=a.y-b.y; a.z=a.z-b.z });

impl Not for Vector3 {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

impl PartialOrd for Vector3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
    fn lt(&self, other: &Self) -> bool {
        if self.x < other.x {
            return true;
        } else if self.x == other.x {
            if self.y < other.y {
                return true;
            } else if self.y == other.y {
                return self.z < other.z;
            }
        }
        false
    }
    fn le(&self, other: &Self) -> bool {
        self < other || self == other
    }
    fn gt(&self, other: &Self) -> bool {
        if self.x > other.x {
            return true;
        } else if self.x == other.x {
            if self.y > other.y {
                return true;
            } else if self.y == other.y {
                return self.z > other.z;
            }
        }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Vector3({}, {}, {})", self.x, self.y, self.z))
    }
}
