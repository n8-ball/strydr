use crate::{pt3::Pt3, vec3::Vec3, scalar::Scalar};
use std::{fmt, fmt::{Formatter, Display}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane<T: Scalar> {
    origin: Pt3<T>,
    normal: Vec3<T>,
}

impl <T: Scalar> Plane<T> {
    fn new(origin: Pt3<T>, normal: Vec3<T>) -> Self {
        assert!(normal.is_unit(), 
            "plane normal must be unit!");

        Self { origin, normal } 
    }

    fn origin(self) -> Pt3<T> {
        self.origin
    }

    fn normal(self) -> Vec3<T> {
        self.normal
    }
}

impl<T: Scalar> Display for Plane<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position: {}\n direction: {}", self.origin, self.normal)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    scalar_test!(test_new, |T| {
        let pos = Pt3::<T>::ZERO; 
        let norm = Vec3::<T>::UNIT_X;

        let axis = Plane::new(pos, norm);

        assert_eq!(axis.origin, pos);
        assert_eq!(axis.normal, norm);
    });

    scalar_test!(
        #[should_panic(expected = "plane normal must be unit!")]
        test_new_nonunit_normal, 
        |T| {
        let position = Pt3::<T>::ZERO; 
        let normal = Vec3::<T>::new(2.0, 0.0, 0.0);

        let _ = Plane::new(position, normal);
    });

    scalar_test!(test_getters, |T| {
        let position = Pt3::<T>::ZERO; 
        let normal = Vec3::<T>::UNIT_X;
        let plane = Plane::<T>::new(position, normal);
        
        assert_eq!(plane.origin(), position);
        assert_eq!(plane.normal(), normal);
    });



    scalar_test!(test_print, |T| {
        let a = Plane::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}






