use huginn::float;
use huginn::types::vectors::{Vector4, AXIS};
use huginn::utils::float_consts;
use huginn::utils::CMP_EPSILON;

const SQRT_3: float!() = 1.732050807568877293527446341505872367;

macro_rules! assert_approx_eq {
    ($x:expr, $y:expr) => {
        assert!(($x - $y).abs() < CMP_EPSILON);
    };
    ($x:expr, $y:expr, $msg:expr) => {
        assert!(($x - $y).abs() < CMP_EPSILON, $msg);
    };
}
macro_rules! assert_approx_eq_with_tolerance {
    ($x:expr, $y:expr, $z:expr) => {
        assert!(($x - $y).abs() < $z);
    };
    ($x:expr, $y:expr, $z:expr, $msg:expr) => {
        assert!(($x - $y).abs() < $z, $msg);
    };
}

#[test]
fn constructor_methods() {
    let vector_empty = Vector4::default();
    let vector_zero = Vector4::new(0.0, 0.0, 0.0, 0.0);

    assert_eq!(
        vector_empty, vector_zero,
        "Constructor with no inputs should return a zero Vector4."
    );
}

#[test]
fn axis_methods() {
    let mut vector = Vector4::new(1.2, 3.4, 5.6, -0.9);

    assert_eq!(
        vector.max_axis_index(),
        AXIS::Z,
        "max_axis_index should work as expected."
    );
    assert_eq!(
        vector.min_axis_index(),
        AXIS::W,
        "min_axis_index should work as expected."
    );
    assert_eq!(
        vector.get_axis(vector.max_axis_index()),
        5.6,
        "array operator should work as expected."
    );
    assert_eq!(
        vector.get_axis(vector.min_axis_index()),
        -0.9,
        "array operator should work as expected."
    );
    vector.set_axis(AXIS::Y, 3.7);
    assert_eq!(
        vector.get_axis(AXIS::Y),
        3.7,
        "array operator setter should work as expected."
    );
}

#[test]
fn interpolation_methods() {
    let vector1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let vector2 = Vector4::new(4.0, 5.0, 6.0, 7.0);

    assert_eq!(
        vector1.lerp(&vector2, 0.5),
        Vector4::new(2.5, 3.5, 4.5, 5.5),
        "lerp should work as expected."
    );
    assert!(
        vector1
            .lerp(&vector2, 1.0 / 3.0)
            .is_equal_approx(&Vector4::new(2.0, 3.0, 4.0, 5.0)),
        "lerp should work as expected."
    );
    assert_eq!(
        vector1.cubic_interpolate(
            &vector2,
            &Vector4::default(),
            &Vector4::new(7.0, 7.0, 7.0, 7.0),
            0.5
        ),
        Vector4::new(2.375, 3.5, 4.625, 5.75),
        "cubic_interpolate should work as expected."
    );
    assert!(
        vector1
            .cubic_interpolate(
                &vector2,
                &Vector4::default(),
                &Vector4::new(7.0, 7.0, 7.0, 7.0),
                1.0 / 3.0
            )
            .is_equal_approx(&Vector4::new(
                1.851851940155029297,
                2.962963104248046875,
                4.074074268341064453,
                5.185185185185
            )),
        "cubic_interpolate should work as expected."
    );
}

#[test]
fn length_methods() {
    let vector1 = Vector4::new(10.0, 10.0, 10.0, 10.0);
    let vector2 = Vector4::new(20.0, 30.0, 40.0, 50.0);

    assert_eq!(
        vector1.length_squared(),
        400.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(vector1.length(), 20.0, "length should work as expected.");
    assert_eq!(
        vector2.length_squared(),
        5400.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector2.length(),
        73.484692283495,
        "length should work as expected."
    );
    assert_approx_eq!(
        vector1.distance_to(&vector2),
        54.772255750517,
        "distance_to should work as expected."
    );
    assert_approx_eq!(
        vector1.distance_squared_to(&vector2),
        3000.0,
        "distance_squared_to should work as expected."
    );
}

#[test]
fn limiting_methods() {
    let vector = Vector4::new(10.0, 10.0, 10.0, 10.0);

    assert_eq!(
        Vector4::new(-5.0, 5.0, 15.0, -15.0).clamp(&Vector4::default(), &vector),
        Vector4::new(0.0, 5.0, 10.0, 0.0),
        "clamp should work as expected."
    );
    assert_eq!(
        vector.clamp(
            &Vector4::new(0.0, 10.0, 15.0, 18.0),
            &Vector4::new(5.0, 10.0, 20.0, 25.0)
        ),
        Vector4::new(5.0, 10.0, 15.0, 18.0),
        "clamp should work as expected."
    );
}

#[test]
fn normalization_methods() {
    assert!(
        Vector4::new(1.0, 0.0, 0.0, 0.0).is_normalized(),
        "is_normalized should return true for a normalized vector."
    );
    assert!(
        !Vector4::new(1.0, 1.0, 1.0, 1.0).is_normalized(),
        "is_normalized should return false for a non-normalized vector."
    );
    assert_eq!(
        Vector4::new(1.0, 0.0, 0.0, 0.0).normalized(),
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        "normalized should return the same vector for a normalized vector."
    );
    assert!(
        Vector4::new(1.0, 1.0, 0.0, 0.0)
            .normalized()
            .is_equal_approx(&Vector4::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0,
                0.0
            )),
        "normalized should work as expected."
    );
    assert!(
        Vector4::new(1.0, 1.0, 1.0, 1.0)
            .normalized()
            .is_equal_approx(&Vector4::new(0.5, 0.5, 0.5, 0.5)),
        "normalized should work as expected."
    );
}

#[test]
fn operators() {
    let decimal1 = Vector4::new(2.3, 4.9, 7.8, 3.2);
    let decimal2 = Vector4::new(1.2, 3.4, 5.6, 1.7);
    let power1 = Vector4::new(0.75, 1.5, 0.625, 0.125);
    let power2 = Vector4::new(0.5, 0.125, 0.25, 0.75);
    let int1 = Vector4::new(4.0, 5.0, 9.0, 2.0);
    let int2 = Vector4::new(1.0, 2.0, 3.0, 1.0);

    assert_eq!(
        -decimal1,
        Vector4::new(-2.3, -4.9, -7.8, -3.2),
        "change of sign should work as expected."
    );
    assert!(
        (decimal1 + decimal2).is_equal_approx(&Vector4::new(3.5, 8.3, 13.4, 4.9)),
        "addition should behave as expected."
    );
    assert_eq!(
        power1 + power2,
        Vector4::new(1.25, 1.625, 0.875, 0.875),
        "addition with powers of two should give exact results."
    );
    assert_eq!(
        int1 + int2,
        Vector4::new(5.0, 7.0, 12.0, 3.0),
        "addition with integers should give exact results."
    );
    assert!(
        (decimal1 - decimal2).is_equal_approx(&Vector4::new(1.1, 1.5, 2.2, 1.5)),
        "subtraction should behave as expected."
    );
    assert_eq!(
        power1 - power2,
        Vector4::new(0.25, 1.375, 0.375, -0.625),
        "subtraction with powers of two should give exact results."
    );
    assert_eq!(
        int1 - int2,
        Vector4::new(3.0, 3.0, 6.0, 1.0),
        "subtraction with integers should give exact results."
    );
    assert!(
        (decimal1 * decimal2).is_equal_approx(&Vector4::new(2.76, 16.66, 43.68, 5.44)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * power2,
        Vector4::new(0.375, 0.1875, 0.15625, 0.09375),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * int2,
        Vector4::new(4.0, 10.0, 27.0, 2.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / decimal2).is_equal_approx(&Vector4::new(
            1.91666666666666666,
            1.44117647058823529,
            1.39285714285714286,
            1.88235294118
        )),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / power2,
        Vector4::new(1.5, 12.0, 2.5, 1.0 / 6.0),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / int2,
        Vector4::new(4.0, 2.5, 3.0, 2.0),
        "division with integers should give exact results."
    );
    assert!(
        (decimal1 * 2.0).is_equal_approx(&Vector4::new(4.6, 9.8, 15.6, 6.4)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * 2.0,
        Vector4::new(1.5, 3.0, 1.25, 0.25),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * 2.0,
        Vector4::new(8.0, 10.0, 18.0, 4.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / 2.0).is_equal_approx(&Vector4::new(1.15, 2.45, 3.9, 1.6)),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / 2.0,
        Vector4::new(0.375, 0.75, 0.3125, 0.0625),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / 2.0,
        Vector4::new(2.0, 2.5, 4.5, 1.0),
        "division with integers should give exact results."
    );
    assert_eq!(
        decimal1.to_string(),
        "Vector4(2.3, 4.9, 7.8, 3.2)",
        "cast to String should work as expected."
    );
    assert_eq!(
        decimal2.to_string(),
        "Vector4(1.2, 3.4, 5.6, 1.7)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector4::new(9.7, 9.8, 9.9, -1.8).to_string(),
        "Vector4(9.7, 9.8, 9.9, -1.8)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector4::new(float_consts::E, float_consts::SQRT_2, SQRT_3, SQRT_3).to_string(),
        format!(
            "Vector4({}, {}, {}, {})",
            float_consts::E,
            float_consts::SQRT_2,
            SQRT_3,
            SQRT_3
        ),
        "cast to String should print the correct amount of digits for real_t = let."
    );
}

#[test]
fn other_methods() {
    let vector = Vector4::new(1.2, 3.4, 5.6, 1.6);

    assert!(
        vector
            .direction_to(&Vector4::default())
            .is_equal_approx(&-vector.normalized()),
        "direction_to should work as expected."
    );
    assert!(
        Vector4::new(1.0, 1.0, 1.0, 1.0)
            .direction_to(&Vector4::new(2.0, 2.0, 2.0, 2.0))
            .is_equal_approx(&Vector4::new(0.5, 0.5, 0.5, 0.5)),
        "direction_to should work as expected."
    );
    assert!(
        vector
            .inverse()
            .is_equal_approx(&Vector4::new(1.0 / 1.2, 1.0 / 3.4, 1.0 / 5.6, 1.0 / 1.6)),
        "inverse should work as expected."
    );
    assert!(
        vector
            .posmod(2.0)
            .is_equal_approx(&Vector4::new(1.2, 1.4, 1.6, 1.6)),
        "posmod should work as expected."
    );
    assert!(
        (-vector)
            .posmod(2.0)
            .is_equal_approx(&Vector4::new(0.8, 0.6, 0.4, 0.4)),
        "posmod should work as expected."
    );
    assert!(
        vector
            .posmod_v(&Vector4::new(1.0, 2.0, 3.0, 4.0))
            .is_equal_approx(&Vector4::new(0.2, 1.4, 2.6, 1.6)),
        "posmodv should work as expected."
    );
    assert!(
        (-vector)
            .posmod_v(&Vector4::new(2.0, 3.0, 4.0, 5.0))
            .is_equal_approx(&Vector4::new(0.8, 2.6, 2.4, 3.4)),
        "posmodv should work as expected."
    );
    assert_eq!(
        vector.snapped(&Vector4::new(1.0, 1.0, 1.0, 1.0)),
        Vector4::new(1.0, 3.0, 6.0, 2.0),
        "snapped to integers should be the same as rounding."
    );
    assert_eq!(
        vector.snapped(&Vector4::new(0.25, 0.25, 0.25, 0.25)),
        Vector4::new(1.25, 3.5, 5.5, 1.5),
        "snapped to 0.25 should give exact results."
    );
    assert!(
        Vector4::new(1.2, 2.5, 2.0, 1.6)
            .is_equal_approx(&vector.min(&Vector4::new(3.0, 2.5, 2.0, 3.4))),
        "min should return expected value."
    );
    assert!(
        Vector4::new(5.3, 3.4, 5.6, 4.2)
            .is_equal_approx(&vector.max(&Vector4::new(5.3, 2.0, 3.0, 4.2))),
        "max should return expected value."
    );
}

#[test]
fn rounding_methods() {
    let vector1 = Vector4::new(1.2, 3.4, 5.6, 1.6);
    let vector2 = Vector4::new(1.2, -3.4, -5.6, -1.6);

    assert_eq!(vector1.abs(), vector1, "abs should work as expected.");
    assert_eq!(vector2.abs(), vector1, "abs should work as expected.");
    assert_eq!(
        vector1.ceil(),
        Vector4::new(2.0, 4.0, 6.0, 2.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector2.ceil(),
        Vector4::new(2.0, -3.0, -5.0, -1.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector1.floor(),
        Vector4::new(1.0, 3.0, 5.0, 1.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector2.floor(),
        Vector4::new(1.0, -4.0, -6.0, -2.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector1.round(),
        Vector4::new(1.0, 3.0, 6.0, 2.0),
        "round should work as expected."
    );
    assert_eq!(
        vector2.round(),
        Vector4::new(1.0, -3.0, -6.0, -2.0),
        "round should work as expected."
    );
    assert_eq!(
        vector1.sign(),
        Vector4::new(1.0, 1.0, 1.0, 1.0),
        "sign should work as expected."
    );
    assert_eq!(
        vector2.sign(),
        Vector4::new(1.0, -1.0, -1.0, -1.0),
        "sign should work as expected."
    );
}

#[test]
fn linear_algebra_methods() {
    let vector_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let vector_y = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let vector1 = Vector4::new(1.7, 2.3, 1.0, 9.1);
    let vector2 = Vector4::new(-8.2, -16.0, 3.0, 2.4);

    assert_eq!(
        vector_x.dot(&vector_y),
        0.0,
        "dot product of perpendicular vectors should be zero."
    );
    assert_eq!(
        vector_x.dot(&vector_x),
        1.0,
        "dot product of identical unit vectors should be one."
    );
    assert_eq!(
        (&vector_x * 10.0).dot(&(vector_x * 10.0)),
        100.0,
        "dot product of same direction vectors should behave as expected."
    );
    println!(
        "{}",
        ((vector1 * 2.0).dot(&(vector2 * 4.0)) + 25.9 * 8.0).abs()
    );
    assert_approx_eq_with_tolerance!(
        (vector1 * 2.0).dot(&(vector2 * 4.0)),
        -25.9 * 8.0,
        0.000031,
        "dot product should work as expected."
    );
}

#[test]
fn finite_number_checks() {
    let infinite = vec![
        <float!()>::NAN,
        <float!()>::INFINITY,
        <float!()>::NEG_INFINITY,
    ];

    assert!(
        Vector4::new(0.0, 1.0, 2.0, 3.0).is_finite(),
        "Vector4(0, 1, 2) should be finite"
    );

    for x in &infinite {
        assert!(
            !Vector4::new(*x, 1.0, 2.0, 3.0).is_finite(),
            "Vector4 with one component infinite should not be finite. Failed with: {}",
            x
        );
        assert!(
            !Vector4::new(0.0, *x, 2.0, 3.0).is_finite(),
            "Vector4 with one component infinite should not be finite. Failed with: {}",
            x
        );
        assert!(
            !Vector4::new(0.0, 1.0, *x, 3.0).is_finite(),
            "Vector4 with one component infinite should not be finite. Failed with: {}",
            x
        );
        assert!(
            !Vector4::new(0.0, 1.0, 2.0, *x).is_finite(),
            "Vector4 with one component infinite should not be finite. Failed with: {}",
            x
        );
    }

    for x in &infinite {
        for y in &infinite {
            assert!(
                !Vector4::new(*x, *y, 2.0, 3.0).is_finite(),
                "Vector4 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector4::new(*x, 1.0, *y, 3.0).is_finite(),
                "Vector4 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector4::new(*x, 1.0, 2.0, *y).is_finite(),
                "Vector4 with one component infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector4::new(0.0, *x, *y, 3.0).is_finite(),
                "Vector4 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector4::new(0.0, *x, 2.0, *y).is_finite(),
                "Vector4 with one component infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector4::new(0.0, 1.0, *x, *y).is_finite(),
                "Vector4 with one component infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
        }
    }

    for x in &infinite {
        for y in &infinite {
            for z in &infinite {
                assert!(!
                            Vector4::new(*x, *y, *z, 3.0).is_finite(),
                        "Vector4 with three components infinite should not be finite. Failed with: {}, {}, {}", x, y, z);
                assert!(
                    !Vector4::new(*x, *y, 2.0, *z).is_finite(),
                    "Vector4 with one component infinite should not be finite. Failed with: {}, {}, {}",
                    x, y, z
                );
                assert!(
                    !Vector4::new(*x, 1.0, *y, *z).is_finite(),
                    "Vector4 with one component infinite should not be finite. Failed with: {}, {}, {}",
                    x, y, z
                );
                assert!(
                    !Vector4::new(0.0, *x, *y, *z).is_finite(),
                    "Vector4 with one component infinite should not be finite. Failed with: {}, {}, {}",
                    x, y, z
                );
            }
        }
    }

    for x in &infinite {
        for y in &infinite {
            for z in &infinite {
                for w in &infinite {
                    assert!(
                        !Vector4::new(*x, *y, *z, *w).is_finite(),
                        "Vector4 with one component infinite should not be finite. Failed with: {}, {}, {}, {}",
                        x, y, z, w
                    );
                }
            }
        }
    }
}
