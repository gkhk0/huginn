#[cfg(feature = "double-precision-float")]
#[macro_export]
macro_rules! float {
    () => {
        f64
    };
}
#[cfg(not(feature = "double-precision-float"))]
#[macro_export]
macro_rules! float {
    () => {
        f32
    };
}

#[cfg(feature = "double-precision-int")]
#[macro_export]
macro_rules! int {
    () => {
        i64
    };
}

#[cfg(not(feature = "double-precision-int"))]
#[macro_export]
macro_rules! int {
    () => {
        i32
    };
}

pub use float;
pub use int;

pub const CMP_EPSILON: float!() = 0.00001;
pub const CMP_EPSILON2: float!() = CMP_EPSILON * CMP_EPSILON;
pub const UNIT_EPSILON: float!() = 0.00001;

pub fn bezier_derivative(
    start: float!(),
    control_1: float!(),
    control_2: float!(),
    end: float!(),
    t: float!(),
) -> float!() {
    let omt = 1.0 - t;
    let omt2 = omt * omt;
    let t2 = t * t;

    (control_1 - start) * 3.0 * omt2
        + (control_2 - control_1) * 6.0 * omt * t
        + (end - control_2) * 3.0 * t2
}

pub fn bezier_interpolate(
    start: float!(),
    control_1: float!(),
    control_2: float!(),
    end: float!(),
    t: float!(),
) -> float!() {
    let omt = 1.0 - t;
    let omt2 = omt * omt;
    let omt3 = omt2 * omt;
    let t2 = t * t;
    let t3 = t2 * t;

    start * omt3 + control_1 * omt2 * t * 3.0 + control_2 * omt * t2 * 3.0 + end * t3
}

pub fn cubic_interpolate(
    from: float!(),
    to: float!(),
    pre: float!(),
    post: float!(),
    weight: float!(),
) -> float!() {
    0.5 * ((from * 2.0)
        + (-pre + to) * weight
        + (2.0 * pre - 5.0 * from + 4.0 * to - post) * (weight * weight)
        + (-pre + 3.0 * from - 3.0 * to + post) * (weight * weight * weight))
}

pub fn cubic_interpolate_in_time(
    from: float!(),
    to: float!(),
    pre: float!(),
    post: float!(),
    weight: float!(),
    to_t: float!(),
    pre_t: float!(),
    post_t: float!(),
) -> float!() {
    /* Barry-Goldman method */
    let t = 0.0.lerp(to_t, weight);
    let a1 = pre.lerp(
        from,
        if pre_t == 0.0 {
            0.0
        } else {
            (t - pre_t) / -pre_t
        },
    );
    let a2 = from.lerp(to, if to_t == 0.0 { 0.5 } else { t / to_t });
    let a3 = to.lerp(
        post,
        if post_t - to_t == 0.0 {
            1.0
        } else {
            (t - to_t) / (post_t - to_t)
        },
    );
    let b1 = a1.lerp(
        a2,
        if to_t - pre_t == 0.0 {
            0.0
        } else {
            (t - pre_t) / (to_t - pre_t)
        },
    );
    let b2 = a2.lerp(a3, if post_t == 0.0 { 1.0 } else { t / post_t });
    b1.lerp(b2, if to_t == 0.0 { 0.5 } else { t / to_t })
}

pub fn is_equal_approx(a: float!(), b: float!()) -> bool {
    // Check for exact equality first, required to handle "infinity" values.
    if a == b {
        return true;
    }
    // Then check for approximate equality.
    let mut tolerance = CMP_EPSILON * a.abs();
    if tolerance < CMP_EPSILON {
        tolerance = CMP_EPSILON;
    }
    (a - b).abs() < tolerance
}

pub fn is_equal_approx_with_tolerance(a: float!(), b: float!(), tolerance: float!()) -> bool {
    // Check for exact equality first, required to handle "infinity" values.
    if a == b {
        return true;
    }

    (a - b).abs() < tolerance
}

pub fn is_zero_approx(s: float!()) -> bool {
    s.abs() < CMP_EPSILON
}

pub const fn posmod_f(x: float!(), y: float!()) -> float!() {
    let mut value = x % y;
    if ((value < 0.0) && (y > 0.0)) || ((value > 0.0) && (y < 0.0)) {
        value += y;
    }
    value += 0.0;
    value
}

pub fn snapped(value: float!(), step: float!()) -> float!() {
    if step != 0.0 {
        (value / step + 0.5).floor() * step
    } else {
        value
    }
}

pub fn snapped_i(value: int!(), step: int!()) -> int!() {
    if step != 0 {
        ((value as float!() / step as float!() + 0.5).floor() * step as float!()) as int!()
    } else {
        value
    }
}

#[cfg(not(feature = "double-precision-float"))]
pub use std::f32::consts as float_consts;
#[cfg(feature = "double-precision-float")]
pub use std::f64::consts as float_consts;

pub trait FloatExt {
    fn lerp(self, rhs: Self, t: Self) -> Self;

    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self;

    fn remap(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self;

    fn sign(self) -> Self;

    fn safe_acos(self) -> Self;
    fn safe_asin(self) -> Self;
}

impl FloatExt for float!() {
    fn lerp(self, rhs: Self, t: Self) -> Self {
        self + (rhs - self) * t
    }

    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self {
        (v - a) / (b - a)
    }

    fn remap(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self {
        let t = <float!()>::inverse_lerp(in_start, in_end, self);
        out_start.lerp(out_end, t)
    }

    fn sign(self) -> Self {
        if self == 0.0 {
            0.0
        } else if self > 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    fn safe_acos(self) -> Self {
        if self < -1.0 {
            float_consts::PI
        } else if self > 1.0 {
            0.0
        } else {
            self.acos()
        }
    }
    fn safe_asin(self) -> Self {
        if self < -1.0 {
            -float_consts::PI / 2.0
        } else if self > 1.0 {
            float_consts::PI / 2.0
        } else {
            self.asin()
        }
    }
}
