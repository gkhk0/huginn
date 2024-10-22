use huginn::types::vectors::{Vector3, Vector3i, AXIS};
use huginn::utils::{float, float_consts, CMP_EPSILON};

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
    let vector_empty = Vector3i::default();
    let vector_zero = Vector3i::new(0, 0, 0);

    assert_eq!(
        vector_empty, vector_zero,
        "Constructor with no inputs should return a zero Vector3i."
    );
}

#[test]
fn axis_methods() {
    let mut vector = Vector3i::new(1, 2, 3);

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
        3,
        "array operator should work as expected."
    );
    assert_eq!(
        vector.get_axis(vector.min_axis_index()),
        1,
        "array operator should work as expected."
    );
    vector.set_axis(AXIS::Y, 5);
    assert_eq!(
        vector.get_axis(AXIS::Y),
        5,
        "array operator setter should work as expected."
    );
}

#[test]
fn clamp_method() {
    let vector = Vector3i::new(10, 10, 10);

    assert_eq!(
        Vector3i::new(-5, 5, 15).clamp(&Vector3i::default(), &vector),
        Vector3i::new(0, 5, 10),
        "clamp should work as expected."
    );
    assert_eq!(
        vector.clamp(&Vector3i::new(0, 10, 15), &Vector3i::new(5, 10, 20)),
        Vector3i::new(5, 10, 15),
        "clamp should work as expected."
    );
}

#[test]
fn length_methods() {
    let vector1 = Vector3i::new(10, 10, 10);
    let vector2 = Vector3i::new(20, 30, 40);

    assert_eq!(
        vector1.length_squared(),
        300,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.length(),
        10.0 * SQRT_3,
        "length should work as expected."
    );
    assert_eq!(
        vector2.length_squared(),
        2900,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector2.length(),
        53.8516480713450403125,
        "length should work as expected."
    );
    assert_eq!(
        vector1.distance_squared_to(&vector2),
        1400,
        "distance_squared_to should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.distance_to(&vector2),
        37.41657386773941385584,
        "distance_to should work as expected."
    );
}

#[test]
fn operators() {
    let vector1 = Vector3i::new(4, 5, 9);
    let vector2 = Vector3i::new(1, 2, 3);

    assert_eq!(
        (&vector1 + vector2),
        Vector3i::new(5, 7, 12),
        "addition with integers should give exact results."
    );
    assert_eq!(
        (&vector1 - vector2),
        Vector3i::new(3, 3, 6),
        "subtraction with integers should give exact results."
    );
    assert_eq!(
        (&vector1 * vector2),
        Vector3i::new(4, 10, 27),
        "multiplication with integers should give exact results."
    );
    assert_eq!(
        (&vector1 / vector2),
        Vector3i::new(4, 2, 3),
        "division with integers should give exact results."
    );
    assert_eq!(
        (&vector1 * 2),
        Vector3i::new(8, 10, 18),
        "multiplication with integers should give exact results."
    );
    assert_eq!(
        (&vector1 / 2),
        Vector3i::new(2, 2, 4),
        "division with integers should give exact results."
    );
    assert_eq!(
        Vector3::from(vector1),
        Vector3::new(4.0, 5.0, 9.0),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector3::from(vector2),
        Vector3::new(1.0, 2.0, 3.0),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector3i::from(&Vector3::new(1.1, 2.9, 3.9)),
        Vector3i::new(1, 2, 3),
        "constructed from should work as expected."
    );
}

#[test]
fn other_methods() {
    let vector = Vector3i::new(1, 3, -7);

    assert_eq!(
        vector.min(&Vector3i::new(3, 2, 5)),
        Vector3i::new(1, 2, -7),
        "min should return expected value."
    );
    assert_eq!(
        vector.max(&Vector3i::new(5, 2, 4)),
        Vector3i::new(5, 3, 4),
        "max should return expected value."
    );
    assert_eq!(
        vector.snapped(&Vector3i::new(4, 2, 5)),
        Vector3i::new(0, 4, -5),
        "snapped should work as expected."
    );
}

#[test]
fn abs_and_sign_methods() {
    let vector1 = Vector3i::new(1, 3, 5);
    let vector2 = Vector3i::new(1, -3, -5);

    assert_eq!(vector1.abs(), vector1, "abs should work as expected.");
    assert_eq!(vector2.abs(), vector1, "abs should work as expected.");
    assert_eq!(
        vector1.sign(),
        Vector3i::new(1, 1, 1),
        "sign should work as expected."
    );
    assert_eq!(
        vector2.sign(),
        Vector3i::new(1, -1, -1),
        "sign should work as expected."
    );
}
