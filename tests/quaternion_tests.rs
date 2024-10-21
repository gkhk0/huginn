use huginn::types::{vectors::Vector3, Basis, EulerOrder, Quaternion};
use huginn::utils::{float, CMP_EPSILON};

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

fn quat_euler_yxz_deg(angle: Vector3) -> Quaternion {
    let yaw = angle.y.to_radians();
    let pitch = angle.x.to_radians();
    let roll = angle.z.to_radians();

    // Generate YXZ (Z-then-X-then-Y) Quaternion using single-axis Euler
    // constructor and quaternion product, both tested separately.
    let q_y = Quaternion::from_euler(&Vector3::new(0.0, yaw, 0.0));
    let q_p = Quaternion::from_euler(&Vector3::new(pitch, 0.0, 0.0));
    let q_r = Quaternion::from_euler(&Vector3::new(0.0, 0.0, roll));
    // Roll-Z is followed by Pitch-X, then Yaw-Y.
    q_y * q_p * q_r
}

#[test]
fn default_construct() {
    let q = Quaternion::default();

    assert_eq!(q.x, 0.0);
    assert_eq!(q.y, 0.0);
    assert_eq!(q.z, 0.0);
    assert_eq!(q.w, 1.0);
}

#[test]
fn construct_xyzw() {
    // Values are taken from actual use in another project & are valid (except roundoff error).
    let q = Quaternion::new(0.2391, 0.099, 0.3696, 0.8924);

    assert_approx_eq!(q.x, 0.2391);
    assert_approx_eq!(q.y, 0.099);
    assert_approx_eq!(q.z, 0.3696);
    assert_approx_eq!(q.w, 0.8924);
}

#[test]
fn construct_axis_angle_1() {
    // Easy to visualize: 120 deg about X-axis.
    let q = Quaternion::from((
        &Vector3::new(1.0, 0.0, 0.0),
        (120.0 as float!()).to_radians(),
    ));

    // 0.866 isn't close enough; doctest::Approx doesn't cut much slack!
    // Sine of half the angle.

    // Cosine of half the angle.
    assert_approx_eq!(q.x, 0.866025);
    assert_approx_eq!(q.y, 0.0);
    assert_approx_eq!(q.z, 0.0);
    assert_approx_eq!(q.w, 0.5);
}

#[test]
fn construct_axis_angle_2() {
    // Easy to visualize: 30 deg about Y-axis.
    let q = Quaternion::from((
        &Vector3::new(0.0, 1.0, 0.0),
        (30.0 as float!()).to_radians(),
    ));

    // Sine of half the angle.

    // Cosine of half the angle.
    assert_approx_eq!(q.x, 0.0);
    assert_approx_eq!(q.y, 0.258819);
    assert_approx_eq!(q.z, 0.0);
    assert_approx_eq!(q.w, 0.965926);
}

#[test]
fn construct_axis_angle_3() {
    // Easy to visualize: 60 deg about Z-axis.
    let q = Quaternion::from((
        &Vector3::new(0.0, 0.0, 1.0),
        (60.0 as float!()).to_radians(),
    ));

    // Sine of half the angle.
    // Cosine of half the angle.
    assert_approx_eq!(q.x, 0.0);
    assert_approx_eq!(q.y, 0.0);
    assert_approx_eq!(q.z, 0.5);
    assert_approx_eq!(q.w, 0.866025);
}

#[test]
fn construct_axis_angle_4() {
    // More complex & hard to visualize, so test w/ data from online calculator.
    let axis = Vector3::new(1.0, 2.0, 0.5);
    let q = Quaternion::from((&axis.normalized(), (35.0 as float!()).to_radians()));

    assert_approx_eq!(q.x, 0.131239);
    assert_approx_eq!(q.y, 0.262478);
    assert_approx_eq!(q.z, 0.0656194);
    assert_approx_eq!(q.w, 0.953717);
}

#[test]
fn construct_from_quaternion() {
    let axis = Vector3::new(1.0, 2.0, 0.5);
    let q_src = Quaternion::from((&axis.normalized(), (35.0 as float!()).to_radians()));
    let q = Quaternion::from(q_src);

    assert_approx_eq!(q.x, 0.131239);
    assert_approx_eq!(q.y, 0.262478);
    assert_approx_eq!(q.z, 0.0656194);
    assert_approx_eq!(q.w, 0.953717);
}

#[test]
fn construct_euler_single_axis() {
    let yaw = (45.0 as float!()).to_radians();
    let pitch = (30.0 as float!()).to_radians();
    let roll = (10.0 as float!()).to_radians();

    let euler_y = Vector3::new(0.0, yaw, 0.0);
    let q_y = Quaternion::from_euler(&euler_y);

    let euler_p = Vector3::new(pitch, 0.0, 0.0);
    let q_p = Quaternion::from_euler(&euler_p);

    let euler_r = Vector3::new(0.0, 0.0, roll);
    let q_r = Quaternion::from_euler(&euler_r);

    assert_approx_eq!(q_y.x, 0.0);
    assert_approx_eq!(q_y.y, 0.382684);
    assert_approx_eq!(q_y.z, 0.0);
    assert_approx_eq!(q_y.w, 0.923879);
    assert_approx_eq!(q_p.x, 0.258819);
    assert_approx_eq!(q_p.y, 0.0);
    assert_approx_eq!(q_p.z, 0.0);
    assert_approx_eq!(q_p.w, 0.965926);
    assert_approx_eq!(q_r.x, 0.0);
    assert_approx_eq!(q_r.y, 0.0);
    assert_approx_eq!(q_r.z, 0.0871558);
    assert_approx_eq!(q_r.w, 0.996195);
}

#[test]
fn construct_euler_yxz_dynamic_axes() {
    let yaw = (45.0 as float!()).to_radians();
    let pitch = (30.0 as float!()).to_radians();
    let roll = (10.0 as float!()).to_radians();

    // Generate YXZ comparison data (Z-then-X-then-Y) using single-axis Euler
    // constructor and quaternion product, both tested separately.
    let euler_y = Vector3::new(0.0, yaw, 0.0);
    let q_y = Quaternion::from_euler(&euler_y);
    let euler_p = Vector3::new(pitch, 0.0, 0.0);
    let q_p = Quaternion::from_euler(&euler_p);
    let euler_r = Vector3::new(0.0, 0.0, roll);
    let q_r = Quaternion::from_euler(&euler_r);

    // Instrinsically, Yaw-Y then Pitch-X then Roll-Z.
    // Extrinsically, Roll-Z is followed by Pitch-X, then Yaw-Y.
    let check_yxz = q_y * q_p * q_r;

    // Test construction from YXZ Euler angles.
    let euler_yxz = Vector3::new(pitch, yaw, roll);
    let q = Quaternion::from_euler(&euler_yxz);

    assert_approx_eq!(q.x, check_yxz.x);
    assert_approx_eq!(q.y, check_yxz.y);
    assert_approx_eq!(q.z, check_yxz.z);
    assert_approx_eq!(q.w, check_yxz.w);
    assert!(q.is_equal_approx(&check_yxz));
    assert!(q.get_euler(None).is_equal_approx(&euler_yxz));
    assert!(check_yxz.get_euler(None).is_equal_approx(&euler_yxz));
}

#[test]
fn construct_euler() {
    let yaw = (45.0 as float!()).to_radians();
    let pitch = (30.0 as float!()).to_radians();
    let roll = (10.0 as float!()).to_radians();
    let euler_yxz = Vector3::new(pitch, yaw, roll);
    let q_yxz = Quaternion::from_euler(&euler_yxz);
    let basis_axes = Basis::from_euler(&euler_yxz, None);
    let q = Quaternion::from(&basis_axes);

    assert!(q.is_equal_approx(&q_yxz));
}

#[test]
fn construct_axes() {
    // Arbitrary Euler angles.
    let euler_yxz = Vector3::new(
        (31.41 as float!()).to_radians(),
        (-49.16 as float!()).to_radians(),
        (12.34 as float!()).to_radians(),
    );
    // vectors from online calculation of rotation matrix.
    let i_unit = Vector3::new(0.5545787, 0.1823950, 0.8118957);
    let j_unit = Vector3::new(-0.5249245, 0.8337420, 0.1712555);
    let k_unit = Vector3::new(-0.6456754, -0.5211586, 0.5581192);
    // from online calculation.
    let q_calc = Quaternion::new(0.2016913, -0.4245716, 0.206033, 0.8582598);
    // from local calculation.
    let q_local = quat_euler_yxz_deg(Vector3::new(31.41, -49.16, 12.34));
    // from Euler angles constructor.
    let q_euler = Quaternion::from_euler(&euler_yxz);

    // Calculate and construct Quaternion.
    // When this is written, it does not construct from basis vectors.
    // This is by design, but may be subject to change.
    // Workaround by constructing from Euler angles.
    // basis_axes = Basis::new(i_unit, j_unit, k_unit);
    let basis_axes = Basis::from_euler(&euler_yxz, None);
    let q = Quaternion::from(&basis_axes);

    assert!(!q.inverse().is_equal_approx(&q_calc));

    assert!(q_calc.is_equal_approx(&q_local));
    assert!(q_local.is_equal_approx(&q_euler));
    assert!(basis_axes.get_column(0).is_equal_approx(&i_unit));
    assert!(basis_axes.get_column(1).is_equal_approx(&j_unit));
    assert!(basis_axes.get_column(2).is_equal_approx(&k_unit));
    assert!(q.is_equal_approx(&q_calc));
    assert!(q.is_equal_approx(&q_local));
    assert!(q.is_equal_approx(&q_euler));
    assert_approx_eq!(q.x, 0.2016913);
    assert_approx_eq!(q.y, -0.4245716);
    assert_approx_eq!(q.z, 0.206033);
    assert_approx_eq!(q.w, 0.8582598);
}

#[test]
fn get_euler_orders() {
    let x = (30.0 as float!()).to_radians();
    let y = (45.0 as float!()).to_radians();
    let z = (10.0 as float!()).to_radians();
    let euler = Vector3::new(x, y, z);
    for order in [
        EulerOrder::YXZ,
        EulerOrder::XYZ,
        EulerOrder::XZY,
        EulerOrder::YZX,
        EulerOrder::ZXY,
        EulerOrder::ZYX,
    ] {
        let basis = Basis::from_euler(&euler, Some(order));
        let q = Quaternion::from(&basis);
        let check = q.get_euler(Some(order));

        assert!(
            check.is_equal_approx(&euler),
            "get_euler method should return the original angles."
        );
        assert!(
            check.is_equal_approx(&basis.get_euler(Some(order))),
            "get_euler method should behave the same as get_euler."
        );
    }
}
#[test]
fn product_book() {
    // Example from "Quaternions and Rotation Sequences" by Jack Kuipers, p. 108.
    let p = Quaternion::new(1.0, -2.0, 1.0, 3.0);
    let q = Quaternion::new(-1.0, 2.0, 3.0, 2.0);

    let pq = p * q;

    assert_approx_eq!(pq.x, -9.0);
    assert_approx_eq!(pq.y, -2.0);
    assert_approx_eq!(pq.z, 11.0);
    assert_approx_eq!(pq.w, 8.0);
}

#[test]
fn product() {
    let yaw = (45.0 as float!()).to_radians();
    let pitch = (30.0 as float!()).to_radians();
    let roll = (10.0 as float!()).to_radians();

    let euler_y = Vector3::new(0.0, yaw, 0.0);
    let q_y = Quaternion::from_euler(&euler_y);

    let euler_p = Vector3::new(pitch, 0.0, 0.0);
    let q_p = Quaternion::from_euler(&euler_p);

    let euler_r = Vector3::new(0.0, 0.0, roll);
    let q_r = Quaternion::from_euler(&euler_r);

    // Test ZYX dynamic-axes since test data is available online.
    // Rotate first about X axis, then new Y axis, then new Z axis.
    // (Godot uses YXZ Yaw-Pitch-Roll order).
    let q_yp = q_y * q_p;

    let q_ryp = q_r * q_yp;

    assert_approx_eq!(q_y.x, 0.0);
    assert_approx_eq!(q_y.y, 0.382684);
    assert_approx_eq!(q_y.z, 0.0);
    assert_approx_eq!(q_y.w, 0.923879);
    assert_approx_eq!(q_p.x, 0.258819);
    assert_approx_eq!(q_p.y, 0.0);
    assert_approx_eq!(q_p.z, 0.0);
    assert_approx_eq!(q_p.w, 0.965926);
    assert_approx_eq!(q_r.x, 0.0);
    assert_approx_eq!(q_r.y, 0.0);
    assert_approx_eq!(q_r.z, 0.0871558);
    assert_approx_eq!(q_r.w, 0.996195);
    assert_approx_eq!(q_yp.x, 0.239118);
    assert_approx_eq!(q_yp.y, 0.369644);
    assert_approx_eq!(q_yp.z, -0.099046);
    assert_approx_eq!(q_yp.w, 0.892399);
    assert_approx_eq!(q_ryp.x, 0.205991);
    assert_approx_eq!(q_ryp.y, 0.389078);
    assert_approx_eq!(q_ryp.z, -0.0208912);
    assert_approx_eq!(q_ryp.w, 0.897636);
}

#[test]
fn xform_unit_vectors() {
    // Easy to visualize: 120 deg about X-axis.
    // Transform the i, j, & k unit vectors.
    let mut q = Quaternion::from((
        &Vector3::new(1.0, 0.0, 0.0),
        (120.0 as float!()).to_radians(),
    ));
    let mut i_t = q.xform(&Vector3::new(1.0, 0.0, 0.0));
    let mut j_t = q.xform(&Vector3::new(0.0, 1.0, 0.0));
    let mut k_t = q.xform(&Vector3::new(0.0, 0.0, 1.0));
    //
    assert!(i_t.is_equal_approx(&Vector3::new(1.0, 0.0, 0.0)));
    assert!(j_t.is_equal_approx(&Vector3::new(0.0, -0.5, 0.866025)));
    assert!(k_t.is_equal_approx(&Vector3::new(0.0, -0.866025, -0.5)));
    assert_approx_eq!(i_t.length_squared(), 1.0);
    assert_approx_eq!(j_t.length_squared(), 1.0);
    assert_approx_eq!(k_t.length_squared(), 1.0);

    // Easy to visualize: 30 deg about Y-axis.
    q = Quaternion::from((
        &Vector3::new(0.0, 1.0, 0.0),
        (30.0 as float!()).to_radians(),
    ));
    i_t = q.xform(&Vector3::new(1.0, 0.0, 0.0));
    j_t = q.xform(&Vector3::new(0.0, 1.0, 0.0));
    k_t = q.xform(&Vector3::new(0.0, 0.0, 1.0));
    //
    assert!(i_t.is_equal_approx(&Vector3::new(0.866025, 0.0, -0.5)));
    assert!(j_t.is_equal_approx(&Vector3::new(0.0, 1.0, 0.0)));
    assert!(k_t.is_equal_approx(&Vector3::new(0.5, 0.0, 0.866025)));
    assert_approx_eq!(i_t.length_squared(), 1.0);
    assert_approx_eq!(j_t.length_squared(), 1.0);
    assert_approx_eq!(k_t.length_squared(), 1.0);

    // Easy to visualize: 60 deg about Z-axis.
    q = Quaternion::from((
        &Vector3::new(0.0, 0.0, 1.0),
        (60.0 as float!()).to_radians(),
    ));
    i_t = q.xform(&Vector3::new(1.0, 0.0, 0.0));
    j_t = q.xform(&Vector3::new(0.0, 1.0, 0.0));
    k_t = q.xform(&Vector3::new(0.0, 0.0, 1.0));
    //
    assert!(i_t.is_equal_approx(&Vector3::new(0.5, 0.866025, 0.0)));
    assert!(j_t.is_equal_approx(&Vector3::new(-0.866025, 0.5, 0.0)));
    assert!(k_t.is_equal_approx(&Vector3::new(0.0, 0.0, 1.0)));
    assert_approx_eq!(i_t.length_squared(), 1.0);
    assert_approx_eq!(j_t.length_squared(), 1.0);
    assert_approx_eq!(k_t.length_squared(), 1.0);
}

#[test]
fn xform_vector() {
    // Arbitrary quaternion rotates an arbitrary vector.
    let euler_yzx = Vector3::new(
        (31.41 as float!()).to_radians(),
        (-49.16 as float!()).to_radians(),
        (12.34 as float!()).to_radians(),
    );
    let basis_axes = Basis::from_euler(&euler_yzx, None);
    let q = Quaternion::from(&basis_axes);

    let v_arb = Vector3::new(3.0, 4.0, 5.0);
    let v_rot = q.xform(&v_arb);
    let v_compare = basis_axes.xform(&v_arb);

    assert_approx_eq!(v_rot.length_squared(), v_arb.length_squared());
    assert!(v_rot.is_equal_approx(&v_compare));
}

#[test]
fn finite_number_checks() {
    let x = <float!()>::NAN;

    assert!(
        !Quaternion::new(x, 1.0, 2.0, 3.0).is_finite(),
        "with one component infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, x, 2.0, 3.0).is_finite(),
        "with one component infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, 1.0, x, 3.0).is_finite(),
        "with one component infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, 1.0, 2.0, x).is_finite(),
        "with one component infinite should not be finite."
    );

    assert!(
        !Quaternion::new(x, x, 2.0, 3.0).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(x, 1.0, x, 3.0).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(x, 1.0, 2.0, x).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, x, x, 3.0).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, x, 2.0, x).is_finite(),
        "with two components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(0.0, 1.0, x, x).is_finite(),
        "with two components infinite should not be finite."
    );

    assert!(
        !Quaternion::new(0.0, x, x, x).is_finite(),
        "with three components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(x, 1.0, x, x).is_finite(),
        "with three components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(x, x, 2.0, x).is_finite(),
        "with three components infinite should not be finite."
    );
    assert!(
        !Quaternion::new(x, x, x, 3.0).is_finite(),
        "with three components infinite should not be finite."
    );

    assert!(
        !Quaternion::new(x, x, x, x).is_finite(),
        "with four components infinite should not be finite."
    );
    assert!(
        Quaternion::new(0.0, 1.0, 2.0, 3.0).is_finite(),
        "with all components finite should be finite"
    );
}
