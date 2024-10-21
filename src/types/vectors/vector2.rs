use crate::types::vectors::vector2i::Vector2i;
use crate::types::vectors::AXIS;
use crate::utils::{
    bezier_derivative, bezier_interpolate, cubic_interpolate, cubic_interpolate_in_time, float,
    int, is_equal_approx, is_equal_approx_with_tolerance, is_zero_approx, posmod_f, snapped,
    FloatExt, CMP_EPSILON, UNIT_EPSILON,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Neg, Not};

/// A 2D vector using floating-point coordinates.
///
/// A 2-element structure that can be used to represent 2D coordinates or any other pair of numeric values.
///
/// It uses floating-point coordinates. By default, these floating-point values use 32-bit precision. If double precision is needed, use the feature `double-precision`.
///
/// See [`Vector2i`] for its integer counterpart.
///
/// **Note:** In a boolean context, a Vector2 will evaluate to `false` if it's equal to `Vector2::(0.0, 0.0)`. Otherwise, a Vector2 will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector2 {
    /// The vector's X component. Also, accessible by using the index position `vec.get(0)`.
    pub x: float!(),
    /// The vector's Y component. Also, accessible by using the index position `vec.get(1)`.
    pub y: float!(),
}

impl From<Vector2i> for Vector2 {
    /// Constructs a new Vector2 from [`Vector2i`].
    fn from(value: Vector2i) -> Self {
        Self {
            x: value.x as float!(),
            y: value.y as float!(),
        }
    }
}

impl Vector2 {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1.0, 1.0);

    /// Infinity vector, a vector with all components set to [`float!()::INFINITY`]
    pub const INF: Self = Self::new(<float!()>::INFINITY, <float!()>::INFINITY);

    /// Left unit vector. Represents the direction of left.
    pub const LEFT: Self = Self::new(-1.0, 0.0);

    /// Right unit vector. Represents the direction of right.
    pub const RIGHT: Self = Self::new(1.0, 0.0);

    /// Up unit vector. Y is down in 2D, so this vector points -Y.
    pub const UP: Self = Self::new(0.0, -1.0);

    /// Down unit vector. Y is down in 2D, so this vector points +Y.
    pub const DOWN: Self = Self::new(0.0, 1.0);

    /// Constructs a new **Vector2** from the given `x` and `y`.
    pub const fn new(x: float!(), y: float!()) -> Self {
        Self { x, y }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns this vector's angle with respect to the positive X axis, or `(1, 0)` vector, in radians.
    ///
    /// For example, [`Vector2::RIGHT.angle()`] will return zero, [`Vector2::DOWN.angle()`] will return `PI / 2` (a quarter turn, or 90 degrees), and [`Vector2::new(1.0, -1.0).angle()`] will return `-PI / 4` (a negative eighth turn, or -45 degrees).
    ///
    /// [Illustration of the returned angle](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle.png).
    ///
    /// Equivalent to the result of `atan2` when called with the vector's `y` and `x` as parameters: `y.atan2(x)`.
    pub fn angle(&self) -> float!() {
        self.y.atan2(self.x)
    }

    /// Returns the angle to the given vector, in radians.
    ///
    /// [Illustration of the returned angle](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to.png).
    pub fn angle_to(&self, to: &Self) -> float!() {
        self.cross(to).atan2(self.dot(to))
    }

    /// Returns the angle between the line connecting the two points and the X axis, in radians.
    ///
    /// a.angle_to_point(b) is equivalent of doing (b - a).angle().
    ///
    /// [Illustration of the returned angle](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to_point.png).
    pub fn angle_to_point(&self, to: &Self) -> float!() {
        (*to - *self).angle()
    }

    /// Returns the aspect ratio of this vector, the ratio of `x` to `y`.
    pub fn aspect(&self) -> float!() {
        self.x / self.y
    }

    /// Returns the derivative at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points.
    pub fn bezier_derivation(
        &self,
        control_1: &Self,
        control_2: &Self,
        end: &Self,
        t: float!(),
    ) -> Self {
        Self {
            x: bezier_derivative(self.x, control_1.x, control_2.x, end.x, t),
            y: bezier_derivative(self.y, control_1.y, control_2.y, end.y, t),
        }
    }

    /// Returns the point at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points.
    pub fn bezier_interpolate(
        &self,
        control_1: &Self,
        control_2: &Self,
        end: &Self,
        t: float!(),
    ) -> Self {
        Self {
            x: bezier_interpolate(self.x, control_1.x, control_2.x, end.x, t),
            y: bezier_interpolate(self.y, control_1.y, control_2.y, end.y, t),
        }
    }

    /// Returns the vector "bounced off" from a line defined by the given normal `n` perpendicular to the line.
    ///
    /// **Note:** [`Vector2::bounce`] performs the operation that most engines and frameworks call `reflect()`.
    pub fn bounce(&self, n: &Self) -> Self {
        -self.reflect(n)
    }

    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Returns a new vector with all components clamped between the components of `min` and `max`, by running [`float!()::clamp`] on each component.
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /// Returns a new vector with all components clamped between `min` and `max`, by running `clamp` on each component.
    pub fn clamp_f(&self, min: float!(), max: float!()) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }

    /// Returns the 2D analog of the cross product for this vector and `with`.
    ///
    /// This is the signed area of the parallelogram formed by the two vectors. If the second vector is clockwise from the first vector, then the cross product is the positive area. If counter-clockwise, the cross product is the negative area. If the two vectors are parallel this returns zero, making it useful for testing if two vectors are parallel.
    ///
    /// **Note:** Cross product is not defined in 2D mathematically. This method embeds the 2D vectors in the XY plane of 3D space and uses their cross product's Z component as the analog.
    pub fn cross(&self, with: &Self) -> float!() {
        self.x * with.y - self.y * with.x
    }

    /// Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    pub fn cubic_interpolate(
        &self,
        b: &Self,
        pre_a: &Self,
        post_b: &Self,
        weight: float!(),
    ) -> Self {
        Self {
            x: cubic_interpolate(self.x, b.x, pre_a.x, post_b.x, weight),
            y: cubic_interpolate(self.y, b.y, pre_a.y, post_b.y, weight),
        }
    }

    /// Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    ///
    /// It can perform smoother interpolation than [`Vector2::cubic_interpolate`] by the time values.
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
        Self {
            x: cubic_interpolate_in_time(
                self.x, b.x, pre_a.x, post_b.x, weight, b_t, pre_a_t, post_b_t,
            ),
            y: cubic_interpolate_in_time(
                self.y, b.y, pre_a.y, post_b.y, weight, b_t, pre_a_t, post_b_t,
            ),
        }
    }

    /// Returns the normalized vector pointing from this vector to `to`. This is equivalent to using `(b - a).normalized()`.
    pub fn direction_to(&self, to: &Self) -> Self {
        (*to - *self).normalized()
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector2::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: &Self) -> float!() {
        (self.x - to.x) * (self.x - to.x) + (self.y - to.y) * (self.y - to.y)
    }

    /// Returns the distance between this vector and `to`.
    pub fn distance_to(&self, to: &Self) -> float!() {
        self.distance_squared_to(to).sqrt()
    }

    /// Returns the dot product of this vector and `with`. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.
    ///
    /// The dot product will be `0` for a right angle (90-degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90-degrees.
    ///
    /// When using unit (normalized) vectors, the result will always be between `-1.0` (180-degree angle) when the vectors are facing opposite directions, and `1.0` (0-degree angle) when the vectors are aligned.
    ///
    /// **Note:** `a.dot(b)` *is* equivalent to `b.dot(a)`.
    pub fn dot(&self, with: &Self) -> float!() {
        self.x * with.x + self.y * with.y
    }

    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Creates a unit Vector2 rotated to the given angle in radians. This is equivalent to doing Vector2(cos(angle), sin(angle)) or Vector2.RIGHT.rotated(angle).
    /// ```
    /// # use huginn::types::vectors::Vector2;
    /// # use huginn::utils::float_consts::PI;
    /// println!("{:?}", Vector2::from_angle(0.0)); // Prints (1, 0).
    /// println!("{:?}", Vector2::new(1.0, 0.0).angle()); // Prints 0, which is the angle used above.
    /// println!("{:?}", Vector2::from_angle(PI / 2.0)); // Prints (0, 1).
    /// ```
    pub fn from_angle(angle: float!()) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    /// Access vector components using their `index`. `v.get(0)` is equivalent to `v.x`, and `v.get(1)` is equivalent to `v.y`.
    pub const fn get(&self, index: usize) -> float!() {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, index: usize, value: float!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> float!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: float!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            _ => panic!("Invalid axis"),
        }
    }

    /// Returns `true` if this vector and `to` are approximately equal, by running [`is_equal_approx`] on each component.
    pub fn is_equal_approx(&self, to: &Self) -> bool {
        is_equal_approx(self.x, to.x) && is_equal_approx(self.y, to.y)
    }

    /// Returns `true` if this vector is finite, by calling `is_finite` on each component.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns `true` if the vector is normalized, i.e. its length is approximately equal to `1`.
    pub fn is_normalized(&self) -> bool {
        is_equal_approx_with_tolerance(self.length_squared(), 1.0, UNIT_EPSILON)
    }

    /// Returns `true` if this vector's values are approximately zero, by running [`is_zero_approx`] on each component.
    ///
    /// This method is faster than using [`Vector2::is_equal_approx`] with one value as a zero vector.
    pub fn is_zero_approx(&self) -> bool {
        is_zero_approx(self.x) && is_zero_approx(self.y)
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> float!() {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector2::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn length_squared(&self) -> float!() {
        self.x * self.x + self.y * self.y
    }

    /// Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    pub fn lerp(&self, to: &Self, weight: float!()) -> Self {
        Self {
            x: self.x.lerp(to.x, weight),
            y: self.y.lerp(to.y, weight),
        }
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

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector2::new(vec.x.max(with.x), vec.y.max(with.y))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(self.x.max(with.x), self.y.max(with.y))
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub fn max_axis_index(&self) -> AXIS {
        if self.x < self.y {
            AXIS::Y
        } else {
            AXIS::X
        }
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector2::(vec.x.max(with), vec.y.mac(with))`.
    pub fn max_f(&self, with: float!()) -> Self {
        Self::new(self.x.max(with), self.y.max(with))
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector2::new(vec.x.min(with.x), vec.y.min(with.y))`.
    pub fn min(&self, with: &Self) -> Self {
        Self::new(self.x.min(with.x), self.y.min(with.y))
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::Y`].
    pub fn min_axis_index(&self) -> AXIS {
        if self.x < self.y {
            AXIS::X
        } else {
            AXIS::Y
        }
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector2::new(ve.x.min(with), vec.y.min(with))`.
    pub fn min_f(&self, with: float!()) -> Self {
        Self::new(self.x.min(with), self.y.min(with))
    }

    /// Returns a new vector moved toward `to` by the fixed `delta` amount. Will not go past the final value.
    pub fn move_toward(&self, to: &Self, delta: float!()) -> Self {
        let v = *self;
        let vd = *to - v;
        let len = vd.length();
        if len <= delta || len < CMP_EPSILON {
            *to
        } else {
            v + vd / len * delta
        }
    }

    /// Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `(0, 0)` if `v.length() == 0`. See also [`Vector2::is_normalized`].
    ///
    /// **Note:** This function may return incorrect values if the input vector length is near zero.
    pub fn normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    /// Scales the vector to unit length. Equivalent to `v / v.length()`. Scales to `(0, 0)` if `v.length() == 0`.
    ///
    /// **Note:** This function may scale incorrectly if the vector length is near zero.
    fn normalize(&mut self) {
        let mut l = self.length_squared();
        if l != 0.0 {
            l = l.sqrt();
            self.x /= l;
            self.y /= l;
        }
    }

    /// Returns a perpendicular vector rotated 90-degrees counter-clockwise compared to the original, with the same length.
    pub fn orthogonal(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module`.
    pub const fn posmod(&self, module: float!()) -> Self {
        Self::new(posmod_f(self.x, module), posmod_f(self.y, module))
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module_v`'s components.
    pub const fn posmodv(&self, module_v: &Self) -> Self {
        Self::new(posmod_f(self.x, module_v.x), posmod_f(self.y, module_v.y))
    }

    /// Returns a new vector resulting from projecting this vector onto the given vector `b`. The resulting new vector is parallel to `b`. See also [`Vector2::slide`].
    ///
    /// **Note:** If the vector `b` is a zero vector, the components of the resulting new vector will be `NaN`.
    pub fn project(&self, b: &Self) -> Self {
        *b * (self.dot(b) / b.length_squared())
    }

    /// Returns the result of reflecting the vector from a line defined by the given direction vector `line`.
    ///
    /// **Note:** [`Vector2::reflect`] differs from what other engines and frameworks call `reflect()`. In other engines, `reflect()` takes a normal direction which is a direction perpendicular to the line. In Grimm, you specify the direction of the line directly. See also [`Vector2::bounce`] which does what most engines call `reflect()`.
    pub fn reflect(&self, line: &Self) -> Self {
        2.0 * line * self.dot(line) - self
    }

    /// Returns the result of rotating this vector by `angle` (in radians).
    pub fn rotated(&self, angle: float!()) -> Self {
        let sine = angle.sin();
        let cosine = angle.cos();
        Self::new(
            self.x * cosine - self.y * sine,
            self.x * sine + self.y * cosine,
        )
    }

    /// Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    /// Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling `sign` on each component.
    pub fn sign(&self) -> Self {
        Self::new(self.x.sign(), self.y.sign())
    }

    /// Returns the result of spherical linear interpolation between this vector and `to`, by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    ///
    /// This method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like [`Vector2::lerp`].
    pub fn slerp(&self, to: &Self, weight: float!()) -> Self {
        let start_length_sq = self.length_squared();
        let end_length_sq = to.length_squared();
        if start_length_sq == 0.0 || end_length_sq == 0.0 {
            // Zero length vectors have no angle, so the best we can do is either lerp or throw an error.
            return self.lerp(to, weight);
        }
        let start_length = start_length_sq.sqrt();
        let result_length = start_length.lerp(end_length_sq.sqrt(), weight);
        let angle = self.angle_to(to);
        self.rotated(angle * weight) * (result_length / start_length)
    }

    /// Returns a new vector resulting from sliding this vector along a line with normal `n`. The resulting new vector is perpendicular to `n`, and is equivalent to this vector minus its projection on `n`. See also [`Vector2::project`].
    ///
    /// **Note:** The vector n must be normalized. See also [`Vector2::normalized`].
    pub fn slide(&self, n: &Self) -> Self {
        self - n * self.dot(n)
    }

    /// Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped(&self, step: &Self) -> Self {
        Self::new(snapped(self.x, step.x), snapped(self.y, step.y))
    }

    /// Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped_f(&self, step: float!()) -> Self {
        Self::new(snapped(self.x, step), snapped(self.y, step))
    }

    pub fn plane_project(&self, d: float!(), vec: &Self) -> Self {
        vec - self * (self.dot(vec) - d)
    }
}
impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vector2 {}

//TODO: impl_op_ex_commutative!(*|a: &Transform2D, b: &Vector2| -> Vector2 { todo!() });

impl_op_ex!(*|a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x * b.x, a.y * b.y) });

impl_op_ex_commutative!(*|a: &Vector2, b: &float!()| -> Vector2 { Vector2::new(a.x * b, a.y * b) });

impl_op_ex_commutative!(*|a: &Vector2, b: int!()| -> Vector2 {
    Vector2::new(a.x * b as float!(), a.y * b as float!())
});

impl_op_ex!(+ |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x + b.x, a.y + b.y) });

impl_op_ex!(-|a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x - b.x, a.y - b.y) });

impl_op_ex!(/ |a: &Vector2, b: &Vector2| -> Vector2 { Vector2::new(a.x / b.x, a.y / b.y) });

impl_op_ex!(/ |a: &Vector2, b: &float!()| -> Vector2 { Vector2::new(a.x/b, a.y/b) });

impl_op_ex!(/ |a: &Vector2, b: int!()| -> Vector2 { Vector2::new(a.x/b as float!(), a.y/b as float!()) });

impl_op_ex!(/= |a: &mut Vector2, b: &float!()| { a.x=a.x/b; a.y=a.y/b; });

impl_op_ex!(/= |a: &mut Vector2, b: int!()| { a.x=a.x/b as float!(); a.y=a.y/b as float!(); });

impl_op_ex!(/= |a: &mut Vector2, b: &Vector2| { a.x=a.x/b.x; a.y=a.y/b.y; });

impl_op_ex!(*= |a: &mut Vector2, b: &float!()| { a.x=a.x*b; a.y=a.y*b; });

impl_op_ex!(*= |a: &mut Vector2, b: int!()| { a.x=a.x*b as float!(); a.y=a.y*b as float!(); });

impl_op_ex!(*= |a: &mut Vector2, b: &Vector2| { a.x=a.x*b.x; a.y=a.y*b.y; });

//TODO: impl_op_ex!(*= |a: &mut Vector2, b: &Transform2D| { todo!() });

impl_op_ex!(+= |a: &mut Vector2, b: &float!()| { a.x=a.x+b; a.y=a.y+b; });

impl_op_ex!(+= |a: &mut Vector2, b: int!()| { a.x=a.x+b as float!(); a.y=a.y+b as float!(); });

impl_op_ex!(+= |a: &mut Vector2, b: &Vector2| { a.x=a.x+b.x; a.y=a.y+b.y; });

impl_op_ex!(-= |a: &mut Vector2, b: &float!()| { a.x=a.x-b; a.y=a.y-b; });

impl_op_ex!(-= |a: &mut Vector2, b: int!()| { a.x=a.x-b as float!(); a.y=a.y-b as float!(); });

impl_op_ex!(-= |a: &mut Vector2, b: &Vector2| { a.x=a.x-b.x; a.y=a.y-b.y; });

impl Not for Vector2 {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.x == 0.0 && self.y == 0.0
    }
}

impl PartialOrd for Vector2 {
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
            return self.y < other.y;
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
            return self.y > other.y;
        }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}
