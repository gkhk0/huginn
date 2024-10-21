use crate::types::vectors::AXIS;
use crate::utils::{
    cubic_interpolate, cubic_interpolate_in_time, float, int, is_equal_approx,
    is_equal_approx_with_tolerance, is_zero_approx, posmod_f, snapped, FloatExt, UNIT_EPSILON,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Neg, Not};

/// A 4D vector using floating-point coordinates.
///
/// A 4-element structure that can be used to represent 4D coordinates or any other quadruplet of numeric values.
///
/// It uses floating-point coordinates. By default, these floating-point values use 32-bit precision. If double precision is needed, use the feature flag `double-precision`.
///
/// See [`Vector4i`] for its integer counterpart.
///
/// **Note:** In a boolean context, a Vector4 will evaluate to `false` if it's equal to `Vector4(0, 0, 0, 0)`. Otherwise, a Vector4 will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector4 {
    /// The vector's W component. Also, accessible by using the index position `v.get(3)`.
    pub w: float!(),
    /// The vector's X component. Also, accessible by using the index position `v.get(0)`.
    pub x: float!(),
    /// The vector's Y component. Also, accessible by using the index position `v.get(1)`.
    pub y: float!(),
    /// The vector's Z component. Also, accessible by using the index position `v.get(2)`.
    pub z: float!(),
}

impl Vector4 {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// Infinity vector, a vector with all components set to [`f32::INFINITY`].
    pub const INF: Self = Self::new(
        <float!()>::INFINITY,
        <float!()>::INFINITY,
        <float!()>::INFINITY,
        <float!()>::INFINITY,
    );

    /// Returns a **Vector4** with the given components.
    pub const fn new(x: float!(), y: float!(), z: float!(), w: float!()) -> Self {
        Self { w, x, y, z }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }

    /// Returns a new vector with all components clamped between the components of `min` and `max`, by running `clamp` on each component.
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
            self.w.clamp(min.w, max.w),
        )
    }

    /// Returns a new vector with all components clamped between `min` and `max`, by running `clamp` on each component.
    pub fn clamp_f(&self, min: float!(), max: float!()) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
            self.w.clamp(min, max),
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
            cubic_interpolate(self.w, b.w, pre_a.w, post_b.w, weight),
        )
    }

    /// Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    ///
    /// It can perform smoother interpolation than [`Vector4::cubic_interpolate`] by the time values.
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
            cubic_interpolate_in_time(
                self.w, b.w, pre_a.w, post_b.w, weight, b_t, pre_a_t, post_b_t,
            ),
        )
    }

    /// Returns the normalized vector pointing from this vector to `to`. This is equivalent to using `(b - a).normalized()`.
    pub fn direction_to(&self, to: &Self) -> Self {
        (to - self).normalized()
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector4::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: &Self) -> float!() {
        (to - self).length_squared()
    }

    /// Returns the distance between this vector and `to`.
    pub fn distance_to(&self, to: &Self) -> float!() {
        (to - self).length()
    }

    /// Returns the dot product of this vector and `with`.
    pub fn dot(&self, with: &Self) -> float!() {
        self.x * with.x + self.y * with.y + self.z * with.z + self.w * with.w
    }

    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
            self.w.floor(),
        )
    }

    /// Returns the inverse of the vector. This is the same as `Vector4::new(1.0 / v.x, 1.0 / v.y, 1.0 / v.z, 1.0 / v.w)`.
    pub fn inverse(&self) -> Self {
        Vector4::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z, 1.0 / self.w)
    }

    /// Returns `true` if this vector and `to` are approximately equal, by running `is_equal_approx` on each component.
    pub fn is_equal_approx(&self, to: &Self) -> bool {
        is_equal_approx(self.x, to.x)
            && is_equal_approx(self.y, to.y)
            && is_equal_approx(self.z, to.z)
            && is_equal_approx(self.w, to.w)
    }

    /// Returns `true` if this vector is finite, by calling `is_finite` on each component.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns true if the vector is normalized, i.e. its length is approximately equal to 1.
    pub fn is_normalized(&self) -> bool {
        is_equal_approx_with_tolerance(self.length_squared(), 1.0, UNIT_EPSILON)
    }

    /// Returns `true` if this vector's values are approximately zero, by running `is_zero_approx` on each component.
    ///
    /// This method is faster than using [`Vector4::is_equal_approx`] with one value as a zero vector.
    pub fn is_zero_approx(&self) -> bool {
        is_zero_approx(self.x)
            && is_zero_approx(self.y)
            && is_zero_approx(self.z)
            && is_zero_approx(self.w)
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> float!() {
        self.length_squared().sqrt()
    }

    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector4::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn length_squared(&self) -> float!() {
        self.dot(self)
    }

    /// Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation.
    pub fn lerp(&self, to: &Self, weight: float!()) -> Self {
        Self::new(
            self.x.lerp(to.x, weight),
            self.y.lerp(to.y, weight),
            self.z.lerp(to.z, weight),
            self.w.lerp(to.w, weight),
        )
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector4::new(x.max(with.x), y.max(with.y), z.max(with.z), w.max(with.w))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(
            self.x.max(with.x),
            self.y.max(with.y),
            self.z.max(with.z),
            self.w.max(with.w),
        )
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub fn max_axis_index(&self) -> AXIS {
        if self.x < self.y {
            if self.y < self.z {
                if self.z < self.w {
                    AXIS::W
                } else {
                    AXIS::Z
                }
            } else if self.y < self.w {
                AXIS::W
            } else {
                AXIS::Y
            }
        } else if self.x < self.z {
            if self.z < self.w {
                AXIS::W
            } else {
                AXIS::Z
            }
        } else if self.x < self.w {
            AXIS::W
        } else {
            AXIS::X
        }
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector4::new(x.max(with), y.max(with), z.max(with), w.max(with))`.
    pub fn max_f(&self, with: float!()) -> Self {
        Self::new(
            self.x.max(with),
            self.y.max(with),
            self.z.max(with),
            self.w.max(with),
        )
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector4::new(x.min(with.x), y.min(with.y), z.min(with.z), w.min(with.w)).
    pub fn min(&self, with: &Self) -> Self {
        Self::new(
            self.x.min(with.x),
            self.y.min(with.y),
            self.z.min(with.z),
            self.w.min(with.w),
        )
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::W`].
    pub fn min_axis_index(&self) -> AXIS {
        if self.x < self.y {
            if self.x < self.z {
                if self.x < self.w {
                    AXIS::X
                } else {
                    AXIS::W
                }
            } else if self.z < self.w {
                AXIS::Z
            } else {
                AXIS::W
            }
        } else if self.y < self.z {
            if self.y < self.w {
                AXIS::Y
            } else {
                AXIS::W
            }
        } else if self.z < self.w {
            AXIS::Z
        } else {
            AXIS::W
        }
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector4::new(x.min(with), y.min(with), z.min(with), w.min(with))`.
    pub fn min_f(&self, with: float!()) -> Self {
        Self::new(
            self.x.min(with),
            self.y.min(with),
            self.z.min(with),
            self.w.min(with),
        )
    }

    fn normalize(&mut self) {
        let length_sq = self.length_squared();
        if length_sq == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            self.w = 0.0;
        } else {
            let length = length_sq.sqrt();
            self.x /= length;
            self.y /= length;
            self.z /= length;
            self.w /= length;
        }
    }

    /// Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `Vector4(0, 0, 0, 0)` if `v.length() == 0`. See also [`Vector4::is_normalized`].
    ///
    /// **Note:** This function may return incorrect values if the input vector length is near zero.
    pub fn normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module`.
    pub const fn posmod(&self, module: float!()) -> Self {
        Self::new(
            posmod_f(self.x, module),
            posmod_f(self.y, module),
            posmod_f(self.z, module),
            posmod_f(self.w, module),
        )
    }

    /// Returns a vector composed of the `posmod_f` of this vector's components and `module_v`'s components.
    pub const fn posmod_v(&self, module_v: &Self) -> Self {
        Self::new(
            posmod_f(self.x, module_v.x),
            posmod_f(self.y, module_v.y),
            posmod_f(self.z, module_v.z),
            posmod_f(self.w, module_v.w),
        )
    }

    /// Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.
    pub fn round(&self) -> Self {
        Self::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }

    /// Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling `sign` on each component.
    pub fn sign(&self) -> Self {
        Self::new(self.x.sign(), self.y.sign(), self.z.sign(), self.w.sign())
    }

    /// Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped(&self, step: &Self) -> Self {
        Self::new(
            snapped(self.x, step.x),
            snapped(self.y, step.y),
            snapped(self.z, step.z),
            snapped(self.w, step.w),
        )
    }

    /// Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals.
    pub fn snapped_f(&self, step: float!()) -> Self {
        Self::new(
            snapped(self.x, step),
            snapped(self.y, step),
            snapped(self.z, step),
            snapped(self.w, step),
        )
    }

    pub const fn get(&self, index: usize) -> float!() {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, index: usize, value: float!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> float!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            AXIS::Z => self.z,
            AXIS::W => self.w,
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: float!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            AXIS::Z => self.z = value,
            AXIS::W => self.w = value,
        }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Eq for Vector4 {}

// TODO: impl_op_ex_commutative!(* |a: &Vector4, b: &Projection| -> Vector4 {});

impl_op_ex!(*|a: &Vector4, b: &Vector4| -> Vector4 {
    Vector4::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w)
});

impl_op_ex!(*= |a: &mut Vector4, b: &Vector4| {
    a.x = a.x * b.x;
    a.y = a.y * b.y;
    a.z = a.z * b.z;
    a.w = a.w * b.w;
});

impl_op_ex_commutative!(*|a: &Vector4, b: &float!()| -> Vector4 {
    Vector4::new(a.x * b, a.y * b, a.z * b, a.w * b)
});

impl_op_ex!(*= |a: &mut Vector4, b: &float!()| {
    a.x = a.x * b;
    a.y = a.y * b;
    a.z = a.z * b;
    a.w = a.w * b;
});

impl_op_ex_commutative!(*|a: &Vector4, b: int!()| -> Vector4 {
    Vector4::new(
        a.x * b as float!(),
        a.y * b as float!(),
        a.z * b as float!(),
        a.w * b as float!(),
    )
});

impl_op_ex!(*= |a: &mut Vector4, b: int!()| {
    a.x = a.x * b as float!();
    a.y = a.y * b as float!();
    a.z = a.z * b as float!();
    a.w = a.w * b as float!();
});

impl_op_ex!(+ |a: &Vector4, b: &Vector4| -> Vector4 {
    Vector4::new(
        a.x + b.x,
        a.y + b.y,
        a.z + b.z,
        a.w + b.w,
    )
});

impl_op_ex!(+= |a: &mut Vector4, b: &Vector4|{
    a.x = a.x + b.x;
    a.y = a.y + b.y;
    a.z = a.z + b.z;
    a.w = a.w + b.w;
});

impl_op_ex!(-|a: &Vector4, b: &Vector4| -> Vector4 {
    Vector4::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w)
});

impl_op_ex!(-= |a: &mut Vector4, b: &Vector4| {
    a.x = a.x - b.x;
    a.y = a.y - b.y;
    a.z = a.z - b.z;
    a.w = a.w - b.w;
});

impl_op_ex!(/ |a: &Vector4, b: &Vector4| -> Vector4 {
    Vector4::new(
        a.x / b.x,
        a.y / b.y,
        a.z / b.z,
        a.w / b.w,
    )
});

impl_op_ex!(/= |a: &mut Vector4, b: &Vector4| {
    a.x = a.x / b.x;
    a.y = a.y / b.y;
    a.z = a.z / b.z;
    a.w = a.w / b.w;
});

impl_op_ex_commutative!(/ |a: &Vector4, b: &float!()| -> Vector4 {
    Vector4::new(
        a.x / b,
        a.y / b,
        a.z / b,
        a.w / b,
    )
});

impl_op_ex!(/= |a: &mut Vector4, b: &float!()| {
    a.x = a.x / b;
    a.y = a.y / b;
    a.z = a.z / b;
    a.w = a.w / b;
});

impl_op_ex_commutative!(/ |a: &Vector4, b: int!()| -> Vector4 {
    Vector4::new(
        a.x / b as float!(),
        a.y / b as float!(),
        a.z / b as float!(),
        a.w / b as float!(),
    )
});

impl_op_ex!(/= |a: &mut Vector4, b: int!()| {
    a.x = a.x / b as float!();
    a.y = a.y / b as float!();
    a.z = a.z / b as float!();
    a.w = a.w / b as float!();
});

impl Not for Vector4 {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0
    }
}

impl PartialOrd for Vector4 {
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
                if self.z < other.z {
                    return true;
                } else if self.z == other.z {
                    return self.w < other.w;
                }
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
                if self.z > other.z {
                    return true;
                } else if self.z == other.z {
                    return self.w > other.w;
                }
            }
        }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }
}

impl Neg for Vector4 {
    type Output = Vector4;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Vector4({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}
