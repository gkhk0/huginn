use huginn::float;
use huginn::types::{vectors::Vector2, Transform2D};
use huginn::utils::{float_consts, is_equal_approx, CMP_EPSILON};

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

fn create_dummy_transform() -> Transform2D {
    Transform2D::new(
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(5.0, 6.0),
    )
}

fn identity() -> Transform2D {
    Transform2D::default()
}

#[test]
fn default_letructor() {
    let default_letructor = Transform2D::default();

    assert_eq!(
        default_letructor,
        Transform2D::new(
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(0.0, 0.0)
        )
    );
}

#[test]
fn copy_letructor() {
    let T = create_dummy_transform();
    let copy_letructor = Transform2D::from(T);

    assert_eq!(T, copy_letructor);
}

#[test]
fn constructor_from_angle_and_position() {
    let ROTATION = float_consts::PI / 4.0;
    let TRANSLATION = Vector2::new(20.0, -20.0);

    let test = Transform2D::from((ROTATION, TRANSLATION));
    let expected = Transform2D::default()
        .rotated(ROTATION)
        .translated(&TRANSLATION);

    assert_eq!(test, expected);
}

#[test]
fn constructor_from_angle_scale_skew_and_position() {
    let rotation = float_consts::PI / 2.0;
    let scale = Vector2::new(2.0, 0.5);
    let skew = float_consts::PI / 4.0;
    let translation = Vector2::new(30.0, 0.0);

    let test = Transform2D::from((rotation, scale, skew, translation));
    let mut expected = Transform2D::default()
        .scaled(&scale)
        .rotated(rotation)
        .translated(&translation);
    expected.set_skew(skew);

    println!("{:?}\n{:?}", test, expected);
    assert!(test.is_equal_approx(&expected));
}

#[test]
fn constructor_from_raw_values() {
    let test = Transform2D::new_from_floats(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let expected = Transform2D::new(
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(5.0, 6.0),
    );

    assert_eq!(test, expected);
}

#[test]
fn xform() {
    let v = Vector2::new(2.0, 3.0);
    let T = Transform2D::new(
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(5.0, 6.0),
    );
    let expected = Vector2::new(
        1.0 * 2.0 + 3.0 * 3.0 + 5.0 * 1.0,
        2.0 * 2.0 + 4.0 * 3.0 + 6.0 * 1.0,
    );

    assert_eq!(T.xform(&v), expected);
}

#[test]
fn basis_xform() {
    let v = Vector2::new(2.0, 2.0);
    let T1 = Transform2D::new(
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(0.0, 0.0),
    );

    // Both versions should be the same when the origin is (0,0).

    let T2 = Transform2D::new(
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(5.0, 6.0),
    );

    // Each version should be different when the origin is not (0,0).
    assert_ne!(T2.basis_xform(&v), T2.xform(&v));
    assert_eq!(T1.basis_xform(&v), T1.xform(&v));
}

#[test]
fn affine_inverse() {
    let orig = create_dummy_transform();
    let affine_inverted = orig.affine_inverse();
    let affine_inverted_again = affine_inverted.affine_inverse();

    assert_eq!(affine_inverted_again, orig);
}

#[test]
fn orthonormalized() {
    let T = create_dummy_transform();
    let orthonormalized_T = T.orthonormalized();

    // Check each basis has length 1.

    let vx = Vector2::new(orthonormalized_T.x.x, orthonormalized_T.y.x);
    let vy = Vector2::new(orthonormalized_T.x.y, orthonormalized_T.y.y);

    // Check the basis are orthogonal.

    assert!(is_equal_approx(orthonormalized_T.x.length_squared(), 1.0));
    assert!(is_equal_approx(orthonormalized_T.y.length_squared(), 1.0));
    assert!(is_equal_approx(orthonormalized_T.tdotx(&vx), 1.0));
    assert!(is_equal_approx(orthonormalized_T.tdotx(&vy), 0.0));
    assert!(is_equal_approx(orthonormalized_T.tdoty(&vx), 0.0));
    assert!(is_equal_approx(orthonormalized_T.tdoty(&vy), 1.0));
}

#[test]
fn translation() {
    let offset = Vector2::new(1.0, 2.0);

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let T = identity().translated(&offset);

    assert_eq!(
        identity().translated(&offset),
        identity().translated_local(&offset)
    );
    assert_eq!(orig.translated(&offset), T * orig);
    assert_eq!(orig.translated_local(&offset), orig * T);
}

#[test]
fn scaling() {
    let scaling = Vector2::new(1.0, 2.0);

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let S = identity().scaled(&scaling);

    assert_eq!(
        identity().scaled(&scaling),
        identity().scaled_local(&scaling)
    );
    assert_eq!(orig.scaled(&scaling), S * orig);
    assert_eq!(orig.scaled_local(&scaling), orig * S);
}

#[test]
fn rotation() {
    let phi = 1.0;

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let R = identity().rotated(phi);

    assert_eq!(identity().rotated(phi), identity().rotated_local(phi));
    assert_eq!(orig.rotated(phi), R * orig);
    assert_eq!(orig.rotated_local(phi), orig * R);
}

#[test]
fn interpolation() {
    let rotate_scale_skew_pos = Transform2D::from((
        (170.0 as float!()).to_radians(),
        Vector2::new(3.6, 8.0),
        (20.0 as float!()).to_radians(),
        Vector2::new(2.4, 6.8),
    ));
    let rotate_scale_skew_pos_halfway = Transform2D::from((
        (85.0 as float!()).to_radians(),
        Vector2::new(2.3, 4.5),
        (10.0 as float!()).to_radians(),
        Vector2::new(1.2, 3.4),
    ));
    let mut interpolated = Transform2D::default().interpolate_with(&rotate_scale_skew_pos, 0.5);

    assert!(interpolated
        .get_origin()
        .is_equal_approx(&rotate_scale_skew_pos_halfway.get_origin()));
    assert_approx_eq!(
        interpolated.get_rotation(),
        rotate_scale_skew_pos_halfway.get_rotation()
    );
    assert!(interpolated
        .get_scale()
        .is_equal_approx(&rotate_scale_skew_pos_halfway.get_scale()));
    assert_approx_eq!(
        interpolated.get_skew(),
        rotate_scale_skew_pos_halfway.get_skew()
    );
    assert!(interpolated.is_equal_approx(&rotate_scale_skew_pos_halfway));
    interpolated = rotate_scale_skew_pos.interpolate_with(&Transform2D::default(), 0.5);
    assert!(interpolated.is_equal_approx(&rotate_scale_skew_pos_halfway));
}

#[test]
fn finite_number_checks() {
    let x = Vector2::new(0.0, 1.0);
    let infinite = Vector2::new(<float!()>::NAN, <float!()>::NAN);

    assert!(
        !Transform2D::new(infinite, x, x).is_finite(),
        "Transform2D with one component infinite should not be finite."
    );
    assert!(
        !Transform2D::new(x, infinite, x).is_finite(),
        "Transform2D with one component infinite should not be finite."
    );
    assert!(
        !Transform2D::new(x, x, infinite).is_finite(),
        "Transform2D with one component infinite should not be finite."
    );

    assert!(
        !Transform2D::new(infinite, infinite, x).is_finite(),
        "Transform2D with two components infinite should not be finite."
    );
    assert!(
        !Transform2D::new(infinite, x, infinite).is_finite(),
        "Transform2D with two components infinite should not be finite."
    );
    assert!(
        !Transform2D::new(x, infinite, infinite).is_finite(),
        "Transform2D with two components infinite should not be finite."
    );

    assert!(
        !Transform2D::new(infinite, infinite, infinite).is_finite(),
        "Transform2D with three components infinite should not be finite."
    );
    assert!(
        Transform2D::new(x, x, x).is_finite(),
        "Transform2D with all components finite should be finite"
    );
}

#[test]
fn is_conformal_checks() {
    assert!(
        !Transform2D::new(
            Vector2::new(1.2, 0.0),
            Vector2::new(0.0, 3.4),
            Vector2::default()
        )
        .is_conformal(),
        "Transform2D with non-uniform scale should not be conformal."
    );

    assert!(
        !Transform2D::new(
            Vector2::new(float_consts::FRAC_1_SQRT_2, float_consts::FRAC_1_SQRT_2),
            Vector2::new(0.0, 1.0),
            Vector2::default()
        )
        .is_conformal(),
        "Transform2D with the X axis skewed 45 degrees should not be conformal."
    );
    assert!(
        Transform2D::default().is_conformal(),
        "Identity Transform2D should be conformal."
    );
    assert!(
        Transform2D::from((1.2, Vector2::default())).is_conformal(),
        "Transform2D with only rotation should be conformal."
    );
    assert!(
        Transform2D::new(
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, -1.0),
            Vector2::default()
        )
        .is_conformal(),
        "Transform2D with only a flip should be conformal."
    );
    assert!(
        Transform2D::new(
            Vector2::new(1.2, 0.0),
            Vector2::new(0.0, 1.2),
            Vector2::default()
        )
        .is_conformal(),
        "Transform2D with only uniform scale should be conformal."
    );
    assert!(
        Transform2D::new(
            Vector2::new(1.2, 3.4),
            Vector2::new(3.4, -1.2),
            Vector2::default()
        )
        .is_conformal(),
        "Transform2D with a flip, rotation, and uniform scale should be conformal."
    );
}
