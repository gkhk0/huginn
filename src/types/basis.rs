use crate::types::vectors::Vector3;
use crate::types::EulerOrder;
use crate::types::Quaternion;
use crate::utils::{
    float, float_consts, int, is_equal_approx, is_equal_approx_with_tolerance, is_zero_approx,
    FloatExt, CMP_EPSILON, UNIT_EPSILON,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::mem::swap;

/// A 3×3 matrix for representing 3D rotation and scale.
///
/// The **Basis** type is a [3×3 matrix](https://en.wikipedia.org/wiki/Matrix_(mathematics)) used to represent 3D rotation, scale, and shear. It is frequently used within a [`Transform3D`].
///
/// A **Basis** is composed by 3 axis vectors, each representing a column of the matrix: `x`, `y`, and `z`. The length of each axis ([`Vector3::length`]) influences the basis's scale, while the direction of all axes influence the rotation. Usually, these axes are perpendicular to one another. However, when you rotate any axis individually, the basis becomes sheared. Applying a sheared basis to a 3D model will make the model appear distorted.
///
/// A **Basis** is **orthogonal** if its axes are perpendicular to each other. A basis is **normalized** if the length of every axis is `1`. A basis is **uniform** if all axes share the same length (see [`Basis::get_scale`]). A basis is **orthonormal** if it is both orthogonal and normalized, which allows it to only represent rotations. A basis is **conformal** if it is both orthogonal and uniform, which ensures it is not distorted.
///
/// **Note:** Grimm uses a [right-handed coordinate system](https://en.wikipedia.org/wiki/Right-hand_rule), which is a common standard. For directions, the convention for built-in types like `Camera3D` is for -Z to point forward (+X is right, +Y is up, and +Z is back). Other objects may use different direction conventions.
///
/// **Note:** The basis matrices are exposed as [column-major](https://www.mindcontrol.org/~hplus/graphics/matrix-layout.html) order, which is the same as OpenGL. However, they are stored internally in row-major order, which is the same as DirectX.
#[derive(Copy, Clone, Debug)]
pub struct Basis {
    /// The row `0` of the matrix.
    ///
    /// On the identity basis, this vector points right ([`Vector3::RIGHT`]).
    pub x: Vector3,
    /// The row `1` of the matrix.
    ///
    /// On the identity basis, this vector points up ([`Vector3::UP`]).
    pub y: Vector3,
    /// The row `2` of the matrix.
    ///
    /// On the identity basis, this vector points back ([`Vector3::BACK`]).
    pub z: Vector3,
}

impl Basis {
    pub fn scaled_local(&self, scale: &Vector3) -> Basis {
        self * Basis::from_scale(scale)
    }
}

impl Basis {
    pub fn set_quaternion_scale(&mut self, quaternion: &Quaternion, scale: &Vector3) {
        self.set_diagonal(scale);
        self.rotate(quaternion);
    }
    fn rotate(&mut self, quaternion: &Quaternion) {
        *self *= Basis::from(quaternion);
    }

    fn set_diagonal(&mut self, diag: &Vector3) {
        self.x.x = diag.x;
        self.x.y = 0.0;
        self.x.z = 0.0;

        self.y.x = 0.0;
        self.y.y = diag.y;
        self.y.z = 0.0;

        self.z.x = 0.0;
        self.z.y = 0.0;
        self.z.z = diag.z;
    }
}

impl Basis {
    pub fn get_axis_angle(&self, axis: &mut Vector3, angle: &mut float!()) {
        // https://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToAngle/index.htm
        if is_zero_approx(self.x.y - self.y.x)
            && is_zero_approx(self.x.z - self.z.x)
            && is_zero_approx(self.y.z - self.z.y)
        {
            // Singularity found.
            // First check for identity matrix which must have +1 for all terms in leading diagonal and zero in other terms.
            if self.is_diagonal()
                && (self.x.x + self.y.y + self.z.z - 3.0).abs() < 3.0 * CMP_EPSILON
            {
                // This singularity is identity matrix so angle = 0.
                axis.x = 0.0;
                axis.y = 1.0;
                axis.z = 0.0;
                *angle = 0.0;
                return;
            }
            // Otherwise this singularity is angle = 180.
            let xx = (self.x.x + 1.0) / 2.0;
            let yy = (self.y.y + 1.0) / 2.0;
            let zz = (self.z.z + 1.0) / 2.0;
            let xy = (self.x.y + self.y.x) / 4.0;
            let xz = (self.x.z + self.z.x) / 4.0;
            let yz = (self.y.z + self.z.y) / 4.0;

            (axis.x, axis.y, axis.z) = if (xx > yy) && (xx > zz) {
                // self.x.x is the largest diagonal term.
                if xx < CMP_EPSILON {
                    (
                        0.0,
                        float_consts::FRAC_1_SQRT_2,
                        float_consts::FRAC_1_SQRT_2,
                    )
                } else {
                    let x = xx.sqrt();
                    (x, xy / x, xz / x)
                }
            } else if yy > zz {
                // self.y.y is the largest diagonal term.
                if yy < CMP_EPSILON {
                    (
                        float_consts::FRAC_1_SQRT_2,
                        0.0,
                        float_consts::FRAC_1_SQRT_2,
                    )
                } else {
                    let y = yy.sqrt();
                    (xy / y, y, yz / y)
                }
            } else {
                // self.z.z is the largest diagonal term so base result on this.
                if zz < CMP_EPSILON {
                    (
                        float_consts::FRAC_1_SQRT_2,
                        float_consts::FRAC_1_SQRT_2,
                        0.0,
                    )
                } else {
                    let z = zz.sqrt();
                    (xz / z, yz / z, z)
                }
            };
            *angle = float_consts::PI;
            return;
        }
        // As we have reached here there are no singularities so we can handle normally.
        let mut s = ((self.z.y - self.y.z) * (self.z.y - self.y.z)
            + (self.x.z - self.z.x) * (self.x.z - self.z.x)
            + (self.y.x - self.x.y) * (self.y.x - self.x.y))
            .sqrt(); // Used to normalize.

        if s.abs() < CMP_EPSILON {
            // Prevent divide by zero, should not happen if matrix is orthogonal and should be caught by singularity test above.
            s = 1.0;
        }

        axis.x = (self.z.y - self.y.z) / s;
        axis.y = (self.x.z - self.z.x) / s;
        axis.z = (self.y.x - self.x.y) / s;

        // safe_acos does clamping.
        *angle = ((self.x.x + self.y.y + self.z.z - 1.0) / 2.0).safe_acos();
        if angle.is_nan() {
            println!(
                "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{}!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
                angle
            );
        }
    }

    fn is_diagonal(&self) -> bool {
        is_zero_approx(self.x.y)
            && is_zero_approx(self.x.z)
            && is_zero_approx(self.y.x)
            && is_zero_approx(self.y.z)
            && is_zero_approx(self.z.x)
            && is_zero_approx(self.z.y)
    }
}

impl Basis {
    pub fn is_orthogonal(&self) -> bool {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        is_zero_approx(x.dot(&y)) && is_zero_approx(x.dot(&z)) && is_zero_approx(y.dot(&z))
    }
}

impl Basis {
    pub fn is_orthonormal(&self) -> bool {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        is_equal_approx(x.length_squared(), 1.0)
            && is_equal_approx(y.length_squared(), 1.0)
            && is_equal_approx(z.length_squared(), 1.0)
            && is_zero_approx(x.dot(&y))
            && is_zero_approx(x.dot(&z))
            && is_zero_approx(y.dot(&z))
    }
}

impl Basis {
    pub fn is_rotation(&self) -> bool {
        self.is_conformal() && is_equal_approx_with_tolerance(self.determinant(), 1.0, UNIT_EPSILON)
    }
}

impl Default for Basis {
    fn default() -> Self {
        Basis::IDENTITY
    }
}

impl Basis {
    /// The basis's X axis, and the column `0` of the matrix.
    ///
    /// On the identity basis, this vector points right ([`Vector3::RIGHT`]).
    pub const fn x(self) -> Vector3 {
        self.get_column(0)
    }
    pub fn set_x(&mut self, vector: &Vector3) {
        self.set_column(0, vector);
    }

    /// The basis's Y axis, and the column `1` of the matrix.
    ///
    /// On the identity basis, this vector points right ([`Vector3::RIGHT`]).
    pub const fn y(self) -> Vector3 {
        self.get_column(1)
    }
    pub fn set_y(&mut self, vector: &Vector3) {
        self.set_column(1, vector);
    }

    /// The basis's Z axis, and the column `2` of the matrix.
    ///
    /// On the identity basis, this vector points right ([`Vector3::RIGHT`]).
    pub const fn z(self) -> Vector3 {
        self.get_column(2)
    }
    pub fn set_z(&mut self, vector: &Vector3) {
        self.set_column(2, vector);
    }

    /// The identity basis. This is a basis with no rotation, no shear, and its scale being `1`. This means that:
    ///
    /// - The `x` points right ([`Vector3::RIGHT`]).
    /// - The `y` points up ([`Vector3::UP`]).
    /// - The `z` points back ([`Vector3.BACK`]).
    ///
    /// This is identical to creating Basis without any parameters. This constant can be used to make your code clearer.
    pub const IDENTITY: Basis = Basis::new(Vector3::RIGHT, Vector3::UP, Vector3::BACK);

    /// When any basis is multiplied by `FLIP_X`, it negates all components of the `x` axis (the X column).
    ///
    /// When `FLIP_X` is multiplied by any basis, it negates the [`Vector3::x`] component of all axes (the X row).
    pub const FLIP_X: Basis = Basis::new(Vector3::LEFT, Vector3::UP, Vector3::BACK);

    /// When any basis is multiplied by `FLIP_Y`, it negates all components of the `y` axis (the Y column).
    ///
    /// When `FLIP_Y` is multiplied by any basis, it negates the [`Vector3::y`] component of all axes (the Y row).
    pub const FLIP_Y: Basis = Basis::new(Vector3::RIGHT, Vector3::DOWN, Vector3::BACK);

    /// When any basis is multiplied by `FLIP_Z`, it negates all components of the `z` axis (the Z column).
    ///
    /// When `FLIP_Z` is multiplied by any basis, it negates the [`Vector3::z`] component of all axes (the Z row).
    pub const FLIP_Z: Basis = Basis::new(Vector3::RIGHT, Vector3::UP, Vector3::FORWARD);

    pub const fn new(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Basis::new_from_floats(x.x, y.x, z.x, x.y, y.y, z.y, x.z, y.z, z.z)
    }
    
    pub const fn new_rows(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self {x,y,z}
    }

    pub const fn new_from_floats(
        xx: float!(),
        xy: float!(),
        xz: float!(),
        yx: float!(),
        yy: float!(),
        yz: float!(),
        zx: float!(),
        zy: float!(),
        zz: float!(),
    ) -> Self {
        Basis {
            x: Vector3::new(xx, xy, xz),
            y: Vector3::new(yx, yy, yz),
            z: Vector3::new(zx, zy, zz),
        }
    }

    /// Returns the [determinant](https://en.wikipedia.org/wiki/Determinant) of this basis's matrix. For advanced math, this number can be used to determine a few attributes:
    ///
    /// -    If the determinant is exactly `0`, the basis is not invertible (see [`Basis::inverse`]).
    ///
    /// -    If the determinant is a negative number, the basis represents a negative scale.
    ///
    /// **Note:** If the basis's scale is the same for every axis, its determinant is always that scale by the power of 2.
    pub fn determinant(&self) -> float!() {
        self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.y.x * (self.x.y * self.z.z - self.z.y * self.x.z)
            + self.z.x * (self.x.y * self.y.z - self.y.y * self.x.z)
    }

    fn set_euler(&mut self, euler: &Vector3, order: EulerOrder) {
        let mut c = euler.x.cos();
        let mut s = euler.x.sin();
        let x_mat = Self::new_from_floats(1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c);

        c = euler.y.cos();
        s = euler.y.sin();
        let y_mat = Self::new_from_floats(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c);

        c = euler.z.cos();
        s = euler.z.sin();
        let z_mat = Self::new_from_floats(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0);

        let basis = match order {
            EulerOrder::XYZ => x_mat * (y_mat * z_mat),
            EulerOrder::XZY => x_mat * z_mat * y_mat,
            EulerOrder::YXZ => y_mat * x_mat * z_mat,
            EulerOrder::YZX => y_mat * z_mat * x_mat,
            EulerOrder::ZXY => z_mat * x_mat * y_mat,
            EulerOrder::ZYX => z_mat * y_mat * x_mat,
        };
        self.x = basis.x;
        self.y = basis.y;
        self.z = basis.z;
    }

    /// Constructs a new **Basis** that only represents rotation from the given [`Vector3`] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians.
    ///
    /// -    The [`Vector3::x`] should contain the angle around the `x` axis (pitch).
    /// -    The [`Vector3::y`] should contain the angle around the `y` axis (yaw).
    /// -    The [`Vector3::z`] should contain the angle around the `z` axis (roll).
    ///
    /// The order of each consecutive rotation can be changed with `order` (see [`EulerOrder`]). By default, the YXZ convention is used ([`EulerOrder::YXZ`]): the basis rotates first around the Y axis (yaw), then X (pitch), and lastly Z (roll). When using the opposite method [`Basis::get_euler`], this order is reversed.
    pub fn from_euler(euler: &Vector3, order: Option<EulerOrder>) -> Self {
        let order = if let Some(order) = order {
            order
        } else {
            EulerOrder::YXZ
        };
        let mut b = Basis::default();
        b.set_euler(euler, order);
        b
    }

    /// Constructs a new Basis that only represents scale, with no rotation or shear, from the given `scale` vector.
    ///
    /// **Note:** In linear algebra, the matrix of this basis is also known as a [diagonal matrix](https://en.wikipedia.org/wiki/Diagonal_matrix).
    pub const fn from_scale(scale: &Vector3) -> Self {
        Self::new_from_floats(scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, scale.z)
    }

    /// Returns this basis's rotation as a [`Vector3`] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians.
    ///
    /// -    The [`Vector3::x`] contains the angle around the `x` axis (pitch);
    /// -    The [`Vector3::y`] contains the angle around the `y` axis (yaw);
    /// -    The [`Vector3::z`] contains the angle around the `z` axis (roll).
    ///
    /// The order of each consecutive rotation can be changed with `order` (see [`EulerOrder`]). By default, the YXZ convention is used ([`EulerOrder::YXZ`]): Z (roll) is calculated first, then X (pitch), and lastly Y (yaw). When using the opposite method [`Basis::from_euler`], this order is reversed.
    ///
    /// **Note:** Euler angles are much more intuitive but are not suitable for 3D math. Because of this, consider using the [`Basis::get_rotation_quaternion`] method instead, which returns a [`Quaternion`].
    pub fn get_euler(&self, order: Option<EulerOrder>) -> Vector3 {
        let order = if let Some(order) = order {
            order
        } else {
            EulerOrder::YXZ
        };

        match order {
            EulerOrder::XYZ => {
                // Euler angles in XYZ convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =    cy*cz           -cy*sz          sy
                //          cz*sx*sy+cx*sz  cx*cz-sx*sy*sz  -cy*sx
                // 			-cx*cz*sy+sx*sz cz*sx+cx*sy*sz  cx*cy
                let sy = self.x.z;
                if sy < (1.0 - CMP_EPSILON) {
                    if sy > -(1.0 - CMP_EPSILON) {
                        // is this a pure Y rotation?
                        if self.y.x == 0.0
                            && self.x.y == 0.0
                            && self.y.z == 0.0
                            && self.z.y == 0.0
                            && self.y.y == 1.0
                        {
                            // return the simplest form (human friendlier in editor and scripts)
                            Vector3::new(0.0, self.x.z.atan2(self.x.x), 0.0)
                        } else {
                            Vector3::new(
                                (-self.y.z).atan2(self.z.z),
                                sy.asin(),
                                (-self.x.y).atan2(self.x.x),
                            )
                        }
                    } else {
                        Vector3::new(self.z.y.atan2(self.y.y), -float_consts::PI / 2.0, 0.0)
                    }
                } else {
                    Vector3::new(self.z.y.atan2(self.y.y), float_consts::PI / 2.0, 0.0)
                }
            }
            EulerOrder::XZY => {
                // Euler angles in XZY convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =  cz*cy             -sz             cz*sy
                //        sx*sy+cx*cy*sz    cx*cz           cx*sz*sy-cy*sx
                //        cy*sx*sz          cz*sx           cx*cy+sx*sz*sy

                let sz = self.x.y;
                if sz < (1.0 - CMP_EPSILON) {
                    if sz > -(1.0 - CMP_EPSILON) {
                        Vector3::new(
                            self.z.y.atan2(self.y.y),
                            self.x.z.atan2(self.x.x),
                            (-sz).asin(),
                        )
                    } else {
                        // It's -1
                        Vector3::new(-self.y.z.atan2(self.z.z), 0.0, float_consts::PI / 2.0)
                    }
                } else {
                    // It's 1
                    Vector3::new(-self.y.z.atan2(self.z.z), 0.0, -float_consts::PI / 2.0)
                }
            }
            EulerOrder::YXZ => {
                // Euler angles in YXZ convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =  cy*cz+sy*sx*sz    cz*sy*sx-cy*sz        cx*sy
                //        cx*sz             cx*cz                 -sx
                //        cy*sx*sz-cz*sy    cy*cz*sx+sy*sz        cy

                let m12 = self.y.z;

                if m12 < (1.0 - CMP_EPSILON) {
                    if m12 > -(1.0 - CMP_EPSILON) {
                        // is it a pure X rotation?
                        if self.y.x == 0.0
                            && self.x.y == 0.0
                            && self.x.z == 0.0
                            && self.z.x == 0.0
                            && self.x.x == 1.0
                        {
                            // return the simplest form (human friendlier in editor and scripts)
                            Vector3::new((-m12).atan2(self.y.y), 0.0, 0.0)
                        } else {
                            Vector3::new(
                                (-m12).asin(),
                                self.x.z.atan2(self.z.z),
                                self.y.x.atan2(self.y.y),
                            )
                        }
                    } else {
                        // It's -1
                        Vector3::new(float_consts::PI / 2.0, self.x.y.atan2(self.x.x), 0.0)
                    }
                } else {
                    // It's 1
                    Vector3::new(-float_consts::PI / 2.0, -self.x.y.atan2(self.x.x), 0.0)
                }
            }
            EulerOrder::YZX => {
                // Euler angles in YZX convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =  cy*cz             sy*sx-cy*cx*sz     cx*sy+cy*sz*sx
                //        sz                cz*cx              -cz*sx
                //        -cz*sy            cy*sx+cx*sy*sz     cy*cx-sy*sz*sx

                let sz = self.y.x;
                if sz < (1.0 - CMP_EPSILON) {
                    if sz > -(1.0 - CMP_EPSILON) {
                        Vector3::new(
                            (-self.y.z).atan2(self.y.y),
                            (-self.z.x).atan2(self.x.x),
                            sz.asin(),
                        )
                    } else {
                        // It's -1
                        Vector3::new(self.z.y.atan2(self.z.z), 0.0, -float_consts::PI / 2.0)
                    }
                } else {
                    // It's 1
                    Vector3::new(self.z.y.atan2(self.z.z), 0.0, float_consts::PI / 2.0)
                }
            }
            EulerOrder::ZXY => {
                // Euler angles in ZXY convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =  cz*cy-sz*sx*sy    -cx*sz                cz*sy+cy*sz*sx
                //        cy*sz+cz*sx*sy    cz*cx                 sz*sy-cz*cy*sx
                //        -cx*sy            sx                    cx*cy

                let sx = self.z.y;
                if sx < (1.0 - CMP_EPSILON) {
                    if sx > -(1.0 - CMP_EPSILON) {
                        Vector3::new(
                            sx.asin(),
                            (-self.z.x).atan2(self.z.z),
                            (-self.x.y).atan2(self.y.y),
                        )
                    } else {
                        // It's -1
                        Vector3::new(-float_consts::PI / 2.0, self.x.z.atan2(self.x.x), 0.0)
                    }
                } else {
                    // It's 1
                    Vector3::new(float_consts::PI / 2.0, self.x.z.atan2(self.x.x), 0.0)
                }
            }
            EulerOrder::ZYX => {
                // Euler angles in ZYX convention.
                // See https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
                //
                // rot =  cz*cy             cz*sy*sx-cx*sz        sz*sx+cz*cx*cy
                //        cy*sz             cz*cx+sz*sy*sx        cx*sz*sy-cz*sx
                //        -sy               cy*sx                 cy*cx

                let sy = self.z.x;
                if sy < (1.0 - CMP_EPSILON) {
                    if sy > -(1.0 - CMP_EPSILON) {
                        Vector3::new(
                            self.z.y.atan2(self.z.z),
                            (-sy).asin(),
                            self.y.x.atan2(self.x.x),
                        )
                    } else {
                        // It's -1
                        Vector3::new(0.0, float_consts::PI / 2.0, -self.x.y.atan2(self.y.y))
                    }
                } else {
                    // It's 1
                    Vector3::new(0.0, -float_consts::PI / 2.0, -self.x.y.atan2(self.y.y))
                }
            }
        }
    }

    fn get_quaternion(&self) -> Quaternion {
        /* Allow getting a quaternion from an un normalized transform */
        let m = *self;
        let trace = m.x.x + m.y.y + m.z.z;
        let mut temp = [0.0; 4];

        if trace > 0.0 {
            let mut s = (trace + 1.0).sqrt();
            temp[3] = s * 0.5;
            s = 0.5 / s;

            temp[0] = (m.z.y - m.y.z) * s;
            temp[1] = (m.x.z - m.z.x) * s;
            temp[2] = (m.y.x - m.x.y) * s;
        } else {
            let i = if m.x.x < m.y.y {
                if m.y.y < m.z.z {
                    2
                } else {
                    1
                }
            } else if m.x.x < m.z.z {
                2
            } else {
                0
            };
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;

            let mut s =
                (m.get_row(i).get(i) - m.get_row(j).get(j) - m.get_row(k).get(k) + 1.0).sqrt();
            temp[i] = s * 0.5;
            s = 0.5 / s;

            temp[3] = (m.get_row(k).get(j) - m.get_row(j).get(k)) * s;
            temp[j] = (m.get_row(j).get(i) + m.get_row(i).get(j)) * s;
            temp[k] = (m.get_row(k).get(i) + m.get_row(i).get(k)) * s;
        }

        Quaternion::new(temp[0], temp[1], temp[2], temp[3])
    }

    /// Returns this basis's rotation as a [`Quaternion`].
    ///
    /// **Note:** Quaternions are much more suitable for 3D math but are less intuitive. For user interfaces, consider using the [`Basis::get_euler`] method, which returns Euler angles.
    pub fn get_rotation_quaternion(&self) -> Quaternion {
        // Assumes that the matrix can be decomposed into a proper rotation and scaling matrix as M = R.S,
        // and returns the Euler angles corresponding to the rotation part, complementing get_scale().
        // See the comment in get_scale() for further information.
        let mut m = self.orthonormalized();
        let det = m.determinant();
        if det < 0.0 {
            // Ensure that the determinant is 1, such that result is a proper rotation matrix which can be represented by Euler angles.
            m.scale(&Vector3::new(-1.0, -1.0, -1.0));
        }
        m.get_quaternion()
    }

    fn get_scale_abs(&self) -> Vector3 {
        Vector3::new(
            self.get_column(0).length(),
            self.get_column(1).length(),
            self.get_column(2).length(),
        )
    }

    /// Returns the length of each axis of this basis, as a Vector3. If the basis is not sheared, this is the scaling factor. It is not affected by rotation.
    ///
    /// **Note:** If the value returned by [`Basis::determinant`] is negative, the scale is also negative.
    pub fn get_scale(&self) -> Vector3 {
        let det_sign = self.determinant().sign();
        det_sign * self.get_scale_abs()
    }

    fn set(
        &mut self,
        xx: float!(),
        xy: float!(),
        xz: float!(),
        yx: float!(),
        yy: float!(),
        yz: float!(),
        zx: float!(),
        zy: float!(),
        zz: float!(),
    ) {
        self.x.x = xx;
        self.x.y = xy;
        self.x.z = xz;
        self.y.x = yx;
        self.y.y = yy;
        self.y.z = yz;
        self.z.x = zx;
        self.z.y = zy;
        self.z.z = zz;
    }

    pub(crate) fn invert(&mut self) {
        macro_rules! cofac {
            ($row1:ident, $col1:ident, $row2:ident, $col2:ident) => {
                self.$row1.$col1 * self.$row2.$col2 - self.$row1.$col2 * self.$row2.$col1
            };
        }
        let co = [cofac!(y, y, z, z), cofac!(y, z, z, x), cofac!(y, x, z, y)];
        let det = self.x.x * co[0] + self.x.y * co[1] + self.x.z * co[2];

        let s = 1.0 / det;

        self.set(
            co[0] * s,
            cofac!(x, z, z, y) * s,
            cofac!(x, y, y, z) * s,
            co[1] * s,
            cofac!(x, x, z, z) * s,
            cofac!(x, z, y, x) * s,
            co[2] * s,
            cofac!(x, y, z, x) * s,
            cofac!(x, x, y, y) * s,
        );
    }

    /// Returns the [inverse of this basis's matrix](https://en.wikipedia.org/wiki/Invertible_matrix).
    pub fn inverse(&self) -> Self {
        let mut inv = *self;
        inv.invert();
        inv
    }

    /// Returns `true` if this basis is conformal. A conformal basis is both *orthogonal* (the axes are perpendicular to each other) and *uniform* (the axes share the same length). This method can be especially useful during physics calculations.
    pub fn is_conformal(&self) -> bool {
        let x = self.get_column(0);
        let y = self.get_column(1);
        let z = self.get_column(2);
        let x_len_sq = x.length_squared();
        is_equal_approx(x_len_sq, y.length_squared())
            && is_equal_approx(x_len_sq, z.length_squared())
            && is_zero_approx(x.dot(&y))
            && is_zero_approx(x.dot(&z))
            && is_zero_approx(y.dot(&z))
    }

    /// Returns `true` if this basis and `b` are approximately equal, by calling [`Vector3::is_equal_approx`] on all vector components.
    pub fn is_equal_approx(&self, b: &Basis) -> bool {
        self.x.is_equal_approx(&b.x) && self.y.is_equal_approx(&b.y) && self.z.is_equal_approx(&b.z)
    }

    /// Returns `true` if this basis is finite, by calling [`Vector3::is_finite`] on all vector components.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Creates a new **Basis** with a rotation such that the forward axis (-Z) points towards the `target` position.
    ///
    /// By default, the -Z axis (camera forward) is treated as forward (implies +X is right). If `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position.
    ///
    /// The up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The returned basis is orthonormalized (see [`Basis::orthonormalized`]). The `target` and `up` vectors cannot be [`Vector3::ZERO`], and cannot be parallel to each other.
    pub fn looking_at(target: &Vector3, up: Option<&Vector3>, use_model_front: bool) -> Self {
        let up = if let Some(up) = up { up } else { &Vector3::UP };

        let mut v_z = target.normalized();
        if !use_model_front {
            v_z = -v_z;
        }
        let v_x = up.cross(&v_z).normalized();
        let v_y = v_z.cross(&v_x);

        let mut b = Basis::default();
        b.set_columns(&v_x, &v_y, &v_z);
        b
    }

    pub(crate) fn orthonormalize(&mut self) {
        let mut x = self.get_column(0);
        let mut y = self.get_column(1);
        let mut z = self.get_column(2);

        x = x.normalized();
        y = y - x * (x.dot(&y));
        y = y.normalized();
        z = z - x * (x.dot(&z)) - y * (y.dot(&z));
        z = z.normalized();

        self.set_columns(&x, &y, &z);
    }

    /// Returns the orthonormalized version of this basis. An orthonormal basis is both *orthogonal* (the axes are perpendicular to each other) and *normalized* (the axes have a length of `1`), which also means it can only represent rotation.
    ///
    /// It is often useful to call this method to avoid rounding errors on a rotating basis:
    ///
    /// ```
    /// # use huginn::utils::float_consts::PI;
    /// # use huginn::types::Basis;
    /// # use huginn::types::vectors::Vector3;
    /// # let mut basis = Basis::default();
    /// basis = basis.rotated(&Vector3::UP, PI);
    /// basis = basis.rotated(&Vector3::RIGHT, PI);
    ///
    /// basis = basis.orthonormalized();
    /// ```
    pub fn orthonormalized(&self) -> Self {
        let mut c = *self;
        c.orthonormalize();
        c
    }

    /// Returns this basis rotated around the given `axis` by `angle` (in radians). The `axis` must be a normalized vector (see [`Vector3::normalized`]).
    ///
    /// Positive values rotate this basis clockwise around the axis, while negative values rotate it counterclockwise.
    /// ```
    /// # use huginn::utils::float_consts::TAU;
    /// # use huginn::types::Basis;
    /// # use huginn::types::vectors::Vector3;
    /// let mut basis = Basis::IDENTITY;
    /// let angle = TAU / 2.0;
    ///
    /// basis = basis.rotated(&Vector3::UP, angle); // Rotate around the up axis (yaw).
    /// basis = basis.rotated(&Vector3::RIGHT, angle); // Rotate around the right axis (pitch).
    /// basis = basis.rotated(&Vector3::BACK, angle); // Rotate around the back axis (roll).
    /// ```
    pub fn rotated(&self, axis: &Vector3, angle: float!()) -> Basis {
        Basis::from((axis, angle)) * self
    }

    fn scale(&mut self, scale: &Vector3) {
        self.x *= scale.x;
        self.y *= scale.y;
        self.z *= scale.z;
    }

    /// Returns this basis with each axis's components scaled by the given `scale`'s components.
    ///
    /// The basis matrix's rows are multiplied by `scale`'s components. This operation is a global scale (relative to the parent).
    /// ```
    /// # use huginn::types::Basis;
    /// # use huginn::types::vectors::Vector3;
    /// let mut basis = Basis::new(
    ///     Vector3::new(1.0, 1.0, 1.0),
    ///     Vector3::new(2.0, 2.0, 2.0),
    ///     Vector3::new(3.0, 3.0, 3.0),
    /// );
    /// basis = basis.scaled(&Vector3::new(0.0, 2.0, -2.0));
    ///
    /// println!("{:#?}", basis.x()); // Prints (0.0, 2.0, -2.0).
    /// println!("{:#?}", basis.y()); // Prints (0.0, 4.0, -4.0).
    /// println!("{:#?}", basis.z()); // Prints (0.0, 6.0, -6.0).
    /// # assert_eq!(basis.x(), Vector3::new(0.0, 2.0, -2.0));
    /// # assert_eq!(basis.y(), Vector3::new(0.0, 4.0, -4.0));
    /// # assert_eq!(basis.z(), Vector3::new(0.0, 6.0, -6.0))
    /// ```
    pub fn scaled(&self, scale: &Vector3) -> Self {
        let mut m = *self;
        m.scale(scale);
        m
    }

    /// Performs a spherical-linear interpolation with the `to` basis, given a `weight`. Both this basis and `to` should represent a rotation.
    pub fn slerp(&self, to: &Self, weight: float!()) -> Self {
        let from = Quaternion::from(self);
        let to_q = Quaternion::from(to);

        let mut b = Basis::from(&from.slerp(&to_q, weight));
        b.x *= self.x.length().lerp(to.x.length(), weight);
        b.y *= self.y.length().lerp(to.y.length(), weight);
        b.z *= self.z.length().lerp(to.z.length(), weight);

        b
    }

    /// Returns the transposed dot product between `with` and the `x` axis (see [`Basis::transposed`]).
    ///
    /// This is equivalent to `basis.x().dot(vector)`.
    pub fn t_dot_x(&self, with: &Vector3) -> float!() {
        self.x.x * with.x + self.y.x * with.y + self.z.x * with.z
    }

    /// Returns the transposed dot product between `with` and the `y` axis (see [`Basis::transposed`]).
    ///
    /// This is equivalent to `basis.y().dot(vector)`.
    pub fn t_dot_y(&self, with: &Vector3) -> float!() {
        self.x.y * with.x + self.y.y * with.y + self.z.y * with.z
    }

    /// Returns the transposed dot product between `with` and the `z` axis (see [`Basis::transposed`]).
    ///
    /// This is equivalent to `basis.z().dot(vector)`.
    pub fn t_dot_z(&self, with: &Vector3) -> float!() {
        self.x.z * with.x + self.y.z * with.y + self.z.z * with.z
    }

    fn transpose(&mut self) {
        swap(&mut self.x.y, &mut self.y.x);
        swap(&mut self.x.z, &mut self.z.x);
        swap(&mut self.y.z, &mut self.z.y);
    }

    /// Returns the transposed version of this basis. This turns the basis matrix's columns into rows, and its rows into columns.
    /// ```
    /// # use huginn::types::Basis;
    /// # use huginn::types::vectors::Vector3;
    /// let mut basis = Basis::new(
    ///     Vector3::new(1.0, 2.0, 3.0),
    ///     Vector3::new(4.0, 5.0, 6.0),
    ///     Vector3::new(7.0, 8.0, 9.0),
    /// );
    /// basis = basis.transposed();
    ///
    /// println!("{:?}", basis.x()); // Prints (1.0, 4.0, 7.0).
    /// println!("{:?}", basis.y()); // Prints (2.0, 5.0, 8.0).
    /// println!("{:?}", basis.z()); // Prints (3.0, 6.0, 9.0).
    /// # assert_eq!(basis.x(), Vector3::new(1.0, 4.0, 7.0));
    /// # assert_eq!(basis.y(), Vector3::new(2.0, 5.0, 8.0));
    /// # assert_eq!(basis.z(), Vector3::new(3.0, 6.0, 9.0));
    /// ```
    pub fn transposed(&self) -> Self {
        let mut tr = *self;
        tr.transpose();
        tr
    }

    pub const fn get_row(&self, index: usize) -> Vector3 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid index"),
        }
    }

    pub fn set_row(&mut self, index: usize, row: &Vector3) {
        match index {
            0 => self.x = *row,
            1 => self.y = *row,
            2 => self.z = *row,
            _ => panic!("Invalid index"),
        }
    }

    pub const fn get_column(&self, index: usize) -> Vector3 {
        match index {
            0 => Vector3::new(self.x.x, self.y.x, self.z.x),
            1 => Vector3::new(self.x.y, self.y.y, self.z.y),
            2 => Vector3::new(self.x.z, self.y.z, self.z.z),
            _ => panic!("Invalid index"),
        }
    }
    pub fn set_column(&mut self, index: usize, column: &Vector3) {
        match index {
            0 => {
                self.x.x = column.x;
                self.y.x = column.y;
                self.z.x = column.z;
            }
            1 => {
                self.x.y = column.x;
                self.y.y = column.y;
                self.z.y = column.z;
            }
            2 => {
                self.x.z = column.x;
                self.y.z = column.y;
                self.z.z = column.z;
            }
            _ => panic!("Invalid index"),
        }
    }

    pub fn set_columns(&mut self, column_1: &Vector3, column_2: &Vector3, column_3: &Vector3) {
        self.set_column(0, column_1);
        self.set_column(1, column_2);
        self.set_column(2, column_3);
    }

    fn set_axis_angle(&mut self, axis: &Vector3, angle: float!()) {
        let axis_sq = Vector3::new(axis.x * axis.x, axis.y * axis.y, axis.z * axis.z);
        let cosine = angle.cos();
        self.x.x = axis_sq.x + cosine * (1.0 - axis_sq.x);
        self.y.y = axis_sq.y + cosine * (1.0 - axis_sq.y);
        self.z.z = axis_sq.z + cosine * (1.0 - axis_sq.z);

        let sine = angle.sin();
        let t = 1.0 - cosine;

        let mut xyzt = axis.x * axis.y * t;
        let mut zyxs = axis.z * sine;
        self.x.y = xyzt - zyxs;
        self.y.x = xyzt + zyxs;

        xyzt = axis.x * axis.z * t;
        zyxs = axis.y * sine;
        self.x.z = xyzt + zyxs;
        self.z.x = xyzt - zyxs;

        xyzt = axis.y * axis.z * t;
        zyxs = axis.x * sine;
        self.y.z = xyzt - zyxs;
        self.z.y = xyzt + zyxs;
    }

    fn set_quaternion(&mut self, quaternion: Quaternion) {
        let d = quaternion.length_squared();
        let s = 2.0 / d;
        let xs = quaternion.x * s;
        let ys = quaternion.y * s;
        let zs = quaternion.z * s;
        let wx = quaternion.w * xs;
        let wy = quaternion.w * ys;
        let wz = quaternion.w * zs;
        let xx = quaternion.x * xs;
        let xy = quaternion.x * ys;
        let xz = quaternion.x * zs;
        let yy = quaternion.y * ys;
        let yz = quaternion.y * zs;
        let zz = quaternion.z * zs;
        self.set(
            1.0 - (yy + zz),
            xy - wz,
            xz + wy,
            xy + wz,
            1.0 - (xx + zz),
            yz - wx,
            xz - wy,
            yz + wx,
            1.0 - (xx + yy),
        );
    }

    pub fn xform(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(self.x.dot(vector), self.y.dot(vector), self.z.dot(vector))
    }
}

impl PartialEq for Basis {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Basis {}

impl_op_ex!(*|a: &Basis, b: &Basis| -> Basis {
    println!("basis before: {:?}", a);
    let basis = Basis::new_from_floats(
        b.t_dot_x(&a.x), b.t_dot_y(&a.x), b.t_dot_z(&a.x),
        b.t_dot_x(&a.y), b.t_dot_y(&a.y), b.t_dot_z(&a.y),
        b.t_dot_x(&a.z), b.t_dot_y(&a.z), b.t_dot_z(&a.z),
    );
    println!("basis: {:?}", basis);
    basis
});
impl_op_ex!(*= |a: &mut Basis, b: &Basis| {
    println!("basis before: {:?}", a);
    //a.x.x = b.t_dot_x(&a.x); a.x.y = b.t_dot_y(&a.x); a.x.z = b.t_dot_z(&a.x);
    //a.y.x = b.t_dot_x(&a.y); a.y.y = b.t_dot_y(&a.y); a.y.z = b.t_dot_z(&a.y);
    //a.z.x = b.t_dot_x(&a.z); a.z.y = b.t_dot_y(&a.z); a.z.z = b.t_dot_z(&a.z);
    *a = *a * b;
    println!("basis: {:?}", a);
});
//TODO: impl_op_ex_commutative!(*|a: &Basis, b: &Vector3| -> Vector3 { todo!() });
impl_op_ex_commutative!(*|a: &Basis, b: &float!()| -> Basis {
    Basis::new(a.x * b, a.y * b, a.z * b)
});
impl_op_ex_commutative!(*|a: &Basis, b: int!()| -> Basis { a * b as float!() });

impl_op_ex!(/ |a: &Basis, b: &float!()| -> Basis {
    Basis::new(
        a.x/b,
        a.y/b,
        a.z/b,
    )
});
impl_op_ex!(/ |a: &Basis, b: int!()| -> Basis {
    a/b as float!()
});

impl_op_ex!(*= |a: &mut Basis, b: &float!()| {
    a.x = a.x * b;
    a.y = a.y * b;
    a.z = a.z * b;
});
impl_op_ex!(*= |a: &mut Basis, b: int!()| {
    a.x = a.x * b;
    a.y = a.y * b;
    a.z = a.z * b;
});
impl_op_ex!(/= |a: &mut Basis, b: &float!()| {
    a.x = a.x / b;
    a.y = a.y / b;
    a.z = a.z / b;
});
impl_op_ex!(/= |a: &mut Basis, b: int!()| {
    a.x = a.x / b;
    a.y = a.y / b;
    a.z = a.z / b;
});
impl From<(&Vector3, float!())> for Basis {
    fn from(tuple: (&Vector3, float!())) -> Self {
        let mut b = Basis::default();
        b.set_axis_angle(tuple.0, tuple.1);
        b
    }
}

impl From<&Quaternion> for Basis {
    fn from(value: &Quaternion) -> Self {
        let mut b = Basis::default();
        b.set_quaternion(*value);
        b
    }
}

impl From<&Basis> for Quaternion {
    /// Constructs a **Quaternion** from the given rotation Basis.
    ///
    /// This constructor is faster than [`Basis::get_rotation_quaternion`], but the given basis must be *orthonormalized* (see [`Basis::orthonormalized`]). Otherwise, the constructor fails and returns [`Quaternion::IDENTITY`].
    fn from(b: &Basis) -> Self {
        b.get_quaternion()
    }
}

impl From<(&Quaternion, &Vector3)> for Basis {
    fn from(value: (&Quaternion, &Vector3)) -> Self {
        let mut basis = Basis::default();
        basis.set_quaternion_scale(value.0, value.1);
        basis
    }
}
