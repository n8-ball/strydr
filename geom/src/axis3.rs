use crate::{pt3::Pt3, vec3::Vec3, scalar::Scalar};
use std::{fmt, fmt::{Formatter, Display}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axis3<T: Scalar> {
    origin: Pt3<T>,
    direction: Vec3<T>,
}

impl<T: Scalar> Axis3<T> {
    pub fn new(origin: Pt3<T>, direction: Vec3<T>) -> Self {
        assert!(direction.is_unit(), 
            "axis direction must be unit!");

        Self { origin, direction }
    }

    pub fn origin(self) -> Pt3<T> {
        self.origin
    }

    pub fn direction(self) -> Vec3<T> {
        self.direction
    }
}

impl<T: Scalar> Display for Axis3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position: {}\n direction: {}", self.origin, self.direction)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    scalar_test!(test_new, |T| {
        let pos = Pt3::<T>::ZERO; 
        let dir = Vec3::<T>::UNIT_X;

        let axis = Axis3::new(pos, dir);

        assert_eq!(axis.origin, pos);
        assert_eq!(axis.direction, dir);
    });

    scalar_test!(
        #[should_panic(expected = "axis direction must be unit!")]
        test_new_nonunit_dir, 
        |T| {
        let pos = Pt3::<T>::ZERO; 
        let dir = Vec3::<T>::new(2.0, 0.0, 0.0);

        let _ = Axis3::new(pos, dir);
    });

    scalar_test!(test_getters, |T| {
        let pos = Pt3::<T>::ZERO; 
        let dir = Vec3::<T>::UNIT_X;
        let axis = Axis3::new(pos, dir);
        
        assert_eq!(axis.origin, pos);
        assert_eq!(axis.direction, dir);
    });

    scalar_test!(test_print, |T| {
        let a = Axis3::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}


