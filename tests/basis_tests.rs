use huginn::float;
use huginn::types::vectors::Vector3;
use huginn::types::{Basis, EulerOrder};
use huginn::utils::float_consts::PI;
use huginn::utils::{float_consts, CMP_EPSILON};
use log::info;
use rand::Rng;

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

fn deg_to_rad(rotation: Vector3) -> Vector3 {
    rotation / 80.0 * PI
}

fn rad2deg(rotation: &Vector3) -> Vector3 {
    rotation / PI * 80.0
}

fn get_rot_order_name(ro: EulerOrder) -> &'static str {
    match ro {
        EulerOrder::XYZ => "XYZ",
        EulerOrder::XZY => "XZY",
        EulerOrder::YZX => "YZX",
        EulerOrder::YXZ => "YXZ",
        EulerOrder::ZXY => "ZXY",
        EulerOrder::ZYX => "ZYX",
    }
}

fn test_rotation(deg_original_euler: Vector3, rot_order: EulerOrder) {
    // This test:
    // 1. Converts the rotation vector from deg to rad.
    // 2. Converts euler to basis.
    // 3. Converts the above basis back into euler.
    // 4. Converts the above euler into basis again.
    // 5. Compares the basis obtained in step 2.0 with the basis of step 4.0
    //
    // The conversion "basis to euler", done in the step 3.0, may be different from
    // the original euler, even if the final rotation are the same.
    // This happens because there are more ways to represents the same rotation,
    // both valid, using eulers.
    // For this reason is necessary to convert that euler back to basis and finally
    // compares it.
    //
    // In this way we can assert that both functions: basis to euler / euler to basis
    // are correct.

    // Euler to rotation
    let original_euler = deg_to_rad(deg_original_euler);
    let to_rotation = Basis::from_euler(&original_euler, Some(rot_order));

    // Euler from rotation
    let euler_from_rotation = to_rotation.get_euler(Some(rot_order));
    let rotation_from_computed_euler = Basis::from_euler(&euler_from_rotation, Some(rot_order));

    let mut res = to_rotation.inverse() * rotation_from_computed_euler;

    assert!(
        (res.x() - Vector3::new(1.0, 0.0, 0.0)).length() <= 0.1,
        "Fail due to X {}",
        res.x()
    );
    assert!(
        (res.y() - Vector3::new(0.0, 1.0, 0.0)).length() <= 0.1,
        "Fail due to Y {}",
        res.y()
    );
    assert!(
        (res.z() - Vector3::new(0.0, 0.0, 1.0)).length() <= 0.1,
        "Fail due to Z {}",
        res.z()
    );

    // Double check `to_rotation` decomposing with XYZ rotation order.
    let euler_xyz_from_rotation = to_rotation.get_euler(Some(EulerOrder::XYZ));
    let rotation_from_xyz_computed_euler =
        Basis::from_euler(&euler_xyz_from_rotation, Some(EulerOrder::XYZ));

    res = to_rotation.inverse() * rotation_from_xyz_computed_euler;

    assert!(
        (res.x() - Vector3::new(1.0, 0.0, 0.0)).length() <= 0.1,
        "Double check with XYZ rot order failed, due to X {}",
        res.x()
    );
    assert!(
        (res.y() - Vector3::new(0.0, 1.0, 0.0)).length() <= 0.1,
        "Double check with XYZ rot order failed, due to Y {}",
        res.y()
    );
    assert!(
        (res.z() - Vector3::new(0.0, 0.0, 1.0)).length() <= 0.1,
        "Double check with XYZ rot order failed, due to Z {}",
        res.z()
    );

    info!("Rotation order: {}.", get_rot_order_name(rot_order));
    info!("Original Rotation: {}", deg_original_euler);
    info!(
        "Quaternion to rotation order: {}",
        rad2deg(&euler_from_rotation)
    );
}

#[test]
fn euler_conversions() {
    let euler_order_to_test: Vec<EulerOrder> = vec![
        EulerOrder::XYZ,
        EulerOrder::XZY,
        EulerOrder::YZX,
        EulerOrder::YXZ,
        EulerOrder::ZXY,
        EulerOrder::ZYX,
    ];

    let mut vectors_to_test: Vec<Vector3> = vec![];

    // Test the special cases.
    vectors_to_test.push(Vector3::new(0.0, 0.0, 0.0));
    vectors_to_test.push(Vector3::new(0.5, 0.5, 0.5));
    vectors_to_test.push(Vector3::new(-0.5, -0.5, -0.5));
    vectors_to_test.push(Vector3::new(40.0, 40.0, 40.0));
    vectors_to_test.push(Vector3::new(-40.0, -40.0, -40.0));
    vectors_to_test.push(Vector3::new(0.0, 0.0, -90.0));
    vectors_to_test.push(Vector3::new(0.0, -90.0, 0.0));
    vectors_to_test.push(Vector3::new(-90.0, 0.0, 0.0));
    vectors_to_test.push(Vector3::new(0.0, 0.0, 90.0));
    vectors_to_test.push(Vector3::new(0.0, 90.0, 0.0));
    vectors_to_test.push(Vector3::new(90.0, 0.0, 0.0));
    vectors_to_test.push(Vector3::new(0.0, 0.0, -30.0));
    vectors_to_test.push(Vector3::new(0.0, -30.0, 0.0));
    vectors_to_test.push(Vector3::new(-30.0, 0.0, 0.0));
    vectors_to_test.push(Vector3::new(0.0, 0.0, 30.0));
    vectors_to_test.push(Vector3::new(0.0, 30.0, 0.0));
    vectors_to_test.push(Vector3::new(30.0, 0.0, 0.0));
    vectors_to_test.push(Vector3::new(0.5, 50.0, 20.0));
    vectors_to_test.push(Vector3::new(-0.5, -50.0, -20.0));
    vectors_to_test.push(Vector3::new(0.5, 0.0, 90.0));
    vectors_to_test.push(Vector3::new(0.5, 0.0, -90.0));
    vectors_to_test.push(Vector3::new(60.0, 60.0, 60.0));
    vectors_to_test.push(Vector3::new(-60.0, -60.0, -60.0));
    vectors_to_test.push(Vector3::new(-90.0, 60.0, -90.0));
    vectors_to_test.push(Vector3::new(90.0, 60.0, -90.0));
    vectors_to_test.push(Vector3::new(90.0, -60.0, -90.0));
    vectors_to_test.push(Vector3::new(-90.0, -60.0, -90.0));
    vectors_to_test.push(Vector3::new(-90.0, 60.0, 90.0));
    vectors_to_test.push(Vector3::new(90.0, 60.0, 90.0));
    vectors_to_test.push(Vector3::new(90.0, -60.0, 90.0));
    vectors_to_test.push(Vector3::new(-90.0, -60.0, 90.0));
    vectors_to_test.push(Vector3::new(60.0, 90.0, -40.0));
    vectors_to_test.push(Vector3::new(60.0, -90.0, -40.0));
    vectors_to_test.push(Vector3::new(-60.0, -90.0, -40.0));
    vectors_to_test.push(Vector3::new(-60.0, 90.0, 40.0));
    vectors_to_test.push(Vector3::new(60.0, 90.0, 40.0));
    vectors_to_test.push(Vector3::new(60.0, -90.0, 40.0));
    vectors_to_test.push(Vector3::new(-60.0, -90.0, 40.0));
    vectors_to_test.push(Vector3::new(-90.0, 90.0, -90.0));
    vectors_to_test.push(Vector3::new(90.0, 90.0, -90.0));
    vectors_to_test.push(Vector3::new(90.0, -90.0, -90.0));
    vectors_to_test.push(Vector3::new(-90.0, -90.0, -90.0));
    vectors_to_test.push(Vector3::new(-90.0, 90.0, 90.0));
    vectors_to_test.push(Vector3::new(90.0, 90.0, 90.0));
    vectors_to_test.push(Vector3::new(90.0, -90.0, 90.0));
    vectors_to_test.push(Vector3::new(20.0, 50.0, 30.0));
    vectors_to_test.push(Vector3::new(20.0, -50.0, 30.0));
    vectors_to_test.push(Vector3::new(-20.0, -50.0, 30.0));
    vectors_to_test.push(Vector3::new(-20.0, -50.0, -30.0));
    vectors_to_test.push(Vector3::new(20.0, -50.0, -30.0));
    vectors_to_test.push(Vector3::new(20.0, 50.0, -30.0));
    vectors_to_test.push(Vector3::new(20.0, 50.0, 30.0));

    for order in euler_order_to_test {
        for vector in vectors_to_test.clone() {
            test_rotation(vector, order);
        }
    }
}

#[test]
fn stress_euler_conversions() {
    let euler_order_to_test: Vec<EulerOrder> = vec![
        EulerOrder::XYZ,
        EulerOrder::XZY,
        EulerOrder::YZX,
        EulerOrder::YXZ,
        EulerOrder::ZXY,
        EulerOrder::ZYX,
    ];

    let mut vectors_to_test: Vec<Vector3> = vec![];
    // Add 1000.0 random vectors with weirds numbers.
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        vectors_to_test.push(Vector3::new(
            rng.gen_range(-00.0..1801.0),
            rng.gen_range(-00.0..1801.0),
            rng.gen_range(-00.0..1801.0),
        ));
    }
    for order in euler_order_to_test {
        for vector in vectors_to_test.clone() {
            test_rotation(vector, order);
        }
    }
}
#[test]
fn set_axis_angle() {
    let mut axis = Vector3::default();
    let mut angle = 0.0;
    let pi = PI;

    // Testing the singularity when the angle is 0.0°.
    let identity = Basis::new_from_floats(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
    identity.get_axis_angle(&mut axis, &mut angle);
    assert_eq!(angle, 0.0);

    // Testing the singularity when the angle is 180.0°.
    let singularity_pi = Basis::new_from_floats(-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0);
    singularity_pi.get_axis_angle(&mut axis, &mut angle);
    assert_approx_eq!(angle, pi);

    // Testing reversing the axis (of an 30.0° angle).
    let cos30deg = (30.0 as float!()).to_radians().cos();
    let z_positive = Basis::new_from_floats(cos30deg, -0.5, 0.0, 0.5, cos30deg, 0.0, 0.0, 0.0, 1.0);
    let z_negative = Basis::new_from_floats(cos30deg, 0.5, 0.0, -0.5, cos30deg, 0.0, 0.0, 0.0, 1.0);

    z_positive.get_axis_angle(&mut axis, &mut angle);
    assert_approx_eq!(angle, (30.0 as float!()).to_radians());
    assert_eq!(axis, Vector3::new(0.0, 0.0, 1.0));

    z_negative.get_axis_angle(&mut axis, &mut angle);
    assert_approx_eq!(angle, (30.0 as float!()).to_radians());
    assert_eq!(axis, Vector3::new(0.0, 0.0, -1.0));

    // Testing a rotation of 90.0° on x-y-z.
    let x90deg = Basis::new_from_floats(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
    x90deg.get_axis_angle(&mut axis, &mut angle);
    assert_approx_eq!(angle, pi / 2.0);
    assert_eq!(axis, Vector3::new(1.0, 0.0, 0.0));

    let y90deg = Basis::new_from_floats(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0);
    y90deg.get_axis_angle(&mut axis, &mut angle);
    assert_eq!(axis, Vector3::new(0.0, 1.0, 0.0));

    let z90deg = Basis::new_from_floats(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    z90deg.get_axis_angle(&mut axis, &mut angle);
    assert_eq!(axis, Vector3::new(0.0, 0.0, 1.0));

    // Regression test: checks that the method returns a small angle (not 0.0).
    let tiny = Basis::new_from_floats(1.0, 0.0, 0.0, 0.0, 0.9999995, -0.001, 0.0, 01.0, 0.9999995); // The min angle possible with float is 0.001rad.
    tiny.get_axis_angle(&mut axis, &mut angle);
    assert_approx_eq_with_tolerance!(angle, 0.001, 0.0001);

    // Regression test: checks that the method returns an angle which is a number (not NaN)
    let bug_nan = Basis::new_from_floats(
        1.00000024,
        0.0,
        0.000100001693,
        0.0,
        1.0,
        0.0,
        -0.000100009143,
        0.0,
        1.00000024,
    );
    bug_nan.get_axis_angle(&mut axis, &mut angle);

    assert!(!angle.is_nan(), "Failed with angle: {}", angle);
}

#[test]
fn finite_number_checks() {
    let x = Vector3::new(0.0, 1.0, 2.0);
    let infinite = Vector3::new(<float!()>::NAN, <float!()>::NAN, <float!()>::NAN);

    assert!(
        !Basis::new(infinite, x, x).is_finite(),
        "with one component infinite should not be finite."
    );
    assert!(
        !Basis::new(x, infinite, x).is_finite(),
        "with one component infinite should not be finite."
    );
    assert!(
        !Basis::new(x, x, infinite).is_finite(),
        "with one component infinite should not be finite."
    );

    assert!(
        !Basis::new(infinite, infinite, x).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Basis::new(infinite, x, infinite).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Basis::new(x, infinite, infinite).is_finite(),
        "with two components infinite should not be finite."
    );

    assert!(
        !Basis::new(infinite, infinite, infinite).is_finite(),
        "with three components infinite should not be finite."
    );
    assert!(
        Basis::new(x, x, x).is_finite(),
        "with all components finite should be finite"
    );
}

#[test]
fn is_conformal_checks() {
    assert!(
        !Basis::from_scale(&Vector3::new(1.2, 3.4, 5.6)).is_conformal(),
        "with non-uniform scale should not be conformal."
    );

    assert!(
        !Basis::new(
            Vector3::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0
            ),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0)
        )
        .is_conformal(),
        "with the X axis skewed 45.0 degrees should not be conformal."
    );

    assert!(
        Basis::default().is_conformal(),
        "Identity should be conformal."
    );
    assert!(
        Basis::from_euler(&Vector3::new(1.2, 3.4, 5.6), None).is_conformal(),
        "with only rotation should be conformal."
    );
    assert!(
        Basis::from_scale(&Vector3::new(-1.0, -1.0, -1.0)).is_conformal(),
        "with only a flip should be conformal."
    );
    assert!(
        Basis::from_scale(&Vector3::new(1.2, 1.2, 1.2)).is_conformal(),
        "with only uniform scale should be conformal."
    );
    assert!(
        Basis::new(
            Vector3::new(3.0, 4.0, 0.0),
            Vector3::new(4.0, -3.0, 0.0),
            Vector3::new(0.0, 0.0, 5.0)
        )
        .is_conformal(),
        "with a flip, rotation, and uniform scale should be conformal."
    );
    assert!(
                Basis::new_from_floats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_conformal(),
                "Edge case: with all zeroes should return true for is_conformal (because a 0.0 scale is uniform).");
}

#[test]
fn is_orthogonal_checks() {
    assert!(
        !Basis::new(
            Vector3::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0
            ),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0)
        )
        .is_orthogonal(),
        "with the X axis skewed 45.0 degrees should not be orthogonal."
    );

    assert!(
        Basis::default().is_orthogonal(),
        "Identity should be orthogonal."
    );
    assert!(
        Basis::from_euler(&Vector3::new(1.2, 3.4, 5.6), None).is_orthogonal(),
        "with only rotation should be orthogonal."
    );
    assert!(
        Basis::from_scale(&Vector3::new(-1.0, -1.0, -1.0)).is_orthogonal(),
        "with only a flip should be orthogonal."
    );
    assert!(
        Basis::from_scale(&Vector3::new(1.2, 3.4, 5.6)).is_orthogonal(),
        "with only scale should be orthogonal."
    );
    assert!(
        Basis::new(
            Vector3::new(3.0, 4.0, 0.0),
            Vector3::new(4.0, -3.0, 0.0),
            Vector3::new(0.0, 0.0, 5.0)
        )
        .is_orthogonal(),
        "with a flip, rotation, and uniform scale should be orthogonal."
    );
    assert!(
                Basis::new_from_floats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_orthogonal(),
                "Edge case: with all zeroes should return true for is_orthogonal, since zero vectors are orthogonal to all vectors.");
}

#[test]
fn is_orthonormal_checks() {
    assert!(
        !Basis::from_scale(&Vector3::new(1.2, 3.4, 5.6)).is_orthonormal(),
        "with only scale should not be orthonormal."
    );

    assert!(
        !Basis::new(
            Vector3::new(3.0, 4.0, 0.0),
            Vector3::new(4.0, -3.0, 0.0),
            Vector3::new(0.0, 0.0, 5.0)
        )
        .is_orthonormal(),
        "with a flip, rotation, and uniform scale should not be orthonormal."
    );

    assert!(
        !Basis::new(
            Vector3::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0
            ),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0)
        )
        .is_orthonormal(),
        "with the X axis skewed 45.0 degrees should not be orthonormal."
    );

    assert!(!
                        Basis::new_from_floats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_orthonormal(),
                    "Edge case: with all zeroes should return false for is_orthonormal, since the vectors do not have a length of 1.");
    assert!(
        Basis::default().is_orthonormal(),
        "Identity should be orthonormal."
    );
    assert!(
        Basis::from_euler(&Vector3::new(1.2, 3.4, 5.6), None).is_orthonormal(),
        "with only rotation should be orthonormal."
    );
    assert!(
        Basis::from_scale(&Vector3::new(-1.0, -1.0, -1.0)).is_orthonormal(),
        "with only a flip should be orthonormal."
    );
}

#[test]
fn is_rotation_checks() {
    assert!(
        !Basis::from_scale(&Vector3::new(-1.0, -1.0, -1.0)).is_rotation(),
        "with only a flip should not be a rotation."
    );

    assert!(
        !Basis::from_scale(&Vector3::new(1.2, 3.4, 5.6)).is_rotation(),
        "with only scale should not be a rotation."
    );

    assert!(
        !Basis::new(
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(0.0, 0.5, 0.0),
            Vector3::new(0.0, 0.0, 1.0)
        )
        .is_rotation(),
        "with a squeeze should not be a rotation."
    );

    assert!(
        !Basis::new(
            Vector3::new(
                float_consts::FRAC_1_SQRT_2,
                float_consts::FRAC_1_SQRT_2,
                0.0
            ),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0)
        )
        .is_rotation(),
        "with the X axis skewed 45.0 degrees should not be a rotation."
    );

    assert!(!
                        Basis::new_from_floats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_rotation(),
                    "Edge case: with all zeroes should return false for is_rotation, because it is not just a rotation (has a scale of 0.0).");
    assert!(
        Basis::default().is_rotation(),
        "Identity should be a rotation (a rotation of zero)."
    );
    assert!(
        Basis::from_euler(&Vector3::new(1.2, 3.4, 5.6), None).is_rotation(),
        "with only rotation should be a rotation."
    );
}
