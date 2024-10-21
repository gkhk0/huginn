use huginn::types::vectors::AXIS;
use huginn::types::vectors::{Vector2, Vector2i};
use huginn::utils::float_consts;
use huginn::utils::CMP_EPSILON;

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
    let vector_empty = Vector2i::default();
    let vector_zero = Vector2i::new(0, 0);

    assert_eq!(
        vector_empty, vector_zero,
        "Constructor with no inputs should return a zero Vector2i."
    );
}

#[test]
fn axis_methods() {
    let mut vector = Vector2i::new(2, 3);

    vector.set_axis(AXIS::Y, 5);

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
        2,
        "array operator should work as expected."
    );
    assert_eq!(
        vector.get_axis(AXIS::Y),
        5,
        "array operator setter should work as expected."
    );
}

#[test]
fn clamp_method() {
    let vector = Vector2i::new(10, 10);

    assert_eq!(
        Vector2i::new(-5, 15).clamp(&Vector2i::default(), &vector),
        Vector2i::new(0, 10),
        "clamp should work as expected."
    );
    assert_eq!(
        vector.clamp(&Vector2i::new(0, 15), &Vector2i::new(5, 20)),
        Vector2i::new(5, 15),
        "clamp should work as expected."
    );
}

#[test]
fn length_methods() {
    let vector1 = Vector2i::new(10, 10);
    let vector2 = Vector2i::new(20, 30);

    assert_eq!(
        vector1.length_squared(),
        200,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.length(),
        10.0 * float_consts::SQRT_2,
        "length should work as expected."
    );
    assert_eq!(
        vector2.length_squared(),
        1300,
        "length_squared should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector2.length(),
        36.05551275463989293119,
        "length should work as expected."
    );
    assert_eq!(
        vector1.distance_squared_to(&vector2),
        500,
        "distance_squared_to should work as expected and return exact result."
    );
    assert_approx_eq!(
        vector1.distance_to(&vector2),
        22.36067977499789696409,
        "distance_to should work as expected."
    );
}

#[test]
fn operators() {
    let vector1 = Vector2i::new(5, 9);
    let vector2 = Vector2i::new(2, 3);

    assert_eq!(
        vector1 + vector2,
        Vector2i::new(7, 12),
        "addition with integers should give exact results."
    );
    assert_eq!(
        vector1 - vector2,
        Vector2i::new(3, 6),
        "subtraction with integers should give exact results."
    );
    assert_eq!(
        vector1 * vector2,
        Vector2i::new(10, 27),
        "multiplication with integers should give exact results."
    );
    assert_eq!(
        vector1 / vector2,
        Vector2i::new(2, 3),
        "division with integers should give exact results."
    );
    assert_eq!(
        &vector1 * 2,
        Vector2i::new(10, 18),
        "multiplication with integers should give exact results."
    );
    assert_eq!(
        &vector1 / 2,
        Vector2i::new(2, 4),
        "division with integers should give exact results."
    );
    assert_eq!(
        Vector2::from(vector1),
        Vector2::new(5.0, 9.0),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector2::from(vector2),
        Vector2::new(2.0, 3.0),
        "cast to should work as expected."
    );
    assert_eq!(
        Vector2i::from(Vector2::new(1.1, 2.9)),
        Vector2i::new(1, 2),
        "constructed from should work as expected."
    );
}

#[test]
fn other_methods() {
    let vector = Vector2i::new(1, 3);

    assert_approx_eq!(
        vector.aspect(),
        1.0 / 3.0,
        "aspect should work as expected."
    );
    assert_eq!(
        vector.min(&Vector2i::new(3, 2)),
        Vector2i::new(1, 2),
        "min should return expected value."
    );
    assert_eq!(
        vector.max(&Vector2i::new(5, 2)),
        Vector2i::new(5, 3),
        "max should return expected value."
    );
    assert_eq!(
        vector.snapped(&Vector2i::new(4, 2)),
        Vector2i::new(0, 4),
        "snapped should work as expected."
    );
}

#[test]
fn abs_and_sign_methods() {
    let vector1 = Vector2i::new(1, 3);
    let vector2 = Vector2i::new(1, -3);

    assert_eq!(vector1.abs(), vector1, "abs should work as expected.");
    assert_eq!(vector2.abs(), vector1, "abs should work as expected.");
    assert_eq!(
        vector1.sign(),
        Vector2i::new(1, 1),
        "sign should work as expected."
    );
    assert_eq!(
        vector2.sign(),
        Vector2i::new(1, -1),
        "sign should work as expected."
    );
}
