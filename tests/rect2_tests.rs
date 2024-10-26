use huginn::float;
use huginn::types::vectors::Vector2;
use huginn::types::{Rect2, Rect2i};
use huginn::types::Side::Top;
use huginn::utils::{is_zero_approx, CMP_EPSILON};

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
    let rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);
    let rect_vector = Rect2::new(Vector2::new(0.0, 100.0), Vector2::new(1280.0, 720.0));
    let rect_copy_rect = Rect2::from(rect);
    let rect_copy_recti = Rect2::from(Rect2i::new_from_dimension(0, 100, 1280, 720));

    assert_eq!(
        rect, rect_vector,
        "Rect2s created with the same dimensions but by different methods should be equal."
    );
    assert_eq!(
        rect, rect_copy_rect,
        "Rect2s created with the same dimensions but by different methods should be equal."
    );
    assert_eq!(
        rect , rect_copy_recti,
        "Rect2s created with the same dimensions but by different methods should be equal.");
}

#[test]
fn string_conversion() {
    // Note: This also depends on the string representation.

    assert_eq!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0).to_string(),
        "[P: (0, 100), S: (1280, 720)]",
        "The string representation should match the expected value."
    );
}

#[test]
fn basic_getters() {
    let rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);

    assert!(
        rect.position().is_equal_approx(&Vector2::new(0.0, 100.0)),
        "get_position() should return the expected value."
    );
    assert!(
        rect.size().is_equal_approx(&Vector2::new(1280.0, 720.0)),
        "get_size() should return the expected value."
    );
    assert!(
        rect.end().is_equal_approx(&Vector2::new(1280.0, 820.0)),
        "get_end() should return the expected value."
    );
    assert!(
        rect.get_center()
            .is_equal_approx(&Vector2::new(640.0, 460.0)),
        "get_center() should return the expected value."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1281.0, 721.0)
            .get_center()
            .is_equal_approx(&Vector2::new(640.5, 460.5)),
        "get_center() should return the expected value."
    );
}

#[test]
fn basic_setters() {
    let mut rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);
    rect.set_end(Vector2::new(4000.0, 4000.0));

    assert!(
        rect.is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 4000.0, 3900.0)),
        "set_end() should result in the expected Rect2."
    );
    rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);
    rect.set_position(Vector2::new(4000.0, 4000.0));
    assert!(
        rect.is_equal_approx(&Rect2::new_from_dimension(4000.0, 4000.0, 1280.0, 720.0)),
        "set_position() should result in the expected Rect2."
    );
    rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);
    rect.set_size(Vector2::new(4000.0, 4000.0));
    assert!(
        rect.is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 4000.0, 4000.0)),
        "set_size() should result in the expected Rect2."
    );
}

#[test]
fn area_getters() {
    assert_approx_eq!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0).get_area(),
        921_600.0,
        "get_area( should return the expected value."
    );
    assert_approx_eq!(
        Rect2::new_from_dimension(0.0, 100.0, -1280.0, -720.0).get_area(),
        921_600.0,
        "get_area( should return the expected value."
    );
    assert_approx_eq!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, -720.0).get_area(),
        -921_600.0,
        "get_area( should return the expected value."
    );
    assert_approx_eq!(
        Rect2::new_from_dimension(0.0, 100.0, -1280.0, 720.0).get_area(),
        -921_600.0,
        "get_area( should return the expected value."
    );
    assert!(
        is_zero_approx(Rect2::new_from_dimension(0.0, 100.0, 0.0, 720.0).get_area()),
        "get_area() should return the expected value."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0).has_area(),
        "has_area() should return the expected value on Rect2 with an area."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 0.0, 500.0).has_area(),
        "has_area() should return the expected value on Rect2 with no area."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 500.0, 0.0).has_area(),
        "has_area() should return the expected value on Rect2 with no area."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 0.0, 0.0).has_area(),
        "has_area() should return the expected value on Rect2 with no area."
    );
}

#[test]
fn absolute_coordinates() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .abs()
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)),
        "abs() should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, -100.0, 1280.0, 720.0)
            .abs()
            .is_equal_approx(&Rect2::new_from_dimension(0.0, -100.0, 1280.0, 720.0)),
        "abs() should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, -100.0, -1280.0, -720.0)
            .abs()
            .is_equal_approx(&Rect2::new_from_dimension(-1280.0, -820.0, 1280.0, 720.0)),
        "abs() should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, -1280.0, 720.0)
            .abs()
            .is_equal_approx(&Rect2::new_from_dimension(-1280.0, 100.0, 1280.0, 720.0)),
        "abs() should return the expected Rect2."
    );
}

#[test]
fn intersection_1() {
    // The resulting Rect2 is 100.0 pixels high because the first Rect2 is vertically offset by 100.0 pixels.

    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .intersection(&Rect2::new_from_dimension(0.0, 300.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 300.0, 100.0, 100.0)),
        "intersection() with fully enclosed Rect2 should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .intersection(&Rect2::new_from_dimension(1200.0, 700.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::new_from_dimension(1200.0, 700.0, 80.0, 100.0)),
        "intersection() with partially enclosed Rect2 should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .intersection(&Rect2::new_from_dimension(-4000.0, -4000.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::default()),
        "intersection() with non-enclosed Rect2 should return the expected result."
    );
}

#[test]
fn enclosing() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .encloses(&Rect2::new_from_dimension(0.0, 300.0, 100.0, 100.0)),
        "encloses() with fully contained Rect2 should return the expected result."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .encloses(&Rect2::new_from_dimension(1200.0, 700.0, 100.0, 100.0)),
        "encloses() with partially contained Rect2 should return the expected result."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .encloses(&Rect2::new_from_dimension(-4000.0, -4000.0, 100.0, 100.0)),
        "encloses() with non-contained Rect2 should return the expected result."
    );
}

#[test]
fn expanding() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .expand(&Vector2::new(500.0, 600.0))
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)),
        "expand() with contained should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .expand(&Vector2::new(0.0, 0.0))
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 0.0, 1280.0, 820.0)),
        "expand() with non-contained should return the expected result."
    );
}

#[test]
fn get_support() {
    let rect = Rect2::new(Vector2::new(-1.5, 2.0), Vector2::new(4.0, 5.0));

    assert_eq!(
        rect.get_support(&Vector2::new(1.0, 0.0)),
        Vector2::new(2.5, 2.0),
        "get_support() should return the expected value."
    );
    assert_eq!(
        rect.get_support(&Vector2::new(0.5, 1.0)),
        Vector2::new(2.5, 7.0),
        "get_support() should return the expected value."
    );
    assert_eq!(
        rect.get_support(&Vector2::new(0.5, 1.0)),
        Vector2::new(2.5, 7.0),
        "get_support() should return the expected value."
    );
    assert_eq!(
        rect.get_support(&Vector2::new(0.0, -1.0)),
        Vector2::new(-1.5, 2.0),
        "get_support() should return the expected value."
    );
    assert_eq!(
        rect.get_support(&Vector2::new(0.0, -0.1)),
        Vector2::new(-1.5, 2.0),
        "get_support() should return the expected value."
    );
    assert_eq!(
        rect.get_support(&Vector2::default()),
        Vector2::new(-1.5, 2.0),
        "get_support() should return the Rect2 position when given a zero vector."
    );
}

#[test]
fn growing() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow(100.0)
            .is_equal_approx(&Rect2::new_from_dimension(-100.0, 0.0, 1480.0, 920.0)),
        "grow() with positive value should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow(-100.0)
            .is_equal_approx(&Rect2::new_from_dimension(100.0, 200.0, 1080.0, 520.0)),
        "grow() with negative value should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow(-4000.0)
            .is_equal_approx(&Rect2::new_from_dimension(4000.0, 4100.0, -6720.0, -7280.0)),
        "grow() with large negative value should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow_individual(100.0, 200.0, 300.0, 400.0)
            .is_equal_approx(&Rect2::new_from_dimension(-100.0, -100.0, 1680.0, 1320.0)),
        "grow_individual() with positive values should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow_individual(-100.0, 200.0, 300.0, -400.0)
            .is_equal_approx(&Rect2::new_from_dimension(100.0, -100.0, 1480.0, 520.0)),
        "grow_individual() with positive and negative values should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow_side(Top, 500.0)
            .is_equal_approx(&Rect2::new_from_dimension(0.0, -400.0, 1280.0, 1220.0)),
        "grow_side() with positive value should return the expected Rect2."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .grow_side(Top, -500.0)
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 600.0, 1280.0, 220.0)),
        "grow_side() with negative value should return the expected Rect2."
    );
}

#[test]
fn has_point() {
    let mut rect = Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0);

    assert!(
        rect.has_point(&Vector2::new(500.0, 600.0)),
        "has_point() with contained should return the expected result."
    );
    assert!(
        !rect.has_point(&Vector2::new(0.0, 0.0)),
        "has_point() with non-contained should return the expected result."
    );
    assert!(
        rect.has_point(&rect.position()),
        "has_point() with positive size should include `position`."
    );
    assert!(
        rect.has_point(&(rect.position() + Vector2::new(1.0, 1.0))),
        "has_point() with positive size should include `position + (1.0, 1.0)`."
    );
    assert!(
        !rect.has_point(&(rect.position() + Vector2::new(1.0, -1.0))),
        "has_point() with positive size should not include `position + (1.0, -1.0)`."
    );
    assert!(
        !rect.has_point(&(rect.position() + rect.size())),
        "has_point() with positive size should not include `position + size`."
    );
    assert!(
        !rect.has_point(&(rect.position() + rect.size() + Vector2::new(1.0, 1.0))),
        "has_point() with positive size should not include `position + size + (1.0, 1.0)`."
    );
    assert!(
        rect.has_point(&(rect.position() + rect.size() + Vector2::new(-1.0, -1.0))),
        "has_point() with positive size should include `position + size + (-1.0, -1.0)`."
    );
    assert!(
        !rect.has_point(&(rect.position() + rect.size() + Vector2::new(-1.0, 1.0))),
        "has_point() with positive size should not include `position + size + (-1.0, 1.0)`."
    );
    assert!(
        rect.has_point(&(rect.position() + Vector2::new(0.0, 10.0))),
        "has_point() with point located on left edge should return true."
    );
    assert!(
        !rect.has_point(&(rect.position() + Vector2::new(rect.size().x, 10.0))),
        "has_point() with point located on right edge should return false."
    );
    assert!(
        rect.has_point(&(rect.position() + Vector2::new(10.0, 0.0))),
        "has_point() with point located on top edge should return true."
    );
    assert!(
        !rect.has_point(&(rect.position() + Vector2::new(10.0, rect.size().y))),
        "has_point() with point located on bottom edge should return false."
    );

    /*
         // FIXME: Disabled for now until GH-37617.0 is fixed one way or another.
         // More tests should then be written like for the positive size case.
         rect = Rect2::new_from_dimension(0.0, 100.0, -1280.0, -720.0);
    assert!(
         rect.has_point(&rect.position()),
         "has_point() with negative size should include `position`."
     );
     assert!(
         !rect.has_point(&(rect.position() + rect.size())),
         "has_point() with negative size should not include `position + size`."
     );

     */
    rect = Rect2::new_from_dimension(-4000.0, -200.0, 1280.0, 720.0);
    assert!(
        rect.has_point(&(rect.position() + Vector2::new(0.0, 10.0))),
        "has_point() with negative position and point located on left edge should return true."
    );
    assert!(
        !rect.has_point(&(rect.position() + Vector2::new(rect.size().x, 10.0))),
        "has_point() with negative position and point located on right edge should return false."
    );
    assert!(
        rect.has_point(&(rect.position() + Vector2::new(10.0, 0.0))),
        "has_point() with negative position and point located on top edge should return true."
    );
    assert!(
        !rect.has_point(&(rect.position() + Vector2::new(10.0, rect.size().y))),
        "has_point() with negative position and point located on bottom edge should return false."
    );
}

#[test]
fn intersection_2() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .intersects(&Rect2::new_from_dimension(0.0, 300.0, 100.0, 100.0), false),
        "intersects() with fully enclosed Rect2 should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0).intersects(
            &Rect2::new_from_dimension(1200.0, 700.0, 100.0, 100.0),
            false
        ),
        "intersects() with partially enclosed Rect2 should return the expected result."
    );
    assert!(
        !Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0).intersects(
            &Rect2::new_from_dimension(-4000.0, -4000.0, 100.0, 100.0),
            false
        ),
        "intersects() with non-enclosed Rect2 should return the expected result."
    );
}

#[test]
fn merging() {
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .merge(&Rect2::new_from_dimension(0.0, 300.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)),
        "merge() with fully enclosed Rect2 should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .merge(&Rect2::new_from_dimension(1200.0, 700.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::new_from_dimension(0.0, 100.0, 1300.0, 720.0)),
        "merge() with partially enclosed Rect2 should return the expected result."
    );
    assert!(
        Rect2::new_from_dimension(0.0, 100.0, 1280.0, 720.0)
            .merge(&Rect2::new_from_dimension(-4000.0, -4000.0, 100.0, 100.0))
            .is_equal_approx(&Rect2::new_from_dimension(-4000.0, -4000.0, 5280.0, 4820.0)),
        "merge() with non-enclosed Rect2 should return the expected result."
    );
}

#[test]
fn finite_number_checks() {
    let x = Vector2::new(0.0, 1.0);
    let infinite = Vector2::new(<float!()>::NAN, <float!()>::NAN);

    assert!(
        !Rect2::new(infinite, x).is_finite(),
        "Rect2 with one component infinite should not be finite."
    );
    assert!(
        !Rect2::new(x, infinite).is_finite(),
        "Rect2 with one component infinite should not be finite."
    );

    assert!(
        !Rect2::new(infinite, infinite).is_finite(),
        "Rect2 with two components infinite should not be finite."
    );
    assert!(
        Rect2::new(x, x).is_finite(),
        "Rect2 with all components finite should be finite"
    );
}
