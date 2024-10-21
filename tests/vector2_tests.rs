use huginn::types::vectors::{Vector2, Vector2i, AXIS};
use huginn::utils::{
    float_consts::{FRAC_1_SQRT_2, PI, SQRT_2, TAU},
    CMP_EPSILON,
};

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
    let vector_empty = Vector2::default();
    let vector_zero = Vector2::new(0.0, 0.0);

    assert_eq!(
        vector_empty, vector_zero,
        "Constructor with no inputs should return a zero Vector2."
    );
}

#[test]
fn angle_methods() {
    let vector_x = Vector2::new(1.0, 0.0);
    let vector_y = Vector2::new(0.0, 1.0);

    assert_approx_eq!(
        vector_x.angle_to(&vector_y),
        TAU / 4.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_y.angle_to(&vector_x),
        -TAU / 4.0,
        "angle_to should work as expected."
    );
    assert_approx_eq!(
        vector_x.angle_to_point(&vector_y),
        TAU * 3.0 / 8.0,
        "angle_to_point should work as expected."
    );
    assert_approx_eq!(
        vector_y.angle_to_point(&vector_x),
        -TAU / 8.0,
        "angle_to_point should work as expected."
    );
}

#[test]
fn axis_methods() {
    let mut vector = Vector2::new(1.2, 3.4);

    assert_eq!(
        vector.max_axis_index(),
        AXIS::Y,
        "max_axis_index should work as expected."
    );
    assert_eq!(
        vector.min_axis_index(),
        AXIS::X,
        "min_axis_index should work as expected."
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
    let vector1 = Vector2::new(1.0, 2.0);
    let vector2 = Vector2::new(4.0, 5.0);

    assert_eq!(
        vector1.lerp(&vector2, 0.5),
        Vector2::new(2.5, 3.5),
        "lerp should work as expected."
    );
    assert!(
        vector1
            .lerp(&vector2, 1.0 / 3.0)
            .is_equal_approx(&Vector2::new(2.0, 3.0)),
        "lerp should work as expected."
    );
    assert!(
        vector1
            .normalized()
            .slerp(&vector2.normalized(), 0.5)
            .is_equal_approx(&Vector2::new(0.538953602313995361, 0.84233558177947998)),
        "slerp should work as expected."
    );
    assert!(
        vector1
            .normalized()
            .slerp(&vector2.normalized(), 1.0 / 3.0)
            .is_equal_approx(&Vector2::new(0.508990883827209473, 0.860771894454956055)),
        "slerp should work as expected."
    );
    assert!(
        Vector2::new(5.0, 0.0)
            .slerp(&Vector2::new(0.0, 5.0), 0.5)
            .is_equal_approx(&(Vector2::new(5.0, 5.0) * FRAC_1_SQRT_2)),
        "slerp with non-normalized values should work as expected."
    );
    assert!(
        Vector2::new(1.0, 1.0)
            .slerp(&Vector2::new(2.0, 2.0), 0.5)
            .is_equal_approx(&Vector2::new(1.5, 1.5)),
        "slerp with colinear inputs should behave as expected."
    );
    assert_eq!(
        Vector2::default().slerp(&Vector2::default(), 0.5),
        Vector2::default(),
        "slerp with both inputs as zero vectors should return a zero vector."
    );
    assert_eq!(
        Vector2::default().slerp(&Vector2::new(1.0, 1.0), 0.5),
        Vector2::new(0.5, 0.5),
        "slerp with one input as zero should behave like a regular lerp."
    );
    assert_eq!(
        Vector2::new(1.0, 1.0).slerp(&Vector2::default(), 0.5),
        Vector2::new(0.5, 0.5),
        "slerp with one input as zero should behave like a regular lerp."
    );
    assert!(
        Vector2::new(4.0, 6.0)
            .slerp(&Vector2::new(8.0, 10.0), 0.5)
            .is_equal_approx(&Vector2::new(5.9076470794008017626, 8.07918879020090480697)),
        "slerp should work as expected."
    );
    assert_approx_eq!(
        vector1.slerp(&vector2, 0.5).length(),
        4.31959610746631919,
        "slerp with different length input should return a vector with an interpolated length."
    );
    assert_approx_eq!(
        vector1.angle_to(&vector1.slerp(&vector2, 0.5)) * 2.0,
        &vector1.angle_to(&vector2),
        "slerp with different length input should return a vector with an interpolated angle."
    );
    assert_eq!(
        vector1.cubic_interpolate(&vector2, &Vector2::default(), &Vector2::new(7.0, 7.0), 0.5),
        Vector2::new(2.375, 3.5),
        "cubic_interpolate should work as expected."
    );
    assert!(
        vector1
            .cubic_interpolate(
                &vector2,
                &Vector2::default(),
                &Vector2::new(7.0, 7.0),
                1.0 / 3.0
            )
            .is_equal_approx(&Vector2::new(1.851851940155029297, 2.962963104248046875)),
        "cubic_interpolate should work as expected."
    );
    assert_eq!(
        Vector2::new(1.0, 0.0).move_toward(&Vector2::new(10.0, 0.0), 3.0),
        Vector2::new(4.0, 0.0),
        "move_toward should work as expected."
    );
}

#[test]
fn length_methods() {
    let vector1 = Vector2::new(10.0, 10.0);
    let vector2 = Vector2::new(20.0, 30.0);

    assert_eq!(
        vector1.length_squared(),
        200.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.length(),
        10.0 * SQRT_2,
        "length should work as expected."
    );
    assert_eq!(
        vector2.length_squared(),
        1300.0,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector2.length(),
        36.05551275463989293119,
        "length should work as expected."
    );
    assert_eq!(
        vector1.distance_squared_to(&vector2),
        500.0,
        "distance_squared_to should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.distance_to(&vector2),
        22.36067977499789696409,
        "distance_to should work as expected."
    );
}

#[test]
fn limiting_methods() {
    let vector = Vector2::new(10.0, 10.0);

    assert!(
        vector
            .limit_length(1.0)
            .is_equal_approx(&Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
        "limit_length should work as expected."
    );
    assert!(
        vector
            .limit_length(5.0)
            .is_equal_approx(&(5.0 * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2))),
        "limit_length should work as expected."
    );
    assert!(
        Vector2::new(-5.0, 15.0)
            .clamp(&Vector2::default(), &vector)
            .is_equal_approx(&Vector2::new(0.0, 10.0)),
        "clamp should work as expected."
    );
    assert!(
        vector
            .clamp(&Vector2::new(0.0, 15.0), &Vector2::new(5.0, 20.0))
            .is_equal_approx(&Vector2::new(5.0, 15.0)),
        "clamp should work as expected."
    );
}

#[test]
fn normalization_methods() {
    let mut vector = Vector2::new(3.2, -5.4);
    vector = vector.normalized();

    assert!(
        Vector2::new(1.0, 0.0).is_normalized(),
        "is_normalized should return true for a normalized vector."
    );
    assert!(
        !Vector2::new(1.0, 1.0).is_normalized(),
        "is_normalized should return false for a non-normalized vector."
    );
    assert_eq!(
        Vector2::new(1.0, 0.0).normalized(),
        Vector2::new(1.0, 0.0),
        "normalized should return the same vector for a normalized vector."
    );
    assert!(
        Vector2::new(1.0, 1.0)
            .normalized()
            .is_equal_approx(&Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
        "normalized should work as expected."
    );
    assert_eq!(
        vector,
        Vector2::new(3.2, -5.4).normalized(),
        "normalize should convert same way as normalized."
    );
    assert!(
        vector.is_equal_approx(&Vector2::new(
            0.509802390301732898898,
            -0.860291533634174266891
        )),
        "normalize should work as expected."
    );
}

#[test]
fn operators() {
    let decimal1 = Vector2::new(2.3, 4.9);
    let decimal2 = Vector2::new(1.2, 3.4);
    let power1 = Vector2::new(0.75, 1.5);
    let power2 = Vector2::new(0.5, 0.125);
    let int1 = Vector2::new(4.0, 5.0);
    let int2 = Vector2::new(1.0, 2.0);

    assert!(
        (decimal1 + decimal2).is_equal_approx(&Vector2::new(3.5, 8.3)),
        "addition should behave as expected."
    );
    assert_eq!(
        power1 + power2,
        Vector2::new(1.25, 1.625),
        "addition with powers of two should give exact results."
    );
    assert_eq!(
        int1 + int2,
        Vector2::new(5.0, 7.0),
        "addition with integers should give exact results."
    );
    assert!(
        (decimal1 - decimal2).is_equal_approx(&Vector2::new(1.1, 1.5)),
        "subtraction should behave as expected."
    );
    assert_eq!(
        power1 - power2,
        Vector2::new(0.25, 1.375),
        "subtraction with powers of two should give exact results."
    );
    assert_eq!(
        int1 - int2,
        Vector2::new(3.0, 3.0),
        "subtraction with integers should give exact results."
    );
    assert!(
        (decimal1 * decimal2).is_equal_approx(&Vector2::new(2.76, 16.66)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * power2,
        Vector2::new(0.375, 0.1875),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * int2,
        Vector2::new(4.0, 10.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / decimal2)
            .is_equal_approx(&Vector2::new(1.91666666666666666, 1.44117647058823529)),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / power2,
        Vector2::new(1.5, 12.0),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / int2,
        Vector2::new(4.0, 2.5),
        "division with integers should give exact results."
    );
    assert!(
        (decimal1 * 2.0).is_equal_approx(&Vector2::new(4.6, 9.8)),
        "multiplication should behave as expected."
    );
    assert_eq!(
        power1 * 2.0,
        Vector2::new(1.5, 3.0),
        "multiplication with powers of two should give exact results."
    );
    assert_eq!(
        int1 * 2.0,
        Vector2::new(8.0, 10.0),
        "multiplication with integers should give exact results."
    );
    assert!(
        (decimal1 / 2.0).is_equal_approx(&Vector2::new(1.15, 2.45)),
        "division should behave as expected."
    );
    assert_eq!(
        power1 / 2.0,
        Vector2::new(0.375, 0.75),
        "division with powers of two should give exact results."
    );
    assert_eq!(
        int1 / 2.0,
        Vector2::new(2.0, 2.5),
        "division with integers should give exact results."
    );
    assert_eq!(
        Vector2i::from(decimal1),
        Vector2i::new(2, 4),
        "cast to Vector2i should work as expected."
    );
    assert_eq!(
        Vector2i::from(decimal2),
        Vector2i::new(1, 3),
        "cast to Vector2i should work as expected."
    );
    assert_eq!(
        Vector2::from(Vector2i::new(1, 2)),
        Vector2::new(1.0, 2.0),
        "costructed from Vector2i should work as expected."
    );
    assert_eq!(
        decimal1.to_string(),
        "Vector2(2.3, 4.9)",
        "cast to String should work as expected."
    );
    assert_eq!(
        decimal2.to_string(),
        "Vector2(1.2, 3.4)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector2::new(9.8, 9.9).to_string(),
        "Vector2(9.8, 9.9)",
        "cast to String should work as expected."
    );
    assert_eq!(
        Vector2::new(PI, TAU).to_string(),
        "Vector2(".to_string() + &*PI.to_string() + ", " + &*TAU.to_string() + ")",
        "cast to String should print the correct amount of digits."
    );
}

#[test]
fn other_methods() {
    let vector = Vector2::new(1.2, 3.4);

    assert_approx_eq!(
        vector.aspect(),
        1.2 / 3.4,
        "aspect should work as expected."
    );
    assert!(
        vector
            .direction_to(&Vector2::default())
            .is_equal_approx(&-vector.normalized()),
        "direction_to should work as expected."
    );
    assert!(
        Vector2::new(1.0, 1.0)
            .direction_to(&Vector2::new(2.0, 2.0))
            .is_equal_approx(&Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
        "direction_to should work as expected."
    );
    assert!(
        vector.posmod(2.0).is_equal_approx(&Vector2::new(1.2, 1.4)),
        "posmod should work as expected."
    );
    assert!(
        (-vector)
            .posmod(2.0)
            .is_equal_approx(&Vector2::new(0.8, 0.6)),
        "posmod should work as expected."
    );
    assert!(
        vector
            .posmodv(&Vector2::new(1.0, 2.0))
            .is_equal_approx(&Vector2::new(0.2, 1.4)),
        "posmodv should work as expected."
    );
    assert!(
        (-vector)
            .posmodv(&Vector2::new(2.0, 3.0))
            .is_equal_approx(&Vector2::new(0.8, 2.6)),
        "posmodv should work as expected."
    );
    assert!(
        vector.rotated(TAU).is_equal_approx(&Vector2::new(1.2, 3.4)),
        "rotated should work as expected."
    );
    assert!(
        vector
            .rotated(TAU / 4.0)
            .is_equal_approx(&Vector2::new(-3.4, 1.2)),
        "rotated should work as expected."
    );
    assert!(
        vector.rotated(TAU / 3.0).is_equal_approx(&Vector2::new(
            -3.544486372867091398996,
            -0.660769515458673623883
        )),
        "rotated should work as expected."
    );
    assert!(
        vector
            .rotated(TAU / 2.0)
            .is_equal_approx(&vector.rotated(TAU / -2.0)),
        "rotated should work as expected."
    );
    assert_eq!(
        vector.snapped(&Vector2::new(1.0, 1.0)),
        Vector2::new(1.0, 3.0),
        "snapped to integers should be the same as rounding."
    );
    assert_eq!(
        Vector2::new(3.4, 5.6).snapped(&Vector2::new(1.0, 1.0)),
        Vector2::new(3.0, 6.0),
        "snapped to integers should be the same as rounding."
    );
    assert_eq!(
        vector.snapped(&Vector2::new(0.25, 0.25)),
        Vector2::new(1.25, 3.5),
        "snapped to 0.25 should give exact results."
    );
    assert!(
        Vector2::new(1.2, 2.5).is_equal_approx(&vector.min(&Vector2::new(3.0, 2.5))),
        "min should return expected value."
    );
    assert!(
        Vector2::new(5.3, 3.4).is_equal_approx(&vector.max(&Vector2::new(5.3, 2.0))),
        "max should return expected value."
    );
}

#[test]
fn plane_methods() {
    let vector = Vector2::new(1.2, 3.4);
    let vector_y = Vector2::new(0.0, 1.0);
    let vector_normal = Vector2::new(0.95879811270838721622267, 0.2840883296913739899919);
    let d = 99.1;

    assert_eq!(
        vector.bounce(&vector_y),
        Vector2::new(1.2, -3.4),
        "bounce on a plane with normal of the Y axis should."
    );
    assert!(
        vector.bounce(&vector_normal).is_equal_approx(&Vector2::new(
            -2.85851197982345523329,
            2.197477931904161412358
        )),
        "bounce with normal should return expected value."
    );
    assert_eq!(
        vector.reflect(&vector_y),
        Vector2::new(-1.2, 3.4),
        "reflect on a plane with normal of the Y axis should."
    );
    assert!(
        vector
            .reflect(&vector_normal)
            .is_equal_approx(&Vector2::new(
                2.85851197982345523329,
                -2.197477931904161412358
            )),
        "reflect with normal should return expected value."
    );
    assert_eq!(
        vector.project(&vector_y),
        Vector2::new(0.0, 3.4),
        "projected on the Y axis should only give the Y component."
    );
    assert!(
        vector
            .project(&vector_normal)
            .is_equal_approx(&Vector2::new(2.0292559899117276166, 0.60126103404791929382)),
        "projected on a normal should return expected value."
    );
    assert!(
        vector_normal
            .plane_project(d, &vector)
            .is_equal_approx(&Vector2::new(94.187635516479631, 30.951892004882851)),
        "plane_project should return expected value."
    );
    assert_eq!(
        vector.slide(&vector_y),
        Vector2::new(1.2, 0.0),
        "slide on a plane with normal of the Y axis should set the Y to zero."
    );
    assert!(
        vector.slide(&vector_normal).is_equal_approx(&Vector2::new(
            -0.8292559899117276166456,
            2.798738965952080706179
        )),
        "slide with normal should return expected value."
    );
    // TODO: Implement math checks
    //let vector_non_normal = Vector2::new(5.4, 1.6);
    //assert!(
    //    vector
    //        .bounce(&vector_non_normal)
    //        .is_equal_approx(&Vector2::default()),
    //    "bounce should return empty with non-normalized input."
    //);
    //assert!(
    //    vector
    //        .reflect(&vector_non_normal)
    //        .is_equal_approx(&Vector2::default()),
    //    "reflect should return empty with non-normalized input."
    //);
    //assert!(
    //    vector
    //        .slide(&vector_non_normal)
    //        .is_equal_approx(&Vector2::default()),
    //    "slide should return empty with non-normalized input."
    //);
}

#[test]
fn rounding_methods() {
    let vector1 = Vector2::new(1.2, 5.6);
    let vector2 = Vector2::new(1.2, -5.6);

    assert_eq!(vector1.abs(), vector1, "abs should work as expected.");
    assert_eq!(vector2.abs(), vector1, "abs should work as expected.");
    assert_eq!(
        vector1.ceil(),
        Vector2::new(2.0, 6.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector2.ceil(),
        Vector2::new(2.0, -5.0),
        "ceil should work as expected."
    );
    assert_eq!(
        vector1.floor(),
        Vector2::new(1.0, 5.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector2.floor(),
        Vector2::new(1.0, -6.0),
        "floor should work as expected."
    );
    assert_eq!(
        vector1.round(),
        Vector2::new(1.0, 6.0),
        "round should work as expected."
    );
    assert_eq!(
        vector2.round(),
        Vector2::new(1.0, -6.0),
        "round should work as expected."
    );
    assert_eq!(
        vector1.sign(),
        Vector2::new(1.0, 1.0),
        "sign should work as expected."
    );
    assert_eq!(
        vector2.sign(),
        Vector2::new(1.0, -1.0),
        "sign should work as expected."
    );
}

#[test]
fn linear_algebra_methods() {
    let vector_x = Vector2::new(1.0, 0.0);
    let vector_y = Vector2::new(0.0, 1.0);
    let a = Vector2::new(3.5, 8.5);
    let b = Vector2::new(5.2, 4.6);

    assert_eq!(
        vector_x.cross(&vector_y),
        1.0,
        "cross product of X and Y should give 1."
    );
    assert_eq!(
        vector_y.cross(&vector_x),
        -1.0,
        "cross product of Y and X should give negative 1."
    );
    assert_approx_eq!(a.cross(&b), -28.1, "cross should return expected value.");
    assert_approx_eq!(
        Vector2::new(-a.x, a.y).cross(&Vector2::new(b.x, -b.y)),
        -28.1,
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
    assert_approx_eq!(a.dot(&b), 57.3, "dot should return expected value.");
    assert_approx_eq!(
        Vector2::new(-a.x, a.y).dot(&Vector2::new(b.x, -b.y)),
        -57.3,
        "dot should return expected value."
    );
}
