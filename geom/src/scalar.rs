use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Scalar:
    Copy
    + Clone
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;
    const HALF: Self;
    const EPSILON: Self;
    const PI: Self;
    const PI_OVER_2: Self;
    const PI_OVER_4: Self;

    const MIN: Self;
    const MAX: Self;

    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn abs(self) -> Self;

    fn powi(self, n: i32) -> Self;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;

    fn to_f64(self) -> f64;

    fn round(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const NEG_ONE: Self = -1.0;
    const HALF: Self = 0.5;

    const EPSILON: Self = f32::EPSILON;
    const PI: Self = std::f32::consts::PI;
    const PI_OVER_2: Self = std::f32::consts::PI * 0.5;
    const PI_OVER_4: Self = std::f32::consts::PI * 0.25;

    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn from_f32(value: f32) -> Self {
        value
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn round(self) -> Self {
        self.round()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const NEG_ONE: Self = -1.0;
    const HALF: Self = 0.5;
    const EPSILON: Self = f64::EPSILON;
    const PI: Self = std::f64::consts::PI;
    const PI_OVER_2: Self = std::f64::consts::PI * 0.5;
    const PI_OVER_4: Self = std::f64::consts::PI * 0.25;

    const MIN: Self = f64::MIN;
    const MAX: Self = f64::MAX;

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn from_f32(value: f32) -> Self {
        value as f64
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn round(self) -> Self {
        self.round()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

pub trait TestScalar: Scalar {
    const TEST_EPS: Self;
    const TEST_ROTATION_EPS: Self;
}

impl TestScalar for f32 {
    const TEST_EPS: Self = 1e-6;
    const TEST_ROTATION_EPS: Self = 1e-5;
}

impl TestScalar for f64 {
    const TEST_EPS: Self = 1e-12;
    const TEST_ROTATION_EPS: Self = 1e-10;
}

#[macro_export]
macro_rules! scalar_test {
    (
        $(#[$attr:meta])*
        $name:ident,
        |$t:ident| $body:block) => {
        mod $name {
            #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
            use super::*;

            #[test]
            $(#[$attr])*
            fn f32() {
                type $t = f32;
                $body
            }

            #[test]
            $(#[$attr])*
            fn f64() {
                type $t = f64;
                $body
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_scalar_generic_add<T: Scalar>(a: T, b: T) -> T {
        a + b
    }

    fn test_scalar_generic_sub<T: Scalar>(lhs: T, rhs: T) -> T {
        lhs - rhs
    }

    fn test_scalar_generic_mul<T: Scalar>(a: T, b: T) -> T {
        a * b
    }

    fn test_scalar_generic_div<T: Scalar>(lhs: T, rhs: T) -> T {
        lhs / rhs
    }

    #[test]
    fn test_scalar_ops() {
        // Add
        assert_eq!(test_scalar_generic_add::<f32>(1.0, 2.0), 3.0);
        assert_eq!(test_scalar_generic_add::<f64>(1.0, 2.0), 3.0);

        // Sub
        assert_eq!(test_scalar_generic_sub::<f32>(1.0, 2.0), -1.0);
        assert_eq!(test_scalar_generic_sub::<f64>(1.0, 2.0), -1.0);

        // Mul
        assert_eq!(test_scalar_generic_mul::<f32>(1.0, 2.0), 2.0);
        assert_eq!(test_scalar_generic_mul::<f64>(1.0, 2.0), 2.0);

        // Div
        assert_eq!(test_scalar_generic_div::<f32>(1.0, 2.0), Scalar::HALF);
        assert_eq!(test_scalar_generic_div::<f64>(1.0, 2.0), Scalar::HALF);
    }
}
