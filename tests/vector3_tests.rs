#![feature(more_float_constants)]

use huginn::float;
use huginn::types::vectors::{Vector3, Vector3i, AXIS};
use huginn::utils::float_consts::TAU;
use huginn::utils::{float_consts, CMP_EPSILON};

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
    let vector_empty = Vector3::default();
    let vector_zero = Vector3::new(0.0, 0.0, 0.0);

    assert_eq!(
        vector_empty, vector_zero,
        "Constructor with no inputs should return a zero Vector3."
    );
}

#[test]
fn angle_methods() {
    let vector_x = Vector3::new(1.0, 0.0, 0.0);
    let vector_y = Vector3::new(0.0, 1.0, 0.0);
    let vector_yz = Vector3::new(0.0, 1.0, 1.0);

    assert_approx_eq!(
        vector_x.angle_to(&vector_y),
        TAU / 4.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_x.angle_to(&vector_yz),
        TAU / 4.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_yz.angle_to(&vector_x),
        TAU / 4.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_y.angle_to(&vector_yz),
        TAU / 8.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_x.signed_angle_to(&vector_y, &vector_y),
        TAU / 4.0,
        "signed_angle_to edge case should be positive."
    );
    assert_approx_eq!(
        vector_x.signed_angle_to(&vector_yz, &vector_y),
        TAU / -4.0,
        "signed_angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_yz.signed_angle_to(&vector_x, &vector_y),
        TAU / 4.0,
        "signed_angle_to should work as expected."
    );
}

#[test]
fn axis_methods() {
    let mut vector = Vector3::new(1.2, 3.4, 5.6);

    assert_eq!(
        vector.max_axis_index(),
        AXIS::Z,
        "max_axis_index should work as expected."
    );
    assert_eq!(
        vector.min_axis_index(),
        AXIS::X,
        "min_axis_index should work as expected."
    );
    assert_eq!(
        vector.get_axis(vector.max_axis_index()),
        5.6,
        "array operator should work as expected."
    );
    assert_eq!(
        vector.get_axis(vector.min_axis_index()),
        1.2,
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
    let vector1 = Vector3::new(1.0, 2.0, 3.0);
    let vector2 = Vector3::new(4.0, 5.0, 6.0);

    assert_eq!(
        vector1.lerp(&vector2, 0.5),
        Vector3::new(2.5, 3.5, 4.5),
        "lerp should work as expected."
    );
    assert!(
        vector1
            .lerp(&vector2, 1.0 / 3.0)
            .is_equal_approx(&Vector3::new(2.0, 3.0, 4.0)),
        "lerp should work as expected."
    );
    assert!(
        vector1
            .normalized()
            .slerp(&vector2.normalized(), 0.5)
            .is_equal_approx(&Vector3::new(
                0.3638667954886597,
                0.5556981892812023,
                0.747529583073745
            )),
        "slerp should work as expected."
    );
    assert!(
        vector1
            .normalized()
            .slerp(&vector2.normalized(), 1.0 / 3.0)
            .is_equal_approx(&Vector3::new(
                0.3321197540663702,
                0.5494138058363556,
                0.7667078576063411
            )),
        "slerp should work as expected."
    );
    assert!(
        Vector3::new(5.0, 0.0, 0.0)
            .slerp(&Vector3::new(0.0, 3.0, 4.0), 0.5)
            .is_equal_approx(&Vector3::new(
                3.535533905029296875,
                2.121320486068725586,
                2.828427314758300781
            )),
        "slerp with non-normalized values should work as expected."
    );
    assert!(
        Vector3::new(1.0, 1.0, 1.0)
            .slerp(&Vector3::new(2.0, 2.0, 2.0), 0.5)
            .is_equal_approx(&Vector3::new(1.5, 1.5, 1.5)),
        "slerp with colinear inputs should behave as expected."
    );
    assert_eq!(
        Vector3::default().slerp(&Vector3::default(), 0.5),
        Vector3::default(),
        "slerp with both inputs as zero vectors should return a zero vector."
    );
    assert_eq!(
        Vector3::default().slerp(&Vector3::new(1.0, 1.0, 1.0), 0.5),
        Vector3::new(0.5, 0.5, 0.5),
        "slerp with one input as zero should behave like a regular lerp."
    );
    assert_eq!(
        Vector3::new(1.0, 1.0, 1.0).slerp(&Vector3::default(), 0.5),
        Vector3::new(0.5, 0.5, 0.5),
        "slerp with one input as zero should behave like a regular lerp."
    );
    assert!(
        Vector3::new(4.0, 6.0, 2.0)
            .slerp(&Vector3::new(8.0, 10.0, 3.0), 0.5)
            .is_equal_approx(&Vector3::new(
                5.901942198114299,
                8.067586888493786,
                2.5583078947183173
            )),
        "slerp should work as expected."
    );
    assert_approx_eq!(
        vector1.slerp(&vector2, 0.5).length(),
        6.258310887083033,
        "slerp with different length input should return a vector with an interpolated length."
    );
    assert_approx_eq_with_tolerance!(
        vector1.angle_to(&vector1.slerp(&vector2, 0.5)) * 2.0,
        &vector1.angle_to(&vector2),
        0.003110131,
        "slerp with different length input should return a vector with an interpolated angle."
    );
    assert_eq!(
        vector1.cubic_interpolate(
            &vector2,
            &Vector3::default(),
            &Vector3::new(7.0, 7.0, 7.0),
            0.5
        ),
        Vector3::new(2.375, 3.5, 4.625),
        "cubic_interpolate should work as expected."
    );
    assert!(
        vector1
            .cubic_interpolate(
                &vector2,
                &Vector3::default(),
                &Vector3::new(7.0, 7.0, 7.0),
                1.0 / 3.0
            )
            .is_equal_approx(&Vector3::new(
                1.851851940155029297,
                2.962963104248046875,
                4.074074268341064453
            )),
        "cubic_interpolate should work as expected."
    );
    assert_eq!(
        Vector3::new(1.0, 0.0, 0.0).move_toward(&Vector3::new(10.0, 0.0, 0.0), 3.0),
        Vector3::new(4.0, 0.0, 0.0),
        "move_toward should work as expected."
    );
}

#[test]
fn length_methods() {
    let vector1 = Vector3::new(10.0, 10.0, 10.0);
    let vector2 = Vector3::new(20.0, 30.0, 40.0);

    assert_eq!(
        vector1.length_squared(),
        300.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.length(),
        10.0 * float_consts::SQRT_3,
        "length should work as expected."
    );
    assert_eq!(
        vector2.length_squared(),
        2900.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector2.length(),
        53.8516480713450403125,
        "length should work as expected."
    );
    assert_eq!(
        vector1.distance_squared_to(&vector2),
        1400.0,
        "distance_squared_to should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.distance_to(&vector2),
        37.41657386773941385584,
        "distance_to should work as expected."
    );
}

#[test]
fn limiting_methods() {
    let vector = Vector3::new(10.0, 10.0, 10.0);

    assert!(
        vector.limit_length(1.0).is_equal_approx(&Vector3::new(
            float_consts::FRAC_1_SQRT_3,
            float_consts::FRAC_1_SQRT_3,
            float_consts::FRAC_1_SQRT_3
        )),
        "limit_length should work as expected."
    );
    assert!(
        vector.limit_length(5.0).is_equal_approx(
            &(5.0
                * Vector3::new(
                    float_consts::FRAC_1_SQRT_3,
                    float_consts::FRAC_1_SQRT_3,
                    float_consts::FRAC_1_SQRT_3
                ))
        ),
        "limit_length should work as expected."
    );
    assert_eq!(
        Vector3::new(-5.0, 5.0, 15.0).clamp(&Vector3::default(), &vector),
        Vector3::new(0.0, 5.0, 10.0),
        "clamp should work as expected."
    );
    assert_eq!(
        vector.clamp(
            &Vector3::new(0.0, 10.0, 15.0),
            &Vector3::new(5.0, 10.0, 20.0)
        ),
        Vector3::new(5.0, 10.0, 15.0),
        "clamp should work as expected."
    );
}

#[test]
fn normalization_methods() {
    let mut vector = Vector3::new(3.2, -5.4, 6.0);
    vector = vector.normalized();

    assert!(
        Vector3::new(1.0, 0.0, 0.0).is_normalized(),
        "is_normalized should return true for a normalized vector."
    );
    assert!(
        !Vector3::new(1.0, 1.0, 1.0).is_normalized(),
        "is_normalized should return false for a non-normalized vector."
    );
    assert_eq!(
        Vector3::new(1.0, 0.0, 0.0).normalized(),
        Vector3::new(1.0, 0.0, 0.0),
        "normalized should return the same vector for a normalized vector."
    );
    assert!(
        Vector3::new(1.0, 1.0, 0.0)
            .normalized()
            .is_equal_approx(&Vector3::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0
            )),
        "normalized should work as expected."
    );
    assert!(
        Vector3::new(1.0, 1.0, 1.0)
            .normalized()
            .is_equal_approx(&Vector3::new(
                float_consts::FRAC_1_SQRT_3,
                float_consts::FRAC_1_SQRT_3,
                float_consts::FRAC_1_SQRT_3
            )),
        "normalized should work as expected."
    );
    assert_eq!(
        vector,
        Vector3::new(3.2, -5.4, 6.0).normalized(),
        "normalize should convert same way as normalized."
    );
    assert!(
        vector.is_equal_approx(&Vector3::new(
            0.368522751763902980457,
            -0.621882143601586279522,
            0.6909801595573180883585
        )),
        "normalize should work as expected."
    );
}

#[test]
fn operators() {
    let decimal1 = Vector3::new(2.3, 4.9, 7.8);
    let decimal2 = Vector3::new(1.2, 3.4, 5.6);
    let power1 = Vector3::new(0.75, 1.5, 0.625);
    let power2 = Vector3::new(0.5, 0.125, 0.25);
    let int1 = Vector3::new(4.0, 5.0, 9.0);
    let int2 = Vector3::new(1.0, 2.0, 3.0);

    assert!(
        (decimal1 + decimal2).is_equal_approx(&Vector3::new(3.5, 8.3, 13.4)),
        "addition should behave as expected."
    );
    assert_eq!(
        power1 + power2,
        Vector3::new(1.25, 1.625, 0.875),
        "addition with powers of two should give exact results."
    );
    assert_eq!(
        int1 + int2,
        Vector3::new(5.0, 7.0, 12.0),
        "addition with integers should give exact results."
    );
    assert!(
        (decimal1 - decimal2).is_equal_approx(&Vector3::new(1.1, 1.5, 2.2)),
        "subtraction should behave as expected."
    );
    assert_eq!(
        power1 - power2,
        Vector3::new(0.25, 1.375, 0.375),
        "subtraction with powers of two should give exact results."
    );
    assert_eq!(
        int1 - int2,
        Vector3::new(3.0, 3.0, 6.0),
        "subtraction with integers should give exact results."
    );
    assert!(
        (decimal1 * decimal2).is_equal_approx(&Vector3::new(2.76, 16.66, 43.68)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * power2,
        Vector3::new(0.375, 0.1875, 0.15625),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * int2,
        Vector3::new(4.0, 10.0, 27.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / decimal2).is_equal_approx(&Vector3::new(
            1.91666666666666666,
            1.44117647058823529,
            1.39285714285714286
        )),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / power2,
        Vector3::new(1.5, 12.0, 2.5),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / int2,
        Vector3::new(4.0, 2.5, 3.0),
        "division with integers should give exact results."
    );
    assert!(
        (decimal1 * 2.0).is_equal_approx(&Vector3::new(4.6, 9.8, 15.6)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * 2.0,
        Vector3::new(1.5, 3.0, 1.25),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * 2.0,
        Vector3::new(8.0, 10.0, 18.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / 2.0).is_equal_approx(&Vector3::new(1.15, 2.45, 3.9)),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / 2.0,
        Vector3::new(0.375, 0.75, 0.3125),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / 2.0,
        Vector3::new(2.0, 2.5, 4.5),
        "division with integers should give exact results."
    );
    assert_eq!(
        Vector3i::from(decimal1),
        Vector3i::new(2, 4, 7),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector3i::from(decimal2),
        Vector3i::new(1, 3, 5),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector3::from(Vector3i::new(1, 2, 3)),
        Vector3::new(1.0, 2.0, 3.0),
        "constructed from should work as expected."
    );
    assert_eq!(
        decimal1.to_string(),
        "Vector3(2.3, 4.9, 7.8)",
        "cast to String should work as expected."
    );
    assert_eq!(
        decimal2.to_string(),
        "Vector3(1.2, 3.4, 5.6)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector3::new(9.7, 9.8, 9.9).to_string(),
        "Vector3(9.7, 9.8, 9.9)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector3::new(float_consts::E, float_consts::SQRT_2, float_consts::SQRT_3).to_string(),
        format!(
            "Vector3({}, {}, {})",
            float_consts::E,
            float_consts::SQRT_2,
            float_consts::SQRT_3
        ),
        "cast to String should print the correct amount of digits for real_t = let."
    );
}

#[test]
fn other_methods() {
    let vector = Vector3::new(1.2, 3.4, 5.6);

    assert!(
        vector
            .direction_to(&Vector3::default())
            .is_equal_approx(&-vector.normalized()),
        "direction_to should work as expected."
    );
    assert!(
        Vector3::new(1.0, 1.0, 1.0)
            .direction_to(&Vector3::new(2.0, 2.0, 2.0))
            .is_equal_approx(&Vector3::new(
                float_consts::FRAC_1_SQRT_3,
                float_consts::FRAC_1_SQRT_3,
                float_consts::FRAC_1_SQRT_3
            )),
        "direction_to should work as expected."
    );
    assert!(
        vector
            .inverse()
            .is_equal_approx(&Vector3::new(1.0 / 1.2, 1.0 / 3.4, 1.0 / 5.6)),
        "inverse should work as expected."
    );
    assert!(
        vector
            .posmod(2.0)
            .is_equal_approx(&Vector3::new(1.2, 1.4, 1.6)),
        "posmod should work as expected."
    );
    assert!(
        (-vector)
            .posmod(2.0)
            .is_equal_approx(&Vector3::new(0.8, 0.6, 0.4)),
        "posmod should work as expected."
    );
    assert!(
        vector
            .posmodv(&Vector3::new(1.0, 2.0, 3.0))
            .is_equal_approx(&Vector3::new(0.2, 1.4, 2.6)),
        "posmodv should work as expected."
    );
    assert!(
        (-vector)
            .posmodv(&Vector3::new(2.0, 3.0, 4.0))
            .is_equal_approx(&Vector3::new(0.8, 2.6, 2.4)),
        "posmodv should work as expected."
    );
    assert!(
        vector
            .rotated(&Vector3::new(0.0, 1.0, 0.0), TAU)
            .is_equal_approx(&vector),
        "rotated should work as expected."
    );
    assert!(
        vector
            .rotated(&Vector3::new(0.0, 1.0, 0.0), TAU / 4.0)
            .is_equal_approx(&Vector3::new(5.6, 3.4, -1.2)),
        "rotated should work as expected."
    );
    println!(
        "{}",
        vector.rotated(&Vector3::new(1.0, 0.0, 0.0), TAU / 3.0)
    );
    assert!(
        vector
            .rotated(&Vector3::new(1.0, 0.0, 0.0), TAU / 3.0)
            .is_equal_approx(&Vector3::new(1.2, -6.54974226119285642, 0.1444863728670914)),
        "rotated should work as expected."
    );
    assert!(
        vector
            .rotated(&Vector3::new(0.0, 0.0, 1.0), TAU / 2.0)
            .is_equal_approx(&vector.rotated(&Vector3::new(0.0, 0.0, 1.0), TAU / -2.0)),
        "rotated should work as expected."
    );
    assert_eq!(
        vector.snapped(&Vector3::new(1.0, 1.0, 1.0)),
        Vector3::new(1.0, 3.0, 6.0),
        "snapped to integers should be the same as rounding."
    );
    assert_eq!(
        vector.snapped(&Vector3::new(0.25, 0.25, 0.25)),
        Vector3::new(1.25, 3.5, 5.5),
        "snapped to 0.25 should give exact results."
    );
    assert!(
        Vector3::new(1.2, 2.5, 2.0).is_equal_approx(&vector.min(&Vector3::new(3.0, 2.5, 2.0))),
        "min should return expected value."
    );
    assert!(
        Vector3::new(5.3, 3.4, 5.6).is_equal_approx(&vector.max(&Vector3::new(5.3, 2.0, 3.0))),
        "max should return expected value."
    );
}

#[test]
fn plane_methods() {
    let vector = Vector3::new(1.2, 3.4, 5.6);
    let vector_y = Vector3::new(0.0, 1.0, 0.0);
    let vector_normal = Vector3::new(
        0.88763458893247992491,
        0.26300284116517923701,
        0.37806658417494515320,
    );

    assert_eq!(
        vector.bounce(&vector_y),
        Vector3::new(1.2, -3.4, 5.6),
        "bounce on a plane with normal of the Y axis should."
    );
    assert!(
        vector.bounce(&vector_normal).is_equal_approx(&Vector3::new(
            -6.0369629829775736287,
            1.25571467171034855444,
            2.517589840583626047
        )),
        "bounce with normal should return expected value."
    );
    assert_eq!(
        vector.reflect(&vector_y),
        Vector3::new(-1.2, 3.4, -5.6),
        "reflect on a plane with normal of the Y axis should."
    );
    assert!(
        vector
            .reflect(&vector_normal)
            .is_equal_approx(&Vector3::new(
                6.0369629829775736287,
                -1.25571467171034855444,
                -2.517589840583626047
            )),
        "reflect with normal should return expected value."
    );
    assert_eq!(
        vector.project(&vector_y),
        Vector3::new(0.0, 3.4, 0.0),
        "projected on the Y axis should only give the Y component."
    );
    assert!(
        vector
            .project(&vector_normal)
            .is_equal_approx(&Vector3::new(
                3.61848149148878681437,
                1.0721426641448257227776,
                1.54120507970818697649
            )),
        "projected on a normal should return expected value."
    );
    assert_eq!(
        vector.slide(&vector_y),
        Vector3::new(1.2, 0.0, 5.6),
        "slide on a plane with normal of the Y axis should set the Y to zero."
    );
    assert!(
        vector.slide(&vector_normal).is_equal_approx(&Vector3::new(
            -2.41848149148878681437,
            2.32785733585517427722237,
            4.0587949202918130235
        )),
        "slide with normal should return expected value."
    );

    // TODO: implement math checks
    // There's probably a better way to test these ones?
    //let vector_non_normal = Vector3::new(5.4, 1.6, 2.3);
    //assert!(
    //    vector.bounce(&vector_non_normal).is_equal_approx(&Vector3::default()),
    //    "bounce should return empty with non-normalized input.");
    //assert!(
    //    vector.reflect(&vector_non_normal).is_equal_approx(&Vector3::default()),
    //    "reflect should return empty with non-normalized input.");
    //assert!(
    //    vector.slide(&vector_non_normal).is_equal_approx(&Vector3::default()),
    //    "slide should return empty with non-normalized input.");
}

#[test]
fn rounding_methods() {
    let vector1 = Vector3::new(1.2, 3.4, 5.6);
    let vector2 = Vector3::new(1.2, -3.4, -5.6);

    assert_eq!(vector1.abs(), vector1, "abs should work as expected.");
    assert_eq!(vector2.abs(), vector1, "abs should work as expected.");
    assert_eq!(
        vector1.ceil(),
        Vector3::new(2.0, 4.0, 6.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector2.ceil(),
        Vector3::new(2.0, -3.0, -5.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector1.floor(),
        Vector3::new(1.0, 3.0, 5.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector2.floor(),
        Vector3::new(1.0, -4.0, -6.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector1.round(),
        Vector3::new(1.0, 3.0, 6.0),
        "round should work as expected."
    );
    assert_eq!(
        vector2.round(),
        Vector3::new(1.0, -3.0, -6.0),
        "round should work as expected."
    );
    assert_eq!(
        vector1.sign(),
        Vector3::new(1.0, 1.0, 1.0),
        "sign should work as expected."
    );
    assert_eq!(
        vector2.sign(),
        Vector3::new(1.0, -1.0, -1.0),
        "sign should work as expected."
    );
}

#[test]
fn linear_algebra_methods() {
    let vector_x = Vector3::new(1.0, 0.0, 0.0);
    let vector_y = Vector3::new(0.0, 1.0, 0.0);
    let vector_z = Vector3::new(0.0, 0.0, 1.0);
    let a = Vector3::new(3.5, 8.5, 2.3);
    let b = Vector3::new(5.2, 4.6, 7.8);

    assert_eq!(
        vector_x.cross(&vector_y),
        vector_z,
        "cross product of X and Y should give Z."
    );
    assert_eq!(
        vector_y.cross(&vector_x),
        -vector_z,
        "cross product of Y and X should give negative Z."
    );
    assert_eq!(
        vector_y.cross(&vector_z),
        vector_x,
        "cross product of Y and Z should give X."
    );
    assert_eq!(
        vector_z.cross(&vector_x),
        vector_y,
        "cross product of Z and X should give Y."
    );
    assert!(
        a.cross(&b)
            .is_equal_approx(&Vector3::new(55.72, -15.34, -28.1)),
        "cross should return expected value."
    );
    assert!(
        Vector3::new(-a.x, a.y, -a.z)
            .cross(&Vector3::new(b.x, -b.y, b.z))
            .is_equal_approx(&Vector3::new(55.72, 15.34, -28.1)),
        "cross should return expected value."
    );
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
    assert_approx_eq!(a.dot(&b), 75.24, "dot should return expected value.");
    assert_approx_eq!(
        Vector3::new(-a.x, a.y, -a.z).dot(&Vector3::new(b.x, -b.y, b.z)),
        -75.24,
        "dot should return expected value."
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
        Vector3::new(0.0, 1.0, 2.0).is_finite(),
        "Vector3(0, 1, 2) should be finite"
    );

    for x in &infinite {
        assert!(
            !Vector3::new(*x, 1.0, 2.0).is_finite(),
            "Vector3 with one component infinite should not be finite. Failed with: {}",
            x
        );
        assert!(
            !Vector3::new(0.0, *x, 2.0).is_finite(),
            "Vector3 with one component infinite should not be finite. Failed with: {}",
            x
        );
        assert!(
            !Vector3::new(0.0, 1.0, *x).is_finite(),
            "Vector3 with one component infinite should not be finite. Failed with: {}",
            x
        );
    }

    for x in &infinite {
        for y in &infinite {
            assert!(
                !Vector3::new(*x, *y, 2.0).is_finite(),
                "Vector3 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector3::new(*x, 1.0, *y).is_finite(),
                "Vector3 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
            assert!(
                !Vector3::new(0.0, *x, *y).is_finite(),
                "Vector3 with two components infinite should not be finite. Failed with: {}, {}",
                x,
                y
            );
        }
    }

    for x in &infinite {
        for y in &infinite {
            for z in &infinite {
                assert!(!
                    Vector3::new(*x, *y, *z).is_finite(),
                    "Vector3 with three components infinite should not be finite. Failed with: {}, {}, {}", x, y, z);
            }
        }
    }
}
