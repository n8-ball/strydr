use geom::{pt3::Pt3, scalar::Scalar, bbox3::Bbox3};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere<T: Scalar> {
    origin: Pt3<T>, 
    radius: T,
}

impl<T: Scalar> Sphere<T> {
    fn new(origin: Pt3<T>, radius: T) -> Self {
        assert!(radius > T::ZERO, 
            "sphere radius must be greater than zero!");
        
        Self { origin, radius }
    }

    fn origin(self) -> Pt3<T> {
        self.origin
    }

    fn radius(self) -> T {
        self.radius
    }
    pub fn calculate_bbox(&self) -> Bbox3<T> {
        let max = Pt3::new(
            self.origin.x + self.radius,
            self.origin.y + self.radius,
            self.origin.z + self.radius
        );

        let min = Pt3::new(
            self.origin.x - self.radius,
            self.origin.y - self.radius,
            self.origin.z - self.radius
        );

        Bbox3::new(min, max)
    }

    pub fn signed_distance(&self, p: Pt3<T>) -> T {
        let x2 = (p.x - self.origin.x).powi(2);
        let x2y2 = (p.y - self.origin.y).powi(2) + x2;
        (x2y2 + (p.z - self.origin.z).powi(2)).sqrt() - self.radius
    }
}

#[cfg(test)]
mod tests {
use super::*;
    use geom::scalar_test;

    scalar_test!(test_new, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(origin, radius);

        assert_eq!(sphere.radius, radius);
        assert_eq!(sphere.origin, origin);
    });

    scalar_test!(
        #[should_panic(expected = "sphere radius must be greater than zero!")]
        test_new_rejects_zero_radius, |T| {
        let radius = 0.0;
        let origin = Pt3::<T>::ZERO;
        let _ = Sphere::new(origin, radius);
    });

    scalar_test!(test_getters, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(origin, radius);

        assert_eq!(sphere.radius(), radius);
        assert_eq!(sphere.origin(), origin);
    });

    scalar_test!(test_calculate_bbox_basic, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(origin, radius);
        let bbox = sphere.calculate_bbox();

        let expected_min = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let expected_max = Pt3::<T>::new(10.0, 10.0, 10.0);
        let expected = Bbox3::new(expected_min, expected_max);

        assert_eq!(bbox, expected);
    });

    scalar_test!(test_calculate_bbox_not_at_origin, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::new(10.0, -10.0, 10.0);

        let sphere = Sphere::new(origin, radius);
        let bbox = sphere.calculate_bbox();

        let expected_min = Pt3::<T>::new(0.0, -20.0, 0.0);
        let expected_max = Pt3::<T>::new(20.0, 0.0, 20.0);
        let expected = Bbox3::new(expected_min, expected_max);

        assert_eq!(bbox, expected);
    });

    scalar_test!(test_signed_distance_basic, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(origin, radius);
        let sample_origin = Pt3::<T>::ZERO;
        let sample_boundary = Pt3::<T>::new(radius, 0.0, 0.0);
        let sample_between = Pt3::<T>::new(radius * 0.5, 0.0, 0.0);

        assert_eq!(sphere.signed_distance(sample_origin), -radius);
        assert_eq!(sphere.signed_distance(sample_boundary), 0.0);
        assert_eq!(sphere.signed_distance(sample_between), -radius * 0.5);
    });

}

