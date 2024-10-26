use crate::types::vectors::{Vector2, AXIS};
use crate::utils::{float, int, snapped_i};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Neg, Not};

/// A 2D vector using integer coordinates.
///
/// A 2-element structure that can be used to represent 2D grid coordinates or any other pair of integers.
///
/// It uses integer coordinates and is therefore preferable to [`Vector2`] when exact precision is required. By default, these floating-point values use 32-bit precision. If double precision is needed, use the feature `double-precision`.
///
/// **Note:** In a boolean context, a Vector2i will evaluate to `false` if it's equal to `Vector2i::new(0, 0)`. Otherwise, a Vector2i will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector2i {
    /// The vector's X component. Also, accessible by using the index position `vec.get(0)`
    pub x: int!(),
    /// The vector's Y component. Also, accessible by using the index position `vec.get(1)`
    pub y: int!(),
}

impl Vector2i {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0, 0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1, 1);

    /// Min vector, a vector with all components equal to [`i32::MIN`]. Can be used as a negative integer equivalent of [`Vector2::INF`].
    pub const MIN: Self = Self::new(<int!()>::MIN, <int!()>::MIN);

    /// Max vector, a vector with all components equal to [`i32::MAX`]. Can be used as an integer equivalent of [`Vector2::INF`].
    pub const MAX: Self = Self::new(<int!()>::MAX, <int!()>::MAX);

    /// Left unit vector. Represents the direction of left.
    pub const LEFT: Self = Self::new(-1, 0);

    /// Right unit vector. Represents the direction of right.
    pub const RIGHT: Self = Self::new(1, 0);

    /// Up unit vector. Y is down in 2D, so this vector points -Y.
    pub const UP: Self = Self::new(0, -1);

    /// Down unit vector. Y is down in 2D, so this vector points +Y.
    pub const DOWN: Self = Self::new(0, 1);

    /// Constructs a new **Vector2i** from the given `x` and `y`.
    pub const fn new(x: int!(), y: int!()) -> Self {
        Self { x, y }
    }

    /// Access vector components using their index. `v.get(0)` is equivalent to `v.x`, and `v.get(1)` is equivalent to `v.y`.
    pub const fn get(&self, index: usize) -> int!() {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid index"),
        }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub const fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Returns the aspect ratio of this vector, the ratio of `x` to `y`.
    pub fn aspect(&self) -> float!() {
        self.x as float!() / self.y as float!()
    }

    /// Returns a new vector with all components clamped between the components of `min` and `max`, by running `clamp` on each component.
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Returns a new vector with all components clamped between `min` and `max`, by running `clamp` on each component.
    pub fn clamp_i(&self, min: int!(), max: int!()) -> Self {
        Self::new(self.x.clamp(min, max), self.y.clamp(min, max))
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector2i::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: &Self) -> int!() {
        (to - self).length_squared()
    }

    /// Returns the distance between this vector and to.
    pub fn distance_to(&self, to: &Self) -> float!() {
        (to - self).length()
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> float!() {
        (self.length_squared() as float!()).sqrt()
    }

    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector2i::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub const fn length_squared(&self) -> int!() {
        self.x * self.x + self.y * self.y
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector2i::new(x.max(with.x), y.max(with.y))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(self.x.max(with.x), self.y.max(with.y))
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub const fn max_axis_index(&self) -> AXIS {
        if self.x < self.y {
            AXIS::Y
        } else {
            AXIS::X
        }
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector2i::new(x.max(with), y.max(with))`.
    pub fn maxi(&self, with: int!()) -> Self {
        Self::new(self.x.max(with), self.y.max(with))
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector2i::new(x.min(with.x), y.min(with.y))`.
    pub fn min(&self, with: &Self) -> Self {
        Self::new(self.x.min(with.x), self.y.min(with.y))
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::Y`].
    pub const fn min_axis_index(&self) -> AXIS {
        if self.x < self.y {
            AXIS::X
        } else {
            AXIS::Y
        }
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector2i::new(x.min(with), y.min(with))`.
    pub fn min_i(&self, with: int!()) -> Self {
        Self::new(self.x.min(with), self.y.min(with))
    }

    /// Returns a new vector with each component set to `1` if it's positive, `-1` if it's negative, and `0` if it's zero. The result is identical to calling `signum` on each component.
    pub const fn sign(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    /// Returns a new vector with each component snapped to the closest multiple of the corresponding component in `step`.
    pub fn snapped(&self, step: &Self) -> Self {
        Self::new(snapped_i(self.x, step.x), snapped_i(self.y, step.y))
    }

    /// Returns a new vector with each component snapped to the closest multiple of `step`.
    pub fn snapped_i(&self, step: int!()) -> Self {
        Self::new(snapped_i(self.x, step), snapped_i(self.y, step))
    }

    pub fn set(&mut self, index: usize, value: int!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> int!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: int!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            _ => panic!("Invalid axis"),
        }
    }
}

impl From<Vector2> for Vector2i {
    /// Constructs a new **Vector2i** from the given [`Vector2`] by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of [`Vector2::ceil`], [`Vector2::floor`] or [`Vector2::round`] to this constructor instead.
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x.trunc() as int!(),
            y: value.y.trunc() as int!(),
        }
    }
}

impl PartialEq for Vector2i {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vector2i {}

impl_op_ex!(% |a: &Vector2i, b: &Vector2i| -> Vector2i {Vector2i::new(a.x % b.x, a.y % b.y)});

impl_op_ex!(% |a: &Vector2i, b: &int!()| -> Vector2i { Vector2i::new(a.x%b, a.y%b) });

impl_op_ex_commutative!(*|a: &Vector2i, b: &float!()| -> Vector2 {
    Vector2::new(a.x as float!() * b, a.y as float!() * b)
});

impl_op_ex_commutative!(*|a: &Vector2i, b: &int!()| -> Vector2i {
    Vector2i::new(a.x * b, a.y * b)
});

impl_op_ex!(*|a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x * b.x, a.y * b.y) });

impl_op_ex!(+ |a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x + b.x, a.y + b.y) });

impl_op_ex!(-|a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x - b.x, a.y - b.y) });

impl_op_ex!(/ |a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x / b.x, a.y / b.y) });

impl_op_ex!(/ |a: &Vector2i, b: &float!()| -> Vector2 { Vector2::new(a.x as float!() / b, a.y as float!() / b) });

impl_op_ex!(/ |a: &Vector2i, b: &int!()| -> Vector2i { Vector2i::new(a.x / b, a.y / b) });

impl_op_ex!(%= |a: &mut Vector2i, b: &Vector2i| {a.x = a.x % b.x; a.y = a.y % b.y});
impl_op_ex!(%= |a: &mut Vector2i, b: &int!()| {a.x = a.x % b; a.y = a.y % b});

impl_op_ex!(/= |a: &mut Vector2i, b: &int!()| { a.x = a.x / b; a.y = a.y / b; });

impl_op_ex!(/= |a: &mut Vector2i, b: &Vector2i| { a.x = a.x / b.x; a.y = a.y / b.y; });

impl_op_ex!(*= |a: &mut Vector2i, b: &int!()| { a.x = a.x * b; a.y = a.y * b; });

impl_op_ex!(*= |a: &mut Vector2i, b: &Vector2i| { a.x = a.x * b.x; a.y = a.y * b.y; });

impl_op_ex!(+= |a: &mut Vector2i, b: &int!()| { a.x = a.x + b; a.y = a.y + b; });

impl_op_ex!(+= |a: &mut Vector2i, b: &Vector2i| { a.x = a.x + b.x; a.y = a.y + b.y; });

impl_op_ex!(-= |a: &mut Vector2i, b: &int!()| { a.x = a.x - b; a.y = a.y - b; });

impl_op_ex!(-= |a: &mut Vector2i, b: &Vector2i| { a.x = a.x - b.x; a.y = a.y - b.y; });

impl Not for Vector2i {
    type Output = bool;
    fn not(self) -> Self::Output {
        self.x == 0 && self.y == 0
    }
}

impl PartialOrd for Vector2i {
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

impl Neg for Vector2i {
    type Output = Vector2i;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vector2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Vector2i({}, {})", self.x, self.y))
    }
}
