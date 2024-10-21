use crate::float;
use crate::types::vectors::{Vector2, Vector2i, Vector3, AXIS};
use crate::utils::{int, snapped_i};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Neg, Not};

/// A 3D vector using integer coordinates.
///
/// A 3-element structure that can be used to represent 3D grid coordinates or any other triplet of integers.
///
/// It uses integer coordinates and is therefore preferable to [`Vector3`] when exact precision is required.
///
/// **Note:** In a boolean context, a Vector3i will evaluate to `false` if it's equal to `Vector3i(0, 0, 0)`. Otherwise, a Vector3i will always evaluate to `true`.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vector3i {
    /// The vector's X component. Also, accessible by using the index position `v.get(0)`.
    pub x: int!(),
    /// The vector's Y component. Also, accessible by using the index position `v.get(1)`.
    pub y: int!(),
    /// The vector's Z component. Also, accessible by using the index position `v.get(2)`.
    pub z: int!(),
}

impl Vector3i {
    /// Zero vector, a vector with all components set to `0`.
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// One vector, a vector with all components set to `1`.
    pub const ONE: Self = Self::new(1, 1, 1);

    /// Min vector, a vector with all components equal to `i32::MIN`. Can be used as a negative integer equivalent of [`Vector3::INF`].
    pub const MIN: Self = Self::new(<int!()>::MIN, <int!()>::MIN, <int!()>::MIN);

    /// Max vector, a vector with all components equal to [`i32::MAX`]. Can be used as an integer equivalent of [`Vector3::INF`].
    pub const MAX: Self = Self::new(<int!()>::MAX, <int!()>::MAX, <int!()>::MAX);

    /// Left unit vector. Represents the local direction of left, and the global direction of west.
    pub const LEFT: Self = Self::new(-1, 0, 0);

    /// Right unit vector. Represents the local direction of right, and the global direction of east.
    pub const RIGHT: Self = Self::new(1, 0, 0);

    /// Up unit vector.
    pub const UP: Self = Self::new(0, 1, 0);

    /// Down unit vector.
    pub const DOWN: Self = Self::new(0, -1, 0);

    /// Forward unit vector. Represents the local direction of forward, and the global direction of north.
    pub const FORWARD: Self = Self::new(0, 0, -1);

    /// Back unit vector. Represents the local direction of back, and the global direction of south.
    pub const BACK: Self = Self::new(0, 0, 1);

    /// Returns a **Vector3i** with the given components.
    pub const fn new(x: int!(), y: int!(), z: int!()) -> Self {
        Self { x, y, z }
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub const fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Returns a new vector with all components clamped between the components of `min` and `max`, by running `clamp` on each component.
    pub fn clamp(&self, min: &Vector3i, max: &Vector3i) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    /// Returns a new vector with all components clamped between `min` and `max`, by running `clamp` on each component.
    pub fn clamp_i(&self, min: int!(), max: int!()) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    /// Returns the squared distance between this vector and `to`.
    ///
    /// This method runs faster than [`Vector3i::distance_to`], so prefer it if you need to compare vectors or need the squared distance for some formula.
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
    /// This method runs faster than [`Vector3i::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub const fn length_squared(&self) -> int!() {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector3i::new(x.max(with.x), y.max(with.y), z.max(with.z))`.
    pub fn max(&self, with: &Self) -> Self {
        Self::new(self.x.max(with.x), self.y.max(with.y), self.z.max(with.z))
    }

    /// Returns the axis of the vector's highest value. If all components are equal, this method returns [`AXIS::X`].
    pub const fn max_axis_index(&self) -> AXIS {
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

    /// Returns the component-wise maximum of this and `with`, equivalent to `Vector3i::new(x.max(with), y.max(with), z.max(with))`.
    pub fn max_i(&self, with: int!()) -> Self {
        Self::new(self.x.max(with), self.y.max(with), self.z.max(with))
    }

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector3i::new(x.min(with.x), y.min(with.y), z.min(with.z))`.
    pub fn min(&self, with: &Self) -> Self {
        Self::new(self.x.min(with.x), self.y.min(with.y), self.z.min(with.z))
    }

    /// Returns the axis of the vector's lowest value. If all components are equal, this method returns [`AXIS::Z`].
    pub const fn min_axis_index(&self) -> AXIS {
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

    /// Returns the component-wise minimum of this and `with`, equivalent to `Vector3i::new(x.min(with), y.min(with), z.min(with))`.
    pub fn min_i(&self, with: int!()) -> Self {
        Self::new(self.x.min(with), self.y.min(with), self.z.min(with))
    }

    /// Returns a new vector with each component set to `1` if it's positive, `-1` if it's negative, and `0` if it's zero. The result is identical to calling `sign` on each component.
    pub fn sign(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
    }

    /// Returns a new vector with each component snapped to the closest multiple of the corresponding component in `step`.
    pub fn snapped(&self, step: &Self) -> Self {
        Self::new(
            snapped_i(self.x, step.x),
            snapped_i(self.y, step.y),
            snapped_i(self.z, step.z),
        )
    }

    /// Returns a new vector with each component snapped to the closest multiple of `step`.
    pub fn snapped_i(&self, step: int!()) -> Self {
        Self::new(
            snapped_i(self.x, step),
            snapped_i(self.y, step),
            snapped_i(self.z, step),
        )
    }

    pub const fn get(&self, index: usize) -> int!() {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid index"),
        }
    }
    pub fn set(&mut self, index: usize, value: int!()) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_axis(&self, axis: AXIS) -> int!() {
        match axis {
            AXIS::X => self.x,
            AXIS::Y => self.y,
            AXIS::Z => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, axis: AXIS, value: int!()) {
        match axis {
            AXIS::X => self.x = value,
            AXIS::Y => self.y = value,
            AXIS::Z => self.z = value,
            _ => panic!("Invalid axis"),
        }
    }
}

impl From<Vector3> for Vector3i {
    fn from(vector: Vector3) -> Self {
        Self {
            x: vector.x.trunc() as int!(),
            y: vector.y.trunc() as int!(),
            z: vector.z.trunc() as int!(),
        }
    }
}

impl PartialEq for Vector3i {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3i {}

impl Display for Vector3i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Vector3i({},{},{})", self.x, self.y, self.z))
    }
}

impl_op_ex!(% |a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(
        a.x % b.x,
        a.y % b.y,
        a.z % b.z,
    )
});

impl_op_ex!(% |a: &Vector3i, b: &int!()| -> Vector3i {
    Vector3i::new(
        a.x % b,
        a.y % b,
        a.z % b,
    )
});

impl_op_ex!(*|a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(a.x * b.x, a.y * b.y, a.z * b.z)
});

impl_op_ex_commutative!(*|a: &Vector3i, b: &float!()| -> Vector3 {
    Vector3::new(
        a.x as float!() * b,
        a.y as float!() * b,
        a.z as float!() * b,
    )
});

impl_op_ex_commutative!(*|a: &Vector3i, b: &int!()| -> Vector3i {
    Vector3i::new(a.x * b, a.y * b, a.z * b)
});

impl_op_ex!(+ |a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(
        a.x + b.x,
        a.y + b.y,
        a.z + b.z,
    )
});

impl_op_ex!(-|a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex!(/ |a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(
        a.x / b.x,
        a.y / b.y,
        a.z / b.z,
    )
});

impl_op_ex!(/ |a: &Vector3i, b: &float!()| -> Vector3 {
    Vector3::new(
        a.x as float!() / b,
        a.y as float!() / b,
        a.z as float!() / b,
    )
});

impl_op_ex!(/ |a: &Vector3i, b: &int!()| -> Vector3i {
    Vector3i::new(
        a.x / b,
        a.y / b,
        a.z / b,
    )
});

impl PartialOrd for Vector3i {
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

impl Neg for Vector3i {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl From<&Vector3> for Vector3i {
    /// Constructs a new **Vector3i** from the given [`Vector3`] by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of [`Vector3::ceil`], [`Vector3::floor`] or [`Vector3::round`] to this constructor instead.
    fn from(value: &Vector3) -> Self {
        Self {
            x: value.x.trunc() as int!(),
            y: value.y.trunc() as int!(),
            z: value.z.trunc() as int!(),
        }
    }
}

impl Not for Vector3i {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.x == 0 && self.y == 0 && self.z == 0
    }
}
