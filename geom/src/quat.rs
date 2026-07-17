use std::{fmt::{Formatter, Display}, fmt, ops::Mul};
use crate::{vec3::Vec3, scalar::Scalar};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat<T> {
    pub w: T,
    pub x: T, 
    pub y: T, 
    pub z: T, 
}

const LENGTH_NEAR_ZERO_EPS: f64 = 1e-8;

impl<T: Scalar> Quat<T> {

    pub const IDENTITY: Self = Self { w: T::ONE, x: T::ZERO, y: T::ZERO, z: T::ZERO };

    pub const fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }

    pub fn len_sq(self) -> T {
        self.w * self.w
        + self.x * self.x
        + self.y * self.y
        + self.z * self.z
    }

    pub fn len(self) -> T {
        self.len_sq().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.len();

        assert!(len > T::from_f64(LENGTH_NEAR_ZERO_EPS), 
            "cannot normalize a zero-length quaternion!"
        );

        Self {
            w: self.w / len,
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn inverse(self) -> Self {
        let len_sq = self.len_sq();

        assert!(len_sq > T::from_f64(LENGTH_NEAR_ZERO_EPS * LENGTH_NEAR_ZERO_EPS), 
            "cannot invert a zero-length quaternion!"
        );

        let conjugate = self.conjugate();

        Self {
            w: conjugate.w / len_sq,
            x: conjugate.x / len_sq,
            y: conjugate.y / len_sq,
            z: conjugate.z / len_sq,
        }
    }

    pub fn from_axis_angle(axis_dir: Vec3<T>, angle_radians: T) -> Self {
        assert!(
            !axis_dir.is_near_zero(),
            "cannot rotate around a near-zero-length axis direction!"
        );

        let axis = axis_dir.normalize();

        let half_angle = angle_radians * T::HALF;
        let sin_half = half_angle.sin();

        Self {
            w: half_angle.cos(),
            x: axis.x * sin_half,
            y: axis.y * sin_half,
            z: axis.z * sin_half,
        }
    }

    fn assert_near(self, b: Quat<T>, eps: T) {
        assert!((self.w - b.w).abs() < eps,
            "left w: {} != right w: {}", self.w, b.w);
        assert!((self.x - b.x).abs() < eps,
            "left x: {} != right x: {}", self.x, b.x);
        assert!((self.y - b.y).abs() < eps,
            "left y: {} != right y: {}", self.y, b.y);
        assert!((self.z - b.z).abs() < eps,
            "left z: {} != right z: {}", self.z, b.z);
    }


}

/// Quat * Quat
impl<T: Scalar> Mul for Quat<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w 
            - self.x * rhs.x
            - self.y * rhs.y 
            - self.z * rhs.z,

            x: self.w * rhs.x
            + self.x * rhs.w
            + self.y * rhs.z
            - self.z * rhs.y,

            y: self.w * rhs.y 
            - self.x * rhs.z
            + self.y * rhs.w
            + self.z * rhs.x,

            z: self.w * rhs.z
            + self.x * rhs.y
            - self.y * rhs.x
            + self.z * rhs.w
        }     
    } 
}

impl<T: Scalar> Display for Quat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}, {}>", self.w, self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scalar_test, scalar::TestScalar};

    scalar_test!(test_new, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(q.w, T::from_f64(1.0));
        assert_eq!(q.x, T::from_f64(2.0));
        assert_eq!(q.y, T::from_f64(3.0));
        assert_eq!(q.z, T::from_f64(4.0));
    });

    scalar_test!(test_identity, |T| {
        let q = Quat::<T>::IDENTITY;
        assert_eq!(q.w, T::from_f64(1.0));
        assert_eq!(q.x, T::from_f64(0.0));
        assert_eq!(q.y, T::from_f64(0.0));
        assert_eq!(q.z, T::from_f64(0.0));
    });

    scalar_test!(test_identity_has_unit_length, |T| {
        assert!(Quat::<T>::IDENTITY.len() - T::ONE < T::TEST_EPS);
    });

    scalar_test!(test_normalize_produces_unit_quaternion, |T| {
        let normalized = Quat::<T>::new(1.0, 2.0, 3.0, 4.0).normalize();
        assert!(normalized.len() - T::ONE < T::TEST_EPS)

    });

    scalar_test!(
        #[should_panic(expected = "cannot normalize a zero-length quaternion")]
        test_normalizing_zero_quaternion_panics, 
        |T| {
        let _ = Quat::<T>::new(0.0, 0.0, 0.0, 0.0).normalize();
    });

    scalar_test!(test_conjugate_negates_vector_part, |T| {
        let q = Quat::<T>::new(1.0, 2.0, -3.0, 4.0);
        let conjugate = q.conjugate();
        let expected = Quat {
            w: q.w,
            x: -q.x,
            y: -q.y,
            z: -q.z,
        };

        assert_eq!(conjugate, expected);
    });

    scalar_test!(test_identity_multiplication_does_not_change_quaternion, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(Quat::IDENTITY * q, q); 
        assert_eq!(q * Quat::IDENTITY, q); 
    });

    scalar_test!(test_quaternion_times_inverse_equals_identity, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let result = q * q.inverse();

        Quat::IDENTITY.assert_near(result, T::TEST_EPS); 
        result.assert_near(Quat::IDENTITY, T::TEST_EPS); 
    });

     scalar_test!(test_print, |T| {
        let v = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        println!("{v}");
    });  
}
