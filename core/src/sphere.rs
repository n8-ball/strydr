use geom::{bbox3::Bbox3, pt3::Pt3, scalar::Scalar};

use crate::{dag::NodeId, sdf::Sdf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere<T: Scalar> {
    id: NodeId,
    origin: Pt3<T>,
    radius: T,
}

impl<T: Scalar> Sphere<T> {
    pub fn new(id: usize, origin: Pt3<T>, radius: T) -> Self {
        assert!(radius > T::ZERO, "sphere radius must be greater than zero!");

        Self {
            id: NodeId(id),
            origin,
            radius,
        }
    }

    pub fn origin(self) -> Pt3<T> {
        self.origin
    }

    pub fn radius(self) -> T {
        self.radius
    }
    pub fn calculate_bbox(&self) -> Bbox3<T> {
        let max = Pt3::new(
            self.origin.x + self.radius,
            self.origin.y + self.radius,
            self.origin.z + self.radius,
        );

        let min = Pt3::new(
            self.origin.x - self.radius,
            self.origin.y - self.radius,
            self.origin.z - self.radius,
        );

        Bbox3::new(min, max)
    }
}

impl<T: Scalar> Sdf<T> for Sphere<T> {
    fn signed_distance(&self, p: Pt3<T>) -> T {
        let dx = p.x - self.origin.x;
        let dy = p.y - self.origin.y;
        let dz = p.z - self.origin.z;

        (dx * dx + dy * dy + dz * dz).sqrt() - self.radius
    }

    fn id(&self) -> NodeId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom::scalar_test;

    scalar_test!(test_new, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(0, origin, radius);

        assert_eq!(sphere.radius, radius);
        assert_eq!(sphere.origin, origin);
    });

    scalar_test!(
        #[should_panic(expected = "sphere radius must be greater than zero!")]
        test_new_rejects_zero_radius,
        |T| {
            let radius = 0.0;
            let origin = Pt3::<T>::ZERO;

            let _ = Sphere::new(0, origin, radius);
        }
    );

    scalar_test!(test_getters, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(0, origin, radius);

        assert_eq!(sphere.radius(), radius);
        assert_eq!(sphere.origin(), origin);
    });

    scalar_test!(test_calculate_bbox_basic, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(0, origin, radius);
        let bbox = sphere.calculate_bbox();

        let expected_min = Pt3::<T>::new(-10.0, -10.0, -10.0);
        let expected_max = Pt3::<T>::new(10.0, 10.0, 10.0);
        let expected = Bbox3::new(expected_min, expected_max);

        assert_eq!(bbox, expected);
    });

    scalar_test!(test_calculate_bbox_not_at_origin, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::new(10.0, -10.0, 10.0);

        let sphere = Sphere::new(0, origin, radius);
        let bbox = sphere.calculate_bbox();

        let expected_min = Pt3::<T>::new(0.0, -20.0, 0.0);
        let expected_max = Pt3::<T>::new(20.0, 0.0, 20.0);
        let expected = Bbox3::new(expected_min, expected_max);

        assert_eq!(bbox, expected);
    });

    scalar_test!(test_signed_distance_basic, |T| {
        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let sphere = Sphere::new(0, origin, radius);
        let sample_origin = Pt3::<T>::ZERO;
        let sample_boundary = Pt3::<T>::new(radius, 0.0, 0.0);
        let sample_between = Pt3::<T>::new(radius * 0.5, 0.0, 0.0);

        assert_eq!(sphere.signed_distance(sample_origin), -radius);
        assert_eq!(sphere.signed_distance(sample_boundary), 0.0);
        assert_eq!(sphere.signed_distance(sample_between), -radius * 0.5);
    });
}
