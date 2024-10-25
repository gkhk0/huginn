use huginn::float;
use huginn::types::vectors::Vector3;
use huginn::types::{Basis, Transform3D};
use huginn::utils::float_consts;

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

fn create_dummy_transform() -> Transform3D {
    Transform3D::new(
        Basis::new_rows(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        ),
        Vector3::new(10.0, 11.0, 12.0),
    )
}

fn identity() -> Transform3D {
    Transform3D::default()
}

#[test]
fn translation() {
    let offset = Vector3::new(1.0, 2.0, 3.0);

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let t = identity().translated(&offset);

    assert_eq!(
        identity().translated(&offset),
        identity().translated_local(&offset)
    );
    assert_eq!(orig.translated(&offset), t * orig);
    assert_eq!(orig.translated_local(&offset), orig * t);
}

#[test]
fn scaling() {
    let scaling = Vector3::new(1.0, 2.0, 3.0);

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let s = identity().scaled(&scaling);

    assert_eq!(
        identity().scaled(&scaling),
        identity().scaled_local(&scaling)
    );
    assert_eq!(orig.scaled(&scaling), s * orig);
    assert_eq!(orig.scaled_local(&scaling), orig * s);
}

#[test]
fn rotation() {
    let axis = Vector3::new(1.0, 2.0, 3.0).normalized();
    let phi = 1.0;

    // Both versions should give the same result applied to identity.

    // Check both versions against left and right multiplications.
    let orig = create_dummy_transform();
    let r = identity().rotated(&axis, phi);

    assert_eq!(
        identity().rotated(&axis, phi),
        identity().rotated_local(&axis, phi)
    );
    assert_eq!(orig.rotated(&axis, phi), r * orig);
    assert_eq!(orig.rotated_local(&axis, phi), orig * r);
}

#[test]
fn finite_number_checks() {
    let y = Vector3::new(0.0, 1.0, 2.0);
    let infinite_vec = Vector3::new(<float!()>::NAN, <float!()>::NAN, <float!()>::NAN);
    let x = Basis::new(y, y, y);
    let infinite_basis = Basis::new(infinite_vec, infinite_vec, infinite_vec);

    assert!(
        !Transform3D::new(x, infinite_vec).is_finite(),
        "Transform3D with one component infinite should not be finite."
    );
    assert!(
        !Transform3D::new(infinite_basis, y).is_finite(),
        "Transform3D with one component infinite should not be finite."
    );

    assert!(
        !Transform3D::new(infinite_basis, infinite_vec).is_finite(),
        "Transform3D with two components infinite should not be finite."
    );
    assert!(
        Transform3D::new(x, y).is_finite(),
        "Transform3D with all components finite should be finite"
    );
}

#[test]
fn rotate_around_global_origin() {
    // Start with the default orientation, but not centered on the origin.
    // Rotating should rotate both our basis and the origin.
    let mut transform = Transform3D::default();
    transform.origin = Vector3::new(0.0, 0.0, 1.0);

    let mut expected = Transform3D::default();
    expected.origin = Vector3::new(0.0, 0.0, -1.0);
    expected.basis.x = Vector3::new(-1.0, 0.0, 0.0);
    expected.basis.z = Vector3::new(0.0, 0.0, -1.0);

    let rotated_transform = transform.rotated(&Vector3::new(0.0, 1.0, 0.0), float_consts::PI);

    println!("!!!!!!!\n{:?}\n{:?}", rotated_transform, expected);
    assert!(
        rotated_transform.is_equal_approx(&expected),
        "The rotated transform should have a new orientation and basis."
    );
}

#[test]
fn rotate_in_place_local_rotation() {
    // Start with the default orientation.
    // Local rotation should not change the origin, only the basis.
    let mut transform = Transform3D::default();
    transform.origin = Vector3::new(1.0, 2.0, 3.0);

    let mut expected = Transform3D::default();
    expected.origin = Vector3::new(1.0, 2.0, 3.0);
    expected.basis.x = Vector3::new(-1.0, 0.0, 0.0);
    expected.basis.z = Vector3::new(0.0, 0.0, -1.0);

    let rotated_transform =
        Transform3D::from(transform.rotated_local(&Vector3::new(0.0, 1.0, 0.0), float_consts::PI));

    assert!(rotated_transform.is_equal_approx(&expected), "The rotated transform should have a new orientation but still be based on the same origin.");
}
