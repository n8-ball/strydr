use crate::{pt3::Pt3, vec3::Vec3, scalar::Scalar};
use std::{fmt, fmt::{Formatter, Display}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axis3<T: Scalar> {
    pub pos: Pt3<T>,
    pub dir: Vec3<T>,
}

impl<T: Scalar> Axis3<T> {
    pub const fn new(pos: Pt3<T>, dir: Vec3<T>) -> Self {
        Self { pos, dir }
    }
}

impl<T: Scalar> Display for Axis3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "position: {}\n direction: {}", self.pos, self.dir)
    }
}

mod tests {
    #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    scalar_test!(test_new_axis, |T| {
        let pos = Pt3::<T>::ZERO; 
        let dir = Vec3::<T>::UNIT_X;

        let axis = Axis3::new(pos, dir);

        assert_eq!(axis.pos, pos);
        assert_eq!(axis.dir, dir);


    });

    scalar_test!(test_print, |T| {
        let a = Axis3::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}


