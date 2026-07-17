use crate::{pt3::Pt3, vec3::Vec3, scalar::Scalar};
use std::{fmt, fmt::{Formatter, Display}};

pub struct Plane<T: Scalar> {
    pos: Pt3<T>,
    norm: Vec3<T>,
}

impl <T: Scalar> Plane<T> {
    fn new(pos: Pt3<T>, norm: Vec3<T>) -> Self {
        Self { pos, norm } 
    }
}

impl<T: Scalar> Display for Plane<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "position: {}\n direction: {}", self.pos, self.norm)
    }
}

mod tests {
    #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    scalar_test!(test_new_axis, |T| {
        let pos = Pt3::<T>::ZERO; 
        let norm = Vec3::<T>::UNIT_X;

        let axis = Plane::new(pos, norm);

        assert_eq!(axis.pos, pos);
        assert_eq!(axis.norm, norm);


    });

    scalar_test!(test_print, |T| {
        let a = Plane::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}






