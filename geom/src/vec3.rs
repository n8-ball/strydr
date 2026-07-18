use std::{fmt::{Formatter, Display}, fmt, ops::{Add, Div, Mul, Neg, Sub}};
use crate::{scalar::Scalar, quat::Quat};

/// A 3d vector represented by scalars, x, y, and z.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: Scalar>{
    pub x: T, 
    pub y: T, 
    pub z: T,
}

const UNIT_LENGTH_EPS: f64 = 1e-6;
const LENGTH_NEAR_ZERO_EPS: f64 = 1e-8;
const ORTHOGONAL_EPS: f64 = 1e-6; 
const ANGLE_TOLERANCE_RADIANS: f64 = 1e-6; 

impl<T: Scalar> Vec3<T> {

    pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO, };
    pub const UNIT_X: Self = Self { x: T::ONE, y: T::ZERO, z: T::ZERO, };
    pub const UNIT_Y: Self = Self { x: T::ZERO, y: T::ONE, z: T::ZERO, };
    pub const UNIT_Z: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ONE, };
    pub const MAX: Self = Self {x: T::MAX, y: T::MAX, z: T::MAX, };
    pub const MIN: Self = Self {x: T::MIN, y: T::MIN, z: T::MIN, };

    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {x, y, z, }
    }

    pub fn vec_add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn vec_sub(&self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    pub fn scalar_mul(&self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    pub fn scalar_div(&self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    pub fn abs(&self) -> Self {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn dot(&self, other: Self) -> T {
        self.x * other.x + 
        self.y * other.y + 
        self.z * other.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length_squared(&self) -> T {
        self.dot(*self)
    }

    pub fn length(self) -> T {
        self.length_squared().sqrt() 
    }

    pub fn normalize(self) -> Self {
        let len = self.length();

        assert!(len > T::from_f64(LENGTH_NEAR_ZERO_EPS),
            "cannot normalize near-zero-length Vec3!");

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn is_unit(self) -> bool {
        let len_sq = self.length_squared();
        (len_sq - T::ONE).abs() < T::from_f64(UNIT_LENGTH_EPS)
    }

    pub fn is_exactly_zero(self) -> bool {
        self.x == T::ZERO && self.y == T::ZERO && self.z == T::ZERO
    }

    pub fn is_near_zero(self) -> bool {
        self.length_squared() < T::from_f64(LENGTH_NEAR_ZERO_EPS * LENGTH_NEAR_ZERO_EPS)
    }

    pub fn angle_to(self, other: Self) -> T{
        // Consider returning a Result instead of panicking.

        // Too small to define an angle.
        assert!(!self.is_near_zero(), 
            "vector 'self' is near-zero, cannot compute angle!"
        );

        assert!(!other.is_near_zero(), 
            "vector 'other' is near-zero, cannot compute angle!"
        );

        let cross_len = self.cross(other).length();
        let dot = self.dot(other);
        cross_len.atan2(dot)
    }

    pub fn is_perpendicular_to(self, other: Self) -> bool {

        // Too small to define an angle.
        if self.is_near_zero() || other.is_near_zero() {
            return false;
        }

        let len = self.length() * other.length();
        let cos = self.dot(other) / len;
        cos.abs() < Scalar::from_f64(ORTHOGONAL_EPS)
    }

    pub fn is_parallel_to(self, other: Self) -> bool {

        // Too small to define an angle.
        if self.is_near_zero() || other.is_near_zero() {
            return false;
        }

        let len_prod = self.length() * other.length();
        let cross_len = self.cross(other).length();

        let sin_angle = cross_len / len_prod;
        let tolerance  = T::from_f64(ANGLE_TOLERANCE_RADIANS).sin();
        return sin_angle <= tolerance
    }

    /// Primarily to be used if a single vector is rotated.
    pub fn rotate_axis_angle(self, axis: Self, angle_radians: T) -> Self {
        let axis_norm = axis.normalize();
        let q = Quat::from_axis_angle(axis_norm, angle_radians);
        self.rotate(q)
    }

    /// Primarily used when multiple Vec3s need to rotated, as a the Quaternion is not built internally.
    pub fn rotate(self, quat: Quat<T>) -> Self {
        assert!(quat.is_unit(), 
        "quaternion must be unit-length!");

        let vector_quat = Quat::new(T::ZERO, self.x, self.y, self.z);
        let applied = quat * vector_quat * quat.conjugate();

        Self {
            x: applied.x,
            y: applied.y,
            z: applied.z,
        }
    }

    pub fn assert_near(self, b: Self, eps: T) {
        assert!((self.x - b.x).abs() < eps,
            "left x: {} != right x: {}", self.x, b.x);
        assert!((self.y - b.y).abs() < eps,
            "left y: {} != right y: {}", self.y, b.y);
        assert!((self.z - b.z).abs() < eps,
            "left z: {} != right z: {}", self.z, b.z);
    }
}

/// -Vec3
impl<T: Scalar> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self)  -> Self::Output {
    Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Vec3 + Vec3
impl<T: Scalar> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self)  -> Self::Output {
    Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Vec3 - Vec3
impl<T: Scalar> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self)  -> Self::Output {
    Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Vec3 * Scalar
impl<T: Scalar> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, s: T)  -> Self::Output {
    Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

/// Scalar * Vec3
#[macro_export]
macro_rules!  impl_scalar_mul_vec3 {
    ($t: ty) => {
        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;

            fn mul(self, v: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self * v.x,
                    y: self * v.y,
                    z: self * v.z,
                }
            }
        } 
    };
}

impl_scalar_mul_vec3!(f32);
impl_scalar_mul_vec3!(f64);

/// Vec3 / Scalar
impl<T: Scalar> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, s: T)  -> Self::Output {
    Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl<T: Scalar> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scalar::TestScalar, scalar_test};

    scalar_test!(test_new, |T| {

        let v = Vec3::<T>::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, T::from_f64(1.0));
        assert_eq!(v.y, T::from_f64(2.0));
        assert_eq!(v.z, T::from_f64(3.0));
    });

    scalar_test!(test_zero, |T| {

        let v = Vec3::<T>::ZERO;

        assert_eq!(v.x, T::ZERO);
        assert_eq!(v.y, T::ZERO);
        assert_eq!(v.z, T::ZERO);
    });

    scalar_test!(test_unit_x, |T| {

        let v = Vec3::<T>::UNIT_X;

        assert_eq!(v.x, T::ONE);
        assert_eq!(v.y, T::ZERO);
        assert_eq!(v.z, T::ZERO);
    });

    scalar_test!(test_unit_y, |T| {

        let v = Vec3::<T>::UNIT_Y;

        assert_eq!(v.x, T::ZERO);
        assert_eq!(v.y, T::ONE);
        assert_eq!(v.z, T::ZERO);
    });

    scalar_test!(test_unit_z, |T| {

        let v = Vec3::<T>::UNIT_Z;

        assert_eq!(v.x, T::ZERO);
        assert_eq!(v.y, T::ZERO);
        assert_eq!(v.z, T::ONE);
    });

    scalar_test!(test_max, |T| {

        let v = Vec3::<T>::MAX;

        assert_eq!(v.x, T::MAX);
        assert_eq!(v.y, T::MAX);
        assert_eq!(v.z, T::MAX);
    });

    scalar_test!(test_min, |T| {

        let v = Vec3::<T>::MIN;

        assert_eq!(v.x, T::MIN);
        assert_eq!(v.y, T::MIN);
        assert_eq!(v.z, T::MIN);
    });

    fn test_vec_add<T: TestScalar>(a: Vec3<T>, b: Vec3<T>) 
    {
        let expected = Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        };

        let sum_fn = a.vec_add(b);
        let sum_op = a + b;

        assert_eq!(sum_fn, expected);
        assert_eq!(sum_op, expected);
        assert_eq!(sum_fn, sum_op);
    }

    scalar_test!(test_vec_add_scalar, |T| {

        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);

        test_vec_add(a, b);
    });

    fn test_vec_sub<T: TestScalar>(lhs: Vec3<T>, rhs: Vec3<T>) 
    {
        let diff_fn = lhs.vec_sub(rhs);
        let diff_op = lhs - rhs;

        let expected = Vec3 {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
        };

        assert_eq!(diff_fn, expected);
        assert_eq!(diff_op, expected);
        assert_eq!(diff_fn, diff_op);
    }

    scalar_test!(test_vec_sub_scalar, |T| {

        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);

        test_vec_sub(a, b);
    });

    fn test_scalar_mul<T>(s: T, v: Vec3<T>)
    where 
        T: TestScalar + Mul<Vec3<T>, Output = Vec3<T>>,
    {
        let prod_fn = v.scalar_mul(s);
        let prod_op_scalar_rhs  = v * s;
        let prod_op_scalar_lhs: Vec3<T>  =  s * v;

        let expected = Vec3 {
            x: s * v.x,
            y: s * v.y,
            z: s * v.z,
        };

        assert_eq!(prod_fn, expected);
        assert_eq!(prod_op_scalar_rhs, expected);
        assert_eq!(prod_op_scalar_lhs, expected);
        assert_eq!(prod_fn, prod_op_scalar_rhs);
        assert_eq!(prod_fn, prod_op_scalar_lhs);
        assert_eq!(prod_op_scalar_rhs, prod_op_scalar_lhs);
    }

    scalar_test!(test_scalar_vec_mul_scalar, |T| {
        let s = 10.0 ;
        let v = Vec3::<T>::new(1.0, 2.0, 3.0);
        test_scalar_mul(s, v);
    });

    fn test_scalar_div<T: TestScalar>(v: Vec3<T>, s: T) 
    {
        let quot_fn = v.scalar_div(s);
        let quot_op = v / s;

        let expected = Vec3 {
            x: v.x / s,
            y: v.y / s,
            z: v.z / s,
        };

        assert_eq!(quot_fn, expected);
        assert_eq!(quot_op, expected);
        assert_eq!(quot_fn, quot_op);
    }

    scalar_test!(test_scalar_vec_div_scalar, |T| {
        let s = 10.0;
        let v = Vec3::<T>::new(1.0, 2.0, 3.0);
        test_scalar_div(v, s);
    });

    fn test_abs<T>(v: Vec3<T>) 
    where 
        T: TestScalar
    {
        let abs = v.abs();
        let expected = Vec3 {
            x: v.x.abs(),
            y: v.y.abs(),
            z: v.z.abs(),
        };

        assert_eq!(abs.x, expected.x);
        assert_eq!(abs.y, expected.y);
        assert_eq!(abs.z, expected.z);
    }

    scalar_test!(test_abs_scalar, |T| {
        let v = Vec3::<T>::new(-1.0, 1.0, 0.0);
        test_abs(v);
    });

    scalar_test!(test_dot_basic, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);
        let dot = a.dot(b);
        assert!(dot - T::from_f64(32.0) < T::TEST_EPS);
    });

    scalar_test!(test_dot_perpendicular, |T| {
        let a = Vec3::<T>::UNIT_X;
        let b = Vec3::<T>::UNIT_Y;
        let dot = a.dot(b);
        assert!(dot.abs() < T::TEST_EPS);
    });

    scalar_test!(test_cross_basic_axes, |T| {
        let x = Vec3::<T>::UNIT_X;
        let y = Vec3::<T>::UNIT_Y;
        let z = x.cross(y);
        let eps = T::TEST_EPS;

        assert!((z.x - T::ZERO).abs() < eps);
        assert!((z.y - T::ZERO).abs() < eps);
        assert!((z.z - T::ONE).abs() < eps);
    });

    scalar_test!(test_cross_known_case, |T| {
        let a = Vec3::<T>::new(2.0, 0.0, 0.0);
        let b = Vec3::<T>::new(0.0, 3.0, 0.0);
        let c = a.cross(b);
        assert!((c.z - T::from_f64(6.0)).abs() < T::TEST_EPS);
    });

    scalar_test!(test_cross_is_perpendicular, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);
        let c = a.cross(b);
        let d1 = c.dot(a);
        let d2 = c.dot(b);
        let eps = T::TEST_EPS;

        assert!(d1.abs() < eps);
        assert!(d2.abs() < eps);
    });

    scalar_test!(test_cross_self_is_zero, |T| {
        let v = Vec3::<T>::new(1.0, 2.0, 3.0);
        let cross = v.cross(v);
        let eps = T::TEST_EPS;

        assert!((cross.x).abs() < eps);
        assert!((cross.y).abs() < eps);
        assert!((cross.z).abs() < eps);
    });

    scalar_test!(test_cross_anticommutative, |T| {
        let a = Vec3::<T>::new(1.0, 3.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);
        let ab = a.cross(b);
        let ba = b.cross(a);
        let eps = T::TEST_EPS;

        assert!((ab.x + ba.x).abs() < eps);
        assert!((ab.y + ba.y).abs() < eps);
        assert!((ab.z + ba.z).abs() < eps);
    });

    scalar_test!(test_len_sq_3_4_5, |T| {
        let v = Vec3::<T>::new(3.0, 4.0, 0.0);

        assert!((v.length_squared() - T::from_f64(25.0)).abs() < T::TEST_EPS);
    });

    scalar_test!(test_len_3_4_5, |T| {
        let v = Vec3::<T>::new(3.0, 4.0, 0.0);

        assert!((v.length() - T::from_f64(5.0)).abs() < T::TEST_EPS);
    });

    scalar_test!(test_len_zero, |T| {
        let v = Vec3::<T>::ZERO;

        assert_eq!(v.length(), T::ZERO);
    });

    scalar_test!(test_normalize_basic, |T| {
        let v = Vec3::<T>::new(3.0, 4.0, 0.0);

        let n = v.normalize();
        let expected = v / v.length();

        let eps = T::TEST_EPS;
        assert!((n.length() - T::ONE).abs() < eps);
        
        n.assert_near(expected, T::TEST_EPS);
    });

    scalar_test!(
        #[should_panic(expected = "cannot normalize near-zero-length Vec3!")] 
        test_normalize_zero, |T| {
        let v = Vec3::<T>::ZERO;
        let n = v.normalize();

        assert_eq!(n, Vec3::<T>::ZERO);
    });

    scalar_test!(test_normalize_idempotent, |T| {
        let v = Vec3::<T>::new(2.0, 3.0, 6.0);
        let n1 = v.normalize();
        let n2 = n1.normalize();

        n1.assert_near(n2, T::TEST_EPS);
    });

    scalar_test!(test_is_unit, |T| {
       let x = Vec3::<T>::UNIT_X;
       let neg_x = -x;
       let normalized = Vec3::<T>::new(0.0, 100.0, -200.0).normalize();

        assert!(x.is_unit());
        assert!(neg_x.is_unit());
        assert!(normalized.is_unit());
    });

    scalar_test!(test_is_not_unit, |T| {
        let big_x = Vec3::<T>::new(100.0, 0.0, 0.0);
        let little_x = Vec3::<T>::new(1e-5, 0.0, 0.0);
        let close = Vec3::<T>::new(0.34, 0.34, 0.34);

        assert!(!big_x.is_unit());
        assert!(!little_x.is_unit());
        assert!(!close.is_unit());
    });

    scalar_test!(test_is_exactly_zero, |T| {
        let v = Vec3::<T>::ZERO;

        assert!(v.is_exactly_zero());
    });

    scalar_test!(test_is_not_exactly_zero, |T| {
        let v = Vec3::<T>::new(1e-9, 0.0, 0.0);

        assert!(!v.is_exactly_zero());
    });

    scalar_test!(test_is_near_zero, |T| {
        let v1 = Vec3::new(9e-9, 0.0, 0.0);
        let v2 = Vec3::new(-9e-9, 0.0, 0.0);

        assert!(v1.is_near_zero());
        assert!(v2.is_near_zero());
    });

    scalar_test!(test_is_not_near_zero, |T| {
        let v1 = Vec3::<T>::new(1e-7, 0.0, 0.0);
        let v2 = Vec3::<T>::new(-1e-7, 0.0, 0.0);
        let z = Vec3::<T>::UNIT_Z;

        assert!(!v1.is_near_zero());
        assert!(!v2.is_near_zero());
        assert!(!z.is_near_zero());
    });

    scalar_test!(test_angle_to_same_direction, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(2.0, 4.0, 6.0);

        assert!(a.angle_to(b).abs() < T::TEST_EPS);
        assert!(b.angle_to(a).abs() < T::TEST_EPS);
    });

    scalar_test!(test_angle_to_opposite_direction, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(-2.0, -4.0, -6.0);

        assert!((a.angle_to(b) - T::PI).abs() < T::TEST_EPS);
        assert!((b.angle_to(a) - T::PI).abs() < T::TEST_EPS);
    });

    scalar_test!(test_angle_to_compound_perpendicular, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, -5.0, 2.0);

        assert!((a.angle_to(b) - T::PI_OVER_2).abs() < T::TEST_EPS);
        assert!((b.angle_to(a) - T::PI_OVER_2).abs() < T::TEST_EPS);
    });

    scalar_test!(test_angle_to_45, |T| {
        let a = Vec3::<T>::new(1.0, 0.0, 0.0);
        let b = Vec3::<T>::new(1.0, 1.0, 0.0);

        assert!((a.angle_to(b) - T::PI_OVER_4).abs() < T::TEST_EPS);
        assert!((b.angle_to(a) - T::PI_OVER_4).abs() < T::TEST_EPS);
    });

    scalar_test!(
        #[should_panic(expected = "vector 'self' is near-zero, cannot compute angle!")]
        test_angle_to_0_length_self_vec3_panic, 
        |T| {
        let a = Vec3::<T>::new(1e-9, 0.0, 0.0);
        let b = Vec3::<T>::new(1.0, 1.0, 0.0);
        a.angle_to(b);
    });

    scalar_test!(
        #[should_panic(expected = "vector 'other' is near-zero, cannot compute angle!")]
        test_angle_to_0_length_other_vec3_panic, 
        |T| {
        let a = Vec3::<T>::new(1.0, 1.0, 0.0);
        let b = Vec3::<T>::new(1e-9, 0.0, 0.0);
        a.angle_to(b);
    });

    scalar_test!(test_is_perpendicular_to_unit, |T| {
        let unit_x_pos = Vec3::<T>::UNIT_X;
        let unit_y_pos = Vec3::<T>::UNIT_Y;
        let unit_z_pos = Vec3::<T>::UNIT_Z;
        let unit_x_neg = -Vec3::<T>::UNIT_X;
        let unit_y_neg = -Vec3::<T>::UNIT_Y;
        let unit_z_neg = -Vec3::<T>::UNIT_Z;

        assert!(unit_x_pos.is_perpendicular_to(unit_y_pos));
        assert!(unit_x_pos.is_perpendicular_to(unit_z_pos));
        assert!(unit_y_pos.is_perpendicular_to(unit_z_pos));

        assert!(unit_x_neg.is_perpendicular_to(unit_y_neg));
        assert!(unit_x_neg.is_perpendicular_to(unit_z_neg));
        assert!(unit_y_neg.is_perpendicular_to(unit_z_neg));

        assert!(unit_x_pos.is_perpendicular_to(unit_y_neg));
        assert!(unit_x_pos.is_perpendicular_to(unit_z_neg));
        assert!(unit_y_pos.is_perpendicular_to(unit_z_neg));
        assert!(unit_x_neg.is_perpendicular_to(unit_y_pos));
        assert!(unit_x_neg.is_perpendicular_to(unit_z_pos));
        assert!(unit_y_neg.is_perpendicular_to(unit_z_pos));
    });

    scalar_test!(test_is_not_perpendicular_to_self, |T| {
        let a = Vec3::<T>::new(1.0, 0.0, 0.0);
        let b = Vec3::<T>::new(1.0, 0.0, 0.0);

        assert!(!a.is_perpendicular_to(b));
        assert!(!b.is_perpendicular_to(a));
    });

    scalar_test!(test_near_0_is_not_perpendicular, |T| {
        let a = Vec3::<T>::new(1e-9, 0.0, 0.0);
        let b = Vec3::<T>::new(0.0, 1.0, 0.0);

        assert!(!a.is_perpendicular_to(b));
        assert!(!b.is_perpendicular_to(a));
    });

    scalar_test!(test_is_perpendicular_to_not_unit, |T| {
        let a = Vec3::<T>::new(10.0, 0.0, 0.0);
        let b = Vec3::<T>::new(0.0, 10.0, 0.0);

        assert!(a.is_perpendicular_to(b));
        assert!(b.is_perpendicular_to(a));
    });

    scalar_test!(test_is_perpendicular_to_compound, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, -5.0, 2.0);
        let c = Vec3::<T>::new(2.0, -3.0, 5.0);
        let d = Vec3::<T>::new(1.0, 4.0, 2.0);

        assert!(a.is_perpendicular_to(b));
        assert!(b.is_perpendicular_to(a));
        assert!(c.is_perpendicular_to(d));
        assert!(d.is_perpendicular_to(c));
    });

    scalar_test!(test_is_not_perpendicular_to_compound, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, 5.0, 6.0);

        assert!(!a.is_perpendicular_to(b));
        assert!(!b.is_perpendicular_to(a));
    });

    scalar_test!(test_is_parallel_to_self, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);

        assert!(a.is_parallel_to(a));
    });

    scalar_test!(test_is_parallel_to_same_direction, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(2.0, 4.0, 6.0);

        assert!(a.is_parallel_to(b));
        assert!(b.is_parallel_to(a));
    });

    scalar_test!(test_is_parallel_to_opposite_direction, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(-2.0, -4.0, -6.0);

        assert!(a.is_parallel_to(b));
        assert!(b.is_parallel_to(a));
    });

    scalar_test!(test_is_parallel_to_scale_invariant, |T| {
        let a = Vec3::<T>::new(10.0, 20.0, 30.0);
        let b = Vec3::<T>::new(20.0, 40.0, 60.0);
        let c = Vec3::<T>::new(0.1, 0.2, 0.3);

        assert!(a.is_parallel_to(b));
        assert!(a.is_parallel_to(c));
        assert!(b.is_parallel_to(-c));
    });

    scalar_test!(test_is_not_parallel_to_perpendicular, |T| {
        let a = Vec3::<T>::new(1.0, 2.0, 3.0);
        let b = Vec3::<T>::new(4.0, -5.0, 2.0);

        assert!(!a.is_parallel_to(b));
        assert!(!b.is_parallel_to(a));
    });

    scalar_test!(test_is_not_parallel_near_zero, |T| {
        let a = Vec3::<T>::new(1e-9, 0.0, 0.0);

        assert!(!a.is_parallel_to(a));
        assert!(!a.is_parallel_to(a));
    });

    scalar_test!(test_rotate_vec_unit_x, |T| {
        let x = Vec3::<T>::UNIT_X;
        let axis = Vec3::<T>::UNIT_Z;
        let angle_radians = T::PI;
        let expected = x * T::NEG_ONE;
        let rotated = x.rotate_axis_angle(axis, angle_radians);

        rotated.assert_near(expected, T::TEST_ROTATION_EPS);
    });

    scalar_test!(test_rotate_vec_big_x, |T| {
        let x = Vec3::<T>::new(100.0, 0.0, 0.0);
        let axis = Vec3::<T>::UNIT_Z;
        let angle_radians = T::PI;
        let expected = x * T::NEG_ONE;
        let rotated = x.rotate_axis_angle(axis, angle_radians);

        rotated.assert_near(expected, T::TEST_ROTATION_EPS);
    });

    scalar_test!(test_rotate_x_90_degree_around_z, |T| {
        let q = Quat::<T>::from_axis_angle(
            Vec3::<T>::UNIT_Z, T::PI_OVER_2);

        let rotated = Vec3::<T>::UNIT_X.rotate(q);

        rotated.assert_near(Vec3::<T>::UNIT_Y, T::TEST_EPS);
    });

    scalar_test!(test_rotate_y_90_degrees_around_x, |T| {
        let q = Quat::<T>::from_axis_angle(
            Vec3::<T>::UNIT_X, T::PI_OVER_2);

        let rotated = Vec3::<T>::UNIT_Y.rotate(q);

        rotated.assert_near(Vec3::<T>::UNIT_Z, T::TEST_EPS);
    });

    scalar_test!(test_rotate_z_90_degrees_around_y, |T| {
        let q = Quat::<T>::from_axis_angle(
            Vec3::<T>::UNIT_Y, T::PI_OVER_2);

        let rotated = Vec3::<T>::UNIT_Z.rotate(q);

        rotated.assert_near(Vec3::<T>::UNIT_X, T::TEST_EPS);
    });

    scalar_test!(test_rotate_leaves_axis_unchanged, |T| {
        let axis = Vec3::<T>::new(1.0, 2.0, 3.0).normalize();

        let q = Quat::<T>::from_axis_angle(axis, 1.234);

        axis.rotate(q).assert_near(axis, T::TEST_EPS);
    });

    scalar_test!(test_rotate_preserves_vector_length, |T| {
        let v = Vec3::<T>::new(4.0, -2.0, 7.0).normalize();

        let q = Quat::<T>::from_axis_angle(Vec3::<T>::new(1.0, 2.0, 3.0), 1.234);
        
        let rotated = v.rotate(q);

        assert!((rotated.length() - v.length()).abs() < T::TEST_EPS)
    });

    scalar_test!(test_inverse_rotation_restores_vector, |T| {
        let v = Vec3::<T>::new(4.0, -2.0, 7.0).normalize();

        let q = Quat::<T>::from_axis_angle(Vec3::<T>::new(1.0, 2.0, 3.0), 1.234);

        let restored = v.rotate(q).rotate(q.inverse());

        restored.assert_near(v, T::TEST_EPS);
    });

    scalar_test!(test_quaternion_composition_order, |T| {
        let rotate_x = Quat::<T>::from_axis_angle(
            Vec3::<T>::UNIT_X, T::PI_OVER_2);
        
        let rotate_z = Quat::<T>::from_axis_angle(
            Vec3::<T>::UNIT_Z, T::PI_OVER_2);

        let v = Vec3::<T>::UNIT_Y;

        let sequential = v.rotate(rotate_x).rotate(rotate_z);

        let combined = v.rotate(rotate_z * rotate_x);

        combined.assert_near(sequential, T::TEST_EPS);
        combined.assert_near(Vec3::<T>::UNIT_Z, T::TEST_EPS);
    });

    scalar_test!(test_quaternion_and_negation_rotate_identically, |T| {
        let q = Quat::<T>::from_axis_angle(Vec3::<T>::new(1.0, 2.0, 3.0), 1.234);

        let negative_q = Quat::<T>::new(
            -q.w,
            -q.x, 
            -q.y,
            -q.z
        );

        let v = Vec3::<T>::new(4.0, -2.0, 7.0);

        v.rotate(q).assert_near(v.rotate(negative_q), T::TEST_EPS);
    });



    scalar_test!(
        test_rotate_zero_vec, 
        |T| {
        let result = Vec3::<T>::ZERO.rotate_axis_angle(Vec3::UNIT_Z, T::PI);

        assert_eq!(result, Vec3::<T>::ZERO)
    });

    scalar_test!(
        #[should_panic(expected = "quaternion must be unit-length!")]
        test_rotation_rejects_non_unit_quat, 
        |T| {
        let v = Vec3::<T>::UNIT_X;
        let q = Quat::<T>::new(2.0, 0.0, 0.0, 0.0);

        let _ = v.rotate(q);
    });

    scalar_test!(test_print, |T| {
        let v = Vec3::<T>::new(1.0, 2.0, 3.0);
        println!("{v}");
    });
}
