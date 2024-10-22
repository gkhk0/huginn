use crate::types::vectors::{Vector4, AXIS};
use crate::utils::{float, int, snapped_i};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Neg, Not};

/// A 4D vector using integer coordinates.
///
/// A 4-element structure that can be used to represent 4D grid coordinates or any other quadruplet of integers.
///
/// It uses integer coordinates and is therefore preferable to [`Vector4`] when exact precision is required.
///
/// **Note:** In a boolean context, a Vector4i will evaluate to `false` if it's equal to `Vector4i(0, 0, 0, 0)`. Otherwise, a Vector4i will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector4i {
    /// The vector's W component. Also, accessible by using the index position `v.get(3)`.
    pub w: int!(),
    /// The vector's X component. Also, accessible by using the index position `v.get(0)`.
    pub x: int!(),
    /// The vector's Y component. Also, accessible by using the index position `v.get(1)`.
    pub y: int!(),
    /// The vector's Z component. Also, accessible by using the index position `v.get(2)`.
    pub z: int!(),
}

impl Vector4i {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0, 0, 0, 0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1, 1, 1, 1);

    /// Min vector, a vector with all components equal to [`i32::MIN`]. Can be used as a negative integer equivalent of [`Vector4::INF`].
    pub const MIN: Self = Self::new(<int!()>::MIN, <int!()>::MIN, <int!()>::MIN, <int!()>::MIN);

    /// Max vector, a vector with all components equal to [`i32::MAX`]. Can be used as an integer equivalent of [`Vector4::INF`].
    pub const MAX: Self = Self::new(<int!()>::MAX, <int!()>::MAX, <int!()>::MAX, <int!()>::MAX);

    /// Returns a **Vector4i** with the given components.
    pub const fn new(x: int!(), y: int!(), z: int!(), w: int!()) -> Self {
        Self { w, x, y, z }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
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
    pub fn clamp_i(&self, min: int!(), max: int!()) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
            self.w.clamp(min, max),
        )
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector4i::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: &Self) -> int!() {
        (to - self).length_squared()
    }

    /// Returns the distance between this vector and `to`.
    pub fn distance_to(&self, to: &Self) -> float!() {
        (to - self).length()
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> float!() {
        (self.length_squared() as float!()).sqrt()
    }

    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector4i::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub const fn length_squared(&self) -> int!() {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector4i::new(x.max(with.x), y.max(with.y), z.max(with.z), w.max(with.w))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(
            self.x.max(with.x),
            self.y.max(with.y),
            self.z.max(with.z),
            self.w.max(with.w),
        )
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub const fn max_axis_index(&self) -> AXIS {
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

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector4i::new(x.max(with), y.max(with), z.max(with), w.max(with)).
    pub fn max_i(&self, with: int!()) -> Self {
        Self::new(
            self.x.max(with),
            self.y.max(with),
            self.z.max(with),
            self.w.max(with),
        )
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector4i::new(x.min(with.x), y.min(with.y), z.min(with.z), w.min(with.w))`.
    pub fn min(&self, with: &Self) -> Self {
        Self::new(
            self.x.min(with.x),
            self.y.min(with.y),
            self.z.min(with.z),
            self.w.min(with.w),
        )
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::W`].
    pub const fn min_axis_index(&self) -> AXIS {
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

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector4i::new(x.min(with), y.min(with), z.min(with), w.min(with))`.
    pub fn min_i(&self, with: int!()) -> Self {
        Self::new(
            self.x.min(with),
            self.y.min(with),
            self.z.min(with),
            self.w.min(with),
        )
    }

    /// Returns a new vector with each component set to `1` if it's positive, `-1` if it's negative, and `0` if it's zero. The result is identical to calling `sign` on each component.
    pub fn sign(&self) -> Self {
        Self::new(
            self.x.signum(),
            self.y.signum(),
            self.z.signum(),
            self.w.signum(),
        )
    }

    /// Returns a new vector with each component snapped to the closest multiple of the corresponding component in `step`.
    pub fn snapped(&self, step: &Self) -> Self {
        Self::new(
            snapped_i(self.x, step.x),
            snapped_i(self.y, step.y),
            snapped_i(self.z, step.z),
            snapped_i(self.w, step.w),
        )
    }

    /// Returns a new vector with each component snapped to the closest multiple of `step`.
    pub fn snapped_i(&self, step: int!()) -> Self {
        Self::new(
            snapped_i(self.x, step),
            snapped_i(self.y, step),
            snapped_i(self.z, step),
            snapped_i(self.w, step),
        )
    }

    pub const fn get(&self, index: usize) -> int!() {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, index: usize, value: int!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> int!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            AXIS::Z => self.z,
            AXIS::W => self.w,
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: int!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            AXIS::Z => self.z = value,
            AXIS::W => self.w = value,
        }
    }
}

impl From<Vector4> for Vector4i {
    /// Constructs a new **Vector4i** from the given [`Vector4`] by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of [`Vector4::ceil`], [`Vector4::floor`] or [Vector4::round`] to this constructor instead.
    fn from(value: Vector4) -> Self {
        Self::new(
            value.x.trunc() as int!(),
            value.y.trunc() as int!(),
            value.z.trunc() as int!(),
            value.w.trunc() as int!(),
        )
    }
}

impl PartialEq for Vector4i {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Eq for Vector4i {}

impl_op_ex!(% |a: &Vector4i, b: &Vector4i| -> Vector4i {
    Vector4i::new(
        a.x % b.x,
        a.y % b.y,
        a.z % b.z,
        a.w % b.w,
    )
});

impl_op_ex!(%= |a: &mut Vector4i, b: &Vector4i| {
    a.x = a.x % b.x;
    a.y = a.y % b.y;
    a.z = a.z % b.z;
    a.w = a.w % b.w;
});

impl_op_ex!(% |a: &Vector4i, b: &int!()| -> Vector4i {
    Vector4i::new(
        a.x % b,
        a.y % b,
        a.z % b,
        a.w % b,
    )
});

impl_op_ex!(%= |a: &mut Vector4i, b: &int!()| {
    a.x = a.x % b;
    a.y = a.y % b;
    a.z = a.z % b;
    a.w = a.w % b;
});

impl_op_ex!(*|a: &Vector4i, b: &Vector4i| -> Vector4i {
    Vector4i::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w)
});

impl_op_ex!(*= |a: &mut Vector4i, b: &Vector4i| {
    a.x = a.x * b.x;
    a.y = a.y * b.y;
    a.z = a.z * b.z;
    a.w = a.w * b.w;
});

impl_op_ex_commutative!(*|a: &Vector4i, b: &float!()| -> Vector4 {
    Vector4::new(
        a.x as float!() * b,
        a.y as float!() * b,
        a.z as float!() * b,
        a.w as float!() * b,
    )
});

impl_op_ex_commutative!(*|a: &Vector4i, b: &int!()| -> Vector4i {
    Vector4i::new(a.x * b, a.y * b, a.z * b, a.w * b)
});

impl_op_ex!(*= |a: &mut Vector4i, b: &int!()| {
    a.x = a.x * b;
    a.y = a.y * b;
    a.z = a.z * b;
    a.w = a.w * b;
});

impl_op_ex!(+ |a: &Vector4i, b: &Vector4i| -> Vector4i {
    Vector4i::new(
        a.x + b.x,
        a.y + b.y,
        a.z + b.z,
        a.w + b.w,
    )
});

impl_op_ex!(+= |a: &mut Vector4i, b: &Vector4i| {
    a.x = a.x + b.x;
    a.y = a.y + b.y;
    a.z = a.z + b.z;
    a.w = a.w + b.w;
});

impl_op_ex!(-|a: &Vector4i, b: &Vector4i| -> Vector4i {
    Vector4i::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w)
});

impl_op_ex!(-= |a: &mut Vector4i, b: &Vector4i| {
    a.x = a.x - b.x;
    a.y = a.y - b.y;
    a.z = a.z - b.z;
    a.w = a.w - b.w;
});

impl_op_ex!(/ |a: &Vector4i, b: &Vector4i| -> Vector4i {
    Vector4i::new(
        a.x / b.x,
        a.y / b.y,
        a.z / b.z,
        a.w / b.w,
    )
});

impl_op_ex!(/= |a: &mut Vector4i, b: &Vector4i| {
    a.x = a.x / b.x;
    a.y = a.y / b.y;
    a.z = a.z / b.z;
    a.w = a.w / b.w;
});

impl_op_ex!(/ |a: &Vector4i, b: &float!()| -> Vector4 {
    Vector4::new(
        a.x as float!() / b,
        a.y as float!() / b,
        a.z as float!() / b,
        a.w as float!() / b,
    )
});

impl_op_ex!(/ |a: &Vector4i, b: &int!()| -> Vector4i {
    Vector4i::new(
        a.x / b,
        a.y / b,
        a.z / b,
        a.w / b,
    )
});

impl_op_ex!(/= |a: &mut Vector4i, b: &int!()| {
    a.x = a.x / b;
    a.y = a.y / b;
    a.z = a.z / b;
    a.w = a.w / b;
});

impl Not for Vector4i {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0
    }
}

impl PartialOrd for Vector4i {
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

impl Neg for Vector4i {
    type Output = Vector4i;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl Display for Vector4i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Vector4i({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}
