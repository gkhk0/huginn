use huginn::types::{Rect2, Rect2i, vectors::Vector2i, Side};
use huginn::types::Side::Top;

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

    let recti = Rect2i::new_from_dimension(0, 100, 1280, 720);
    let recti_vector = Rect2i::new(Vector2i::new(0, 100), Vector2i::new(1280, 720));
    let recti_copy_recti = Rect2i::from(recti);
    let recti_copy_rect = Rect2i::from(Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0));




    assert_eq!(
        recti , recti_vector,
        "Rect2is created with the same dimensions but by different methods should be equal.");
    assert_eq!(
        recti , recti_copy_recti,
        "Rect2is created with the same dimensions but by different methods should be equal.");
    assert_eq!(
        recti , recti_copy_rect,
        "Rect2is created with the same dimensions but by different methods should be equal.");
}

#[test]
fn string_conversion() {

    // Note: This also depends on the string representation.

    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).to_string() , "[P: (0, 100), S: (1280, 720)]",
        "The string representation should match the expected value.");
}

#[test]
fn basic_getters() {

    let rect = Rect2i::new_from_dimension(0, 100, 1280, 720);





    assert_eq!(
        rect.position() , Vector2i::new(0, 100),
        "get_position() should return the expected value.");
    assert_eq!(
        rect.size() , Vector2i::new(1280, 720),
        "get_size() should return the expected value.");
    assert_eq!(
        rect.end() , Vector2i::new(1280, 820),
        "get_end() should return the expected value.");
    assert_eq!(
        rect.get_center() , Vector2i::new(640, 460),
        "get_center() should return the expected value.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1281, 721).get_center() , Vector2i::new(640, 460),
        "get_center() should return the expected value.");
}

#[test]
fn basic_setters() {

    let mut rect = Rect2i::new_from_dimension(0, 100, 1280, 720);
    rect.set_end(Vector2i::new(4000, 4000));
    assert_eq!(
        rect , Rect2i::new_from_dimension(0, 100, 4000, 3900),
        "set_end() should result in the expected Rect2i.");
    
    rect = Rect2i::new_from_dimension(0, 100, 1280, 720);
    rect.set_position(Vector2i::new(4000, 4000));
    assert_eq!(
        rect , Rect2i::new_from_dimension(4000, 4000, 1280, 720),
        "set_position() should result in the expected Rect2i.");
    
    rect = Rect2i::new_from_dimension(0, 100, 1280, 720);
    rect.set_size(Vector2i::new(4000, 4000));
    assert_eq!(
        rect , Rect2i::new_from_dimension(0, 100, 4000, 4000),
        "set_size() should result in the expected Rect2i.");
}

#[test]
fn area_getters() {











    assert_eq!(
			Rect2i::new_from_dimension(0, 100, 1280, 720).get_area() , 921_600,
			"get_area() should return the expected value.");
    assert_eq!(
			Rect2i::new_from_dimension(0, 100, -1280, -720).get_area() , 921_600,
			"get_area() should return the expected value.");
    assert_eq!(
			Rect2i::new_from_dimension(0, 100, 1280, -720).get_area() , -921_600,
			"get_area() should return the expected value.");
    assert_eq!(
			Rect2i::new_from_dimension(0, 100, -1280, 720).get_area() , -921_600,
			"get_area() should return the expected value.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 0, 720).get_area() , 0,
        "get_area() should return the expected value.");
    assert!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).has_area(),
        "has_area() should return the expected value on Rect2i with an area.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 0, 500).has_area(),
        "has_area() should return the expected value on Rect2i with no area.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 500, 0).has_area(),
        "has_area() should return the expected value on Rect2i with no area.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 0, 0).has_area(),
        "has_area() should return the expected value on Rect2i with no area.");
}

#[test]
fn absolute_coordinates() {





    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).abs() , Rect2i::new_from_dimension(0, 100, 1280, 720),
        "abs() should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, -100, 1280, 720).abs() , Rect2i::new_from_dimension(0, -100, 1280, 720),
        "abs() should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, -100, -1280, -720).abs() , Rect2i::new_from_dimension(-1280, -820, 1280, 720),
        "abs() should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, -1280, 720).abs() , Rect2i::new_from_dimension(-1280, 100, 1280, 720),
        "abs() should return the expected Rect2i.");
}

#[test]
fn intersection_1() {


    // The resulting Rect2i is 100 pixels high because the first Rect2i is vertically offset by 100 pixels.


    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).intersection(&Rect2i::new_from_dimension(0, 300, 100, 100)), Rect2i::new_from_dimension(0, 300, 100, 100),
        "intersection() with fully enclosed Rect2i should return the expected result.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).intersection(&Rect2i::new_from_dimension(1200, 700, 100, 100)) , Rect2i::new_from_dimension(1200, 700, 80, 100),
        "intersection() with partially enclosed Rect2i should return the expected result.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).intersection(&Rect2i::new_from_dimension(-4000, -4000, 100, 100)) , Rect2i::default(),
        "intersection() with non-enclosed Rect2i should return the expected result.");
}

#[test]
fn enclosing() {





    assert!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).encloses(&Rect2i::new_from_dimension(0, 300, 100, 100)),
        "encloses() with fully contained Rect2i should return the expected result.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 1280, 720).encloses(&Rect2i::new_from_dimension(1200, 700, 100, 100)),
        "encloses() with partially contained Rect2i should return the expected result.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 1280, 720).encloses(&Rect2i::new_from_dimension(-4000, -4000, 100, 100)),
        "encloses() with non-contained Rect2i should return the expected result.");
    assert!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).encloses(&Rect2i::new_from_dimension(0, 100, 1280, 720)),
        "encloses() with identical Rect2i should return the expected result.");
}

#[test]
fn expanding() {



    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).expand(&Vector2i::new(500, 600)) , Rect2i::new_from_dimension(0, 100, 1280, 720),
        "expand() with contained should return the expected result.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).expand(&Vector2i::new(0, 0)) , Rect2i::new_from_dimension(0, 0, 1280, 820),
        "expand() with non-contained should return the expected result.");
}

#[test]
fn growing() {










    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow(100) , Rect2i::new_from_dimension(-100, 0, 1480, 920),
        "grow() with positive value should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow(-100) , Rect2i::new_from_dimension(100, 200, 1080, 520),
        "grow() with negative value should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow(-4000) , Rect2i::new_from_dimension(4000, 4100, -6720, -7280),
        "grow() with large negative value should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow_individual(100, 200, 300, 400) , Rect2i::new_from_dimension(-100, -100, 1680, 1320),
        "grow_individual() with positive values should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow_individual(-100, 200, 300, -400) , Rect2i::new_from_dimension(100, -100, 1480, 520),
        "grow_individual() with positive and negative values should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow_side(Side::Top, 500) , Rect2i::new_from_dimension(0, -400, 1280, 1220),
        "grow_side() with positive value should return the expected Rect2i.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).grow_side(Top, -500) , Rect2i::new_from_dimension(0, 600, 1280, 220),
        "grow_side() with negative value should return the expected Rect2i.");
}

#[test]
fn has_point() {

    let mut rect = Rect2i::new_from_dimension(0, 100, 1280, 720);
















    




    assert!(
        rect.has_point(&Vector2i::new(500, 600)),
        "has_point() with contained should return the expected result.");
    assert!(
        !rect.has_point(&Vector2i::new(0, 0)),
        "has_point() with non-contained should return the expected result.");
    assert!(
        rect.has_point(&rect.position()),
        "has_point() with positive size should include `position`.");
    assert!(
        rect.has_point(&(rect.position() + Vector2i::new(1, 1))),
        "has_point() with positive size should include `position + (1, 1)`.");
    assert!(
        !rect.has_point(&(rect.position() + Vector2i::new(1, -1))),
        "has_point() with positive size should not include `position + (1, -1)`.");
    assert!(
        !rect.has_point(&(rect.position() + rect.size())),
        "has_point() with positive size should not include `position + size`.");
    assert!(
        !rect.has_point(&(rect.position() + rect.size() + Vector2i::new(1, 1))),
        "has_point() with positive size should not include `position + size + (1, 1)`.");
    assert!(
        rect.has_point(&(rect.position() + rect.size() + Vector2i::new(-1, -1))),
        "has_point() with positive size should include `position + size + (-1, -1)`.");
    assert!(
        !rect.has_point(&(rect.position() + rect.size() + Vector2i::new(-1, 1))),
        "has_point() with positive size should not include `position + size + (-1, 1)`.");
    assert!(
        rect.has_point(&(rect.position() + Vector2i::new(0, 10))),
        "has_point() with point located on left edge should return true.");
    assert!(
        !rect.has_point(&(rect.position() + Vector2i::new(rect.size().x, 10))),
        "has_point() with point located on right edge should return false.");
    assert!(
        rect.has_point(&(rect.position() + Vector2i::new(10, 0))),
        "has_point() with point located on top edge should return true.");
    assert!(
        !rect.has_point(&(rect.position() + Vector2i::new(10, rect.size().y))),
        "has_point() with point located on bottom edge should return false.");
    
    /*
    // FIXME: Disabled for now until GH-37617 is fixed one way or another.
    // More tests should then be written like for the positive size case.
    rect = Rect2i(0, 100, -1280, -720);
    assert!(
        rect.has_point(&rect.position()),
        "has_point() with negative size should include `position`.");
    assert!(
        !rect.has_point(&(rect.position() + rect.size())),
        "has_point() with negative size should not include `position + size`.");
    */

    rect = Rect2i::new_from_dimension(-4000, -200, 1280, 720);
    assert!(
        rect.has_point(&(rect.position() + Vector2i::new(0, 10))),
        "has_point() with negative position and point located on left edge should return true.");
    assert!(
        !rect.has_point(&(rect.position() + Vector2i::new(rect.size().x, 10))),
        "has_point() with negative position and point located on right edge should return false.");
    assert!(
        rect.has_point(&(rect.position() + Vector2i::new(10, 0))),
        "has_point() with negative position and point located on top edge should return true.");
    assert!(
        !rect.has_point(&(rect.position() + Vector2i::new(10, rect.size().y))),
        "has_point() with negative position and point located on bottom edge should return false.");
}

#[test]
fn intersection_2() {





    assert!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).intersects(&Rect2i::new_from_dimension(0, 300, 100, 100)),
        "intersects() with fully enclosed Rect2i should return the expected result.");
    assert!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).intersects(&Rect2i::new_from_dimension(1200, 700, 100, 100)),
        "intersects() with partially enclosed Rect2i should return the expected result.");
    assert!(
        !Rect2i::new_from_dimension(0, 100, 1280, 720).intersects(&Rect2i::new_from_dimension(-4000, -4000, 100, 100)),
        "intersects() with non-enclosed Rect2i should return the expected result.");
    assert!(
        !Rect2i::new_from_dimension(0, 0, 2, 2).intersects(&Rect2i::new_from_dimension(2, 2, 2, 2)),
        "intersects() with adjacent Rect2i should return the expected result.");
}

#[test]
fn merging() {




    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).merge(&Rect2i::new_from_dimension(0, 300, 100, 100)), Rect2i::new_from_dimension(0, 100, 1280, 720),
        "merge() with fully enclosed Rect2i should return the expected result.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).merge(&Rect2i::new_from_dimension(1200, 700, 100, 100)), Rect2i::new_from_dimension(0, 100, 1300, 720),
        "merge() with partially enclosed Rect2i should return the expected result.");
    assert_eq!(
        Rect2i::new_from_dimension(0, 100, 1280, 720).merge(&Rect2i::new_from_dimension(-4000, -4000, 100, 100)), Rect2i::new_from_dimension(-4000, -4000, 5280, 4820),
        "merge() with non-enclosed Rect2i should return the expected result.");
}
