use crate::{scalar::Scalar, vec3::Vec3};
use std::{
    fmt,
    fmt::{Display, Formatter},
    ops::Mul,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

const LENGTH_NEAR_ZERO_EPS: f64 = 1e-8;
const UNIT_LENGTH_EPS: f64 = 1e-6;

impl<T: Scalar> Quat<T> {
    pub const IDENTITY: Self = Self {
        w: T::ONE,
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
    };

    pub const fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }

    pub fn length_squared(self) -> T {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();

        assert!(
            len > T::from_f64(LENGTH_NEAR_ZERO_EPS),
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
        let len_sq = self.length_squared();

        assert!(
            len_sq > T::from_f64(LENGTH_NEAR_ZERO_EPS * LENGTH_NEAR_ZERO_EPS),
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
            "cannot rotate around a zero-length axis direction!"
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

    pub fn is_unit(self) -> bool {
        let len_sq = self.length_squared();
        (len_sq - T::ONE).abs() < T::from_f64(UNIT_LENGTH_EPS)
    }

    pub fn assert_near(self, b: Quat<T>, eps: T) {
        assert!(
            (self.w - b.w).abs() < eps,
            "left w: {} != right w: {}",
            self.w,
            b.w
        );
        assert!(
            (self.x - b.x).abs() < eps,
            "left x: {} != right x: {}",
            self.x,
            b.x
        );
        assert!(
            (self.y - b.y).abs() < eps,
            "left y: {} != right y: {}",
            self.y,
            b.y
        );
        assert!(
            (self.z - b.z).abs() < eps,
            "left z: {} != right z: {}",
            self.z,
            b.z
        );
    }
}

/// Quat * Quat
impl<T: Scalar> Mul for Quat<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,

            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,

            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,

            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
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
    use crate::{scalar::TestScalar, scalar_test};

    scalar_test!(test_new, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(q.w, T::from_f64(1.0));
        assert_eq!(q.x, T::from_f64(2.0));
        assert_eq!(q.y, T::from_f64(3.0));
        assert_eq!(q.z, T::from_f64(4.0));
    });

    scalar_test!(test_identity, |T| {
        let q = Quat::<T>::IDENTITY;
        assert_eq!(q.w, T::ONE);
        assert_eq!(q.x, T::ZERO);
        assert_eq!(q.y, T::ZERO);
        assert_eq!(q.z, T::ZERO);
    });

    scalar_test!(test_length_squared, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

        assert!((q.length_squared() - 30.0).abs() < T::TEST_EPS)
    });

    scalar_test!(test_identity_length_squared_is_one, |T| {
        let q = Quat::<T>::IDENTITY;

        assert!((q.length_squared() - T::ONE).abs() < T::TEST_EPS)
    });

    scalar_test!(test_identity_has_unit_length, |T| {
        assert!((Quat::<T>::IDENTITY.length() - T::ONE).abs() < T::TEST_EPS);
    });

    scalar_test!(test_normalize_produces_unit_quaternion, |T| {
        let normalized = Quat::<T>::new(1.0, 2.0, 3.0, 4.0).normalize();

        assert!((normalized.length() - T::ONE).abs() < T::TEST_EPS)
    });

    scalar_test!(test_normalize_known_quat, |T| {
        let normalized = Quat::<T>::new(2.0, 0.0, 0.0, 0.0).normalize();
        let expected = Quat::<T>::IDENTITY;
        normalized.assert_near(expected, T::TEST_EPS)
    });

    scalar_test!(test_normalize_idempotent, |T| {
        let q = Quat::<T>::new(1.0, -2.0, 3.0, -4.0);
        let once = q.normalize();
        let twice = once.normalize();
        once.assert_near(twice, T::TEST_EPS)
    });

    scalar_test!(
        #[should_panic(expected = "cannot normalize a zero-length quaternion")]
        test_normalizing_zero_quaternion_panics,
        |T| {
            let _ = Quat::<T>::new(0.0, 0.0, 0.0, 0.0).normalize();
        }
    );

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

    scalar_test!(test_conjugate_twice_returns_original, |T| {
        let q = Quat::<T>::new(-1.0, 2.0, -3.0, 4.0);
        let once = q.conjugate();
        let twice = once.conjugate();
        assert_eq!(twice, q);
    });

    scalar_test!(test_identity_conjugate_is_identity, |T| {
        assert_eq!(Quat::<T>::IDENTITY, Quat::<T>::IDENTITY.conjugate());
    });

    scalar_test!(test_conjudate_times_quat_is_norm_squared, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let result = q.conjugate() * q;
        let expected = Quat::<T>::new(30.0, 0.0, 0.0, 0.0);
        assert_eq!(result, expected);
    });

    scalar_test!(
        test_identity_multiplication_does_not_change_quaternion,
        |T| {
            let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

            assert_eq!(Quat::IDENTITY * q, q);
            assert_eq!(q * Quat::IDENTITY, q);
        }
    );

    scalar_test!(test_quaternion_times_inverse_equals_identity, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let result = q * q.inverse();

        result.assert_near(Quat::IDENTITY, T::TEST_EPS);
    });

    scalar_test!(test_inverse_times_quaternion_is_identity, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let result = q.inverse() * q;

        result.assert_near(Quat::IDENTITY, T::TEST_EPS);
    });

    scalar_test!(test_inverse_of_identity_is_identity, |T| {
        let q = Quat::<T>::IDENTITY;
        let inverse = q.inverse();

        inverse.assert_near(Quat::IDENTITY, T::TEST_EPS);
    });

    scalar_test!(test_inverse_of_inverse_is_original, |T| {
        let q = Quat::<T>::IDENTITY;
        let once = q.inverse();
        let twice = once.inverse();

        twice.assert_near(q, T::TEST_EPS);
    });

    scalar_test!(test_unit_quaternion_inverse_equals_conjugate, |T| {
        let q = Quat::<T>::from_axis_angle(Vec3::<T>::new(1.0, 2.0, 3.0), 1.25);

        q.inverse().assert_near(q.conjugate(), T::TEST_EPS);
    });

    scalar_test!(
        #[should_panic(expected = "cannot invert a zero-length quaternion")]
        test_inverse_zero_quaternion_panics,
        |T| {
            let _ = Quat::<T>::new(0.0, 0.0, 0.0, 0.0).inverse();
        }
    );

    scalar_test!(test_i_times_j_equals_k, |T| {
        let i = Quat::<T>::new(0.0, 1.0, 0.0, 0.0);
        let j = Quat::<T>::new(0.0, 0.0, 1.0, 0.0);
        let k = Quat::<T>::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(i * j, k);
    });

    scalar_test!(test_j_times_k_equals_i, |T| {
        let i = Quat::<T>::new(0.0, 1.0, 0.0, 0.0);
        let j = Quat::<T>::new(0.0, 0.0, 1.0, 0.0);
        let k = Quat::<T>::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(j * k, i);
    });

    scalar_test!(test_k_times_i_equals_j, |T| {
        let i = Quat::<T>::new(0.0, 1.0, 0.0, 0.0);
        let j = Quat::<T>::new(0.0, 0.0, 1.0, 0.0);
        let k = Quat::<T>::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(k * i, j);
    });

    scalar_test!(test_j_times_i_equals_negative_k, |T| {
        let i = Quat::<T>::new(0.0, 1.0, 0.0, 0.0);
        let j = Quat::<T>::new(0.0, 0.0, 1.0, 0.0);
        let negative_k = Quat::<T>::new(0.0, 0.0, 0.0, -1.0);
        assert_eq!(j * i, negative_k);
    });

    scalar_test!(test_pure_basic_quaternion_squared_is_negative_one, |T| {
        let i = Quat::<T>::new(0.0, 1.0, 0.0, 0.0);
        let negative_one = Quat::<T>::new(-1.0, 0.0, 0.0, 0.0);
        assert_eq!(i * i, negative_one);
    });

    scalar_test!(test_quaternion_multiplication_is_not_commutative, |T| {
        let a = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let b = Quat::<T>::new(5.0, 6.0, 7.0, 8.0);
        assert_ne!(a * b, b * a);
    });

    scalar_test!(test_quaternion_multiplication_is_associative, |T| {
        let a = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        let b = Quat::<T>::new(-2.0, 0.5, 1.5, -3.0);
        let c = Quat::<T>::new(4.0, -1.0, 2.0, 0.25);
        let left = (a * b) * c;
        let right = a * (b * c);

        left.assert_near(right, T::TEST_EPS);
    });

    scalar_test!(test_axis_angle_normalizes_axis, |T| {
        let angle = 0.75;
        let unit_axis = Vec3::<T>::UNIT_X;
        let scaled_axis = unit_axis * 25.0;

        let from_unit = Quat::<T>::from_axis_angle(unit_axis, angle);
        let from_scaled = Quat::<T>::from_axis_angle(scaled_axis, angle);

        from_unit.assert_near(from_scaled, T::TEST_EPS);
    });

    scalar_test!(test_negative_angle_is_conjugate_of_positive_angle, |T| {
        let axis = Vec3::<T>::new(1.0, 2.0, 3.0);

        let angle = 0.75;

        let positive = Quat::<T>::from_axis_angle(axis, angle);
        let negative = Quat::<T>::from_axis_angle(axis, -angle);

        negative.assert_near(positive.conjugate(), T::TEST_EPS);
    });

    scalar_test!(test_negative_axis_and_angle_produces_same_quaternion, |T| {
        let axis = Vec3::<T>::new(1.0, 2.0, 3.0);
        let angle = 0.75;

        let positive = Quat::<T>::from_axis_angle(axis, angle);
        let negative = Quat::<T>::from_axis_angle(-axis, -angle);

        negative.assert_near(positive, T::TEST_EPS);
    });

    scalar_test!(
        #[should_panic(expected = "cannot rotate around a zero-length axis direction")]
        test_axis_angle_rejects_zero_axis,
        |T| {
            let _ = Quat::<T>::from_axis_angle(Vec3::<T>::ZERO, T::PI);
        }
    );

    scalar_test!(test_is_unit, |T| {
        let identity = Quat::<T>::IDENTITY;
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0).normalize();

        assert!(identity.is_unit());
        assert!(q.is_unit());
    });

    scalar_test!(test_is_not_unit, |T| {
        let q = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);

        assert!(!q.is_unit());
    });

    scalar_test!(test_print, |T| {
        let v = Quat::<T>::new(1.0, 2.0, 3.0, 4.0);
        println!("{v}");
    });
}
