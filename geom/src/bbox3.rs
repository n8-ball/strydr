use crate::{pt3::Pt3, scalar::Scalar};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bbox3<T: Scalar> {
    pub min: Pt3<T>,
    pub max: Pt3<T>,
}

impl<T: Scalar> Bbox3<T> {
    pub const ZERO: Self = Self {
        min: Pt3::<T>::ZERO,
        max: Pt3::<T>::ZERO,
    };

    pub fn new(min: Pt3<T>, max: Pt3<T>) -> Self {
        assert!(min.x <= max.x, "min.x must be < max.x");

        assert!(min.y <= max.y, "min.y must be < max.y");

        assert!(min.z <= max.z, "min.z must be < max.z");

        Self { min, max }
    }

    #[inline]
    pub fn union(a: Self, b: Self) -> Self {
        Self {
            min: Pt3::<T>::new(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            max: Pt3::<T>::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        }
    }

    #[inline]
    pub fn intersection(a: Self, b: Self) -> Self {
        Self {
            min: Pt3::<T>::new(
                a.min.x.max(b.min.x),
                a.min.y.max(b.min.y),
                a.min.z.max(b.min.z),
            ),
            max: Pt3::<T>::new(
                a.max.x.min(b.max.x),
                a.max.y.min(b.max.y),
                a.max.z.min(b.max.z),
            ),
        }
    }

    /// Mostly just to keep the api consistent.
    #[inline]
    pub fn difference(a: Self, _b: Self) -> Self {
        a
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::scalar_test;

    scalar_test!(test_new, |T| {
        let min = Pt3::<T>::MIN;
        let max = Pt3::<T>::MAX;

        let bbox = Bbox3::<T>::new(min, max);

        assert_eq!(bbox.min, min);
        assert_eq!(bbox.max, max);
    });

    scalar_test!(test_zero, |T| {
        let bbox = Bbox3::<T>::ZERO;

        assert_eq!(bbox.min, Pt3::<T>::ZERO);
        assert_eq!(bbox.max, Pt3::<T>::ZERO);
    });

    scalar_test!(test_union_corner_to_corner, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::ZERO;

        let min_b = Pt3::ZERO;
        let max_b = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let union = Bbox3::<T>::union(a, b);

        assert_eq!(union.min, min_a);
        assert_eq!(union.max, max_b);
    });

    scalar_test!(test_union_overlap, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_b = Pt3::<T>::new(5.0, 5.0, 5.0);
        let max_b = Pt3::<T>::new(15.0, 15.0, 15.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let union = Bbox3::<T>::union(a, b);

        assert_eq!(union.min, min_a);
        assert_eq!(union.max, max_b);
    });

    scalar_test!(test_union_identical, |T| {
        let min = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min, max);
        let b = Bbox3::<T>::new(min, max);

        let union = Bbox3::<T>::union(a, b);

        assert_eq!(union.min, min);
        assert_eq!(union.max, max);
    });

    scalar_test!(test_union_one_inside_other, |T| {
        let min_small = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_small = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_big = Pt3::<T>::new(-100.0, -100.0, -100.0);
        let max_big = Pt3::<T>::new(100.0, 100.0, 100.0);

        let a = Bbox3::<T>::new(min_small, max_small);
        let b = Bbox3::<T>::new(min_big, max_big);

        let union = Bbox3::<T>::union(a, b);

        assert_eq!(union.min, min_big);
        assert_eq!(union.max, max_big);
    });

    scalar_test!(test_intersection_corner_to_corner, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::ZERO;

        let min_b = Pt3::ZERO;
        let max_b = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let intersection = Bbox3::<T>::intersection(a, b);

        assert_eq!(intersection.min, max_a);
        assert_eq!(intersection.max, min_b);
    });

    scalar_test!(test_intersection_overlap, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_b = Pt3::<T>::new(5.0, 5.0, 5.0);
        let max_b = Pt3::<T>::new(15.0, 15.0, 15.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let intersection = Bbox3::<T>::intersection(a, b);

        assert_eq!(intersection.min, min_b);
        assert_eq!(intersection.max, max_a);
    });

    scalar_test!(test_intersection_identical, |T| {
        let min = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min, max);
        let b = Bbox3::<T>::new(min, max);

        let intersection = Bbox3::<T>::union(a, b);

        assert_eq!(intersection.min, min);
        assert_eq!(intersection.max, max);
    });

    scalar_test!(test_intersection_one_inside_other, |T| {
        let min_small = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_small = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_big = Pt3::<T>::new(-100.0, -100.0, -100.0);
        let max_big = Pt3::<T>::new(100.0, 100.0, 100.0);

        let a = Bbox3::<T>::new(min_small, max_small);
        let b = Bbox3::<T>::new(min_big, max_big);

        let intersection = Bbox3::<T>::intersection(a, b);

        assert_eq!(intersection.min, min_small);
        assert_eq!(intersection.max, max_small);
    });

    scalar_test!(test_difference_corner_to_corner, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::ZERO;

        let min_b = Pt3::ZERO;
        let max_b = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let difference = Bbox3::<T>::difference(a, b);

        assert_eq!(difference.min, min_a);
        assert_eq!(difference.max, max_a);
    });

    scalar_test!(test_difference_overlap, |T| {
        let min_a = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_a = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_b = Pt3::<T>::new(5.0, 5.0, 5.0);
        let max_b = Pt3::<T>::new(15.0, 15.0, 15.0);

        let a = Bbox3::<T>::new(min_a, max_a);
        let b = Bbox3::<T>::new(min_b, max_b);

        let difference = Bbox3::<T>::difference(a, b);

        assert_eq!(difference.min, min_a);
        assert_eq!(difference.max, max_a);
    });

    scalar_test!(test_difference_identical, |T| {
        let min = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max = Pt3::<T>::new(10.0, 10.0, 10.0);

        let a = Bbox3::<T>::new(min, max);
        let b = Bbox3::<T>::new(min, max);

        let difference = Bbox3::<T>::difference(a, b);

        assert_eq!(difference.min, min);
        assert_eq!(difference.max, max);
    });

    scalar_test!(test_difference_one_inside_other, |T| {
        let min_small = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let max_small = Pt3::<T>::new(10.0, 10.0, 10.0);

        let min_big = Pt3::<T>::new(-100.0, -100.0, -100.0);
        let max_big = Pt3::<T>::new(100.0, 100.0, 100.0);

        let a = Bbox3::<T>::new(min_small, max_small);
        let b = Bbox3::<T>::new(min_big, max_big);

        let difference = Bbox3::<T>::difference(a, b);

        assert_eq!(difference.min, min_small);
        assert_eq!(difference.max, max_small);
    });
}
