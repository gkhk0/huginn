use huginn::int;
use huginn::types::vectors::{Vector4, Vector4i, AXIS};
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

    let vector_empty = Vector4i::default();
    let vector_zero = Vector4i::new(0, 0, 0, 0);

    assert_eq!(
        vector_empty , vector_zero,
        "Constructor with no inputs should return a zero Vector4i.");
}

#[test]
fn axis_methods() {

    let mut vector = Vector4i::new(1, 2, 3, 4);






    assert_eq!(
        vector.max_axis_index() , AXIS::W,
        "max_axis_index should work as expected.");
    assert_eq!(
        vector.min_axis_index() , AXIS::X,
        "min_axis_index should work as expected.");
    assert_eq!(
        vector.get_axis(vector.max_axis_index()) , 4,
        "array operator should work as expected.");
    assert_eq!(
        vector.get_axis(vector.min_axis_index()) , 1,
        "array operator should work as expected.");
    vector.set_axis(AXIS::Y, 5);
    assert_eq!(
        vector.get_axis(AXIS::Y) , 5,
        "array operator setter should work as expected.");
}

#[test]
fn clamp_method() {

    let vector = Vector4i::new(10, 10, 10, 10);


    assert_eq!(
        Vector4i::new(-5, 5, 15, <int!()>::MAX).clamp(&Vector4i::default(), &vector) , Vector4i::new(0, 5, 10, 10),
        "clamp should work as expected.");
    assert_eq!(
        vector.clamp(&Vector4i::new(0, 10, 15, -10), &Vector4i::new(5, 10, 20, -5)) , Vector4i::new(5, 10, 15, -5),
        "clamp should work as expected.");
}

#[test]
fn length_methods() {

    let vector1 = Vector4i::new(10, 10, 10, 10);
    let vector2 = Vector4i::new(20, 30, 40, 50);






    assert_eq!(
        vector1.length_squared() , 400,
        "length_squared should work as expected and return exact result.");
    assert_approx_eq!(
			vector1.length() , 20.0,
			"length should work as expected.");
    assert_eq!(
        vector2.length_squared() , 5400,
        "length_squared should work as expected and return exact result.");
    assert_approx_eq!(
			vector2.length() , 73.4846922835,
			"length should work as expected.");
    assert_eq!(
        vector1.distance_squared_to(&vector2) , 3000,
        "distance_squared_to should work as expected.");
    assert_approx_eq!(
			vector1.distance_to(&vector2) , 54.772255750517,
			"distance_to should work as expected.");
}

#[test]
fn operators() {

    let vector1 = Vector4i::new(4, 5, 9, 2);
    let vector2 = Vector4i::new(1, 2, 3, 4);













    assert_eq!(
        -vector1 , Vector4i::new(-4, -5, -9, -2),
        "change of sign should work as expected.");
    assert_eq!(
        &vector1 + vector2, Vector4i::new(5, 7, 12, 6),
        "addition with integers should give exact results.");
    assert_eq!(
        &vector1 - vector2, Vector4i::new(3, 3, 6, -2),
        "subtraction with integers should give exact results.");
    assert_eq!(
        &vector1 * vector2, Vector4i::new(4, 10, 27, 8),
        "multiplication with integers should give exact results.");
    assert_eq!(
        &vector1 / vector2, Vector4i::new(4, 2, 3, 0),
        "division with integers should give exact results.");
    assert_eq!(
        &vector1 * 2, Vector4i::new(8, 10, 18, 4),
        "multiplication with integers should give exact results.");
    assert_eq!(
        &vector1 / 2, Vector4i::new(2, 2, 4, 1),
        "division with integers should give exact results.");
    assert_eq!(
			Vector4::from(vector1) , Vector4::new(4.0, 5.0, 9.0, 2.0),
			"cast to should work as expected.");
    assert_eq!(
			Vector4::from(vector2) , Vector4::new(1.0, 2.0, 3.0, 4.0),
			"cast to should work as expected.");
    assert_eq!(
        Vector4i::from(Vector4::new(1.1, 2.9, 3.9, 100.5)) , Vector4i::new(1, 2, 3, 100),
        "constructed from should work as expected.");
}

#[test]
fn abs_and_sign_methods() {

    let vector1 = Vector4i::new(1, 3, 5, 7);
    let vector2 = Vector4i::new(1, -3, -5, 7);





    assert_eq!(
        vector1.abs() , vector1,
        "abs should work as expected.");
    assert_eq!(
        vector2.abs() , vector1,
        "abs should work as expected.");
    assert_eq!(
        vector1.sign() , Vector4i::new(1, 1, 1, 1),
        "sign should work as expected.");
    assert_eq!(
        vector2.sign() , Vector4i::new(1, -1, -1, 1),
        "sign should work as expected.");
}
