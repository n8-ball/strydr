use crate::{scalar::Scalar, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bbox3<T: Scalar> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl <T:Scalar> Bbox3<T> {
    fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    
    scalar_test!(test_new, |T| {
        let min = Vec3::<T>::MIN;
        let max = Vec3::<T>::MAX;

        let bbox = Bbox3::<T>::new(min, max);

        assert_eq!(bbox.min, min);
        assert_eq!(bbox.max, max);
    });
}


