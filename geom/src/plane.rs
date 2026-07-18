use crate::{Axis3, pt3::Pt3, scalar::Scalar, vec3::Vec3};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

//      ^
//     -|- - - -
//   /  |     /
//  /   *    /
// /        /
// - - - - -

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane<T: Scalar> {
    origin: Pt3<T>,
    normal: Vec3<T>,
}

impl<T: Scalar> Plane<T> {
    fn new(origin: Pt3<T>, normal: Vec3<T>) -> Self {
        assert!(normal.is_unit(), "plane normal must be unit!");

        Self { origin, normal }
    }

    fn origin(self) -> Pt3<T> {
        self.origin
    }

    fn normal(self) -> Vec3<T> {
        self.normal
    }

    pub fn translate(self, v: Vec3<T>) -> Self {
        Self {
            origin: Pt3::new(
                self.origin.x + v.x,
                self.origin.y + v.y,
                self.origin.z + v.z,
            ),
            normal: self.normal,
        }
    }

    pub fn rotate_about_axis(self, axis: Axis3<T>, angle_radians: T) -> Self {
        Plane::new(
            self.origin.rotate_about_axis(axis, angle_radians),
            self.normal
                .rotate_axis_angle(axis.direction(), angle_radians),
        )
    }

    pub fn assert_near(self, other: Self, eps: T) {
        self.origin.assert_near(other.origin, eps);
        self.normal.assert_near(other.normal, eps);
    }
}

impl<T: Scalar> Display for Plane<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position: {}\n direction: {}", self.origin, self.normal)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    // This is needed to prevent an unused import warning on use super::*; for some reason.
    use super::*;
    use crate::{scalar::TestScalar, scalar_test};

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
        }
    );

    scalar_test!(test_getters, |T| {
        let position = Pt3::<T>::ZERO;
        let normal = Vec3::<T>::UNIT_X;
        let plane = Plane::<T>::new(position, normal);

        assert_eq!(plane.origin(), position);
        assert_eq!(plane.normal(), normal);
    });

    scalar_test!(test_translate_basic, |T| {
        let plane = Plane::<T>::new(Pt3::<T>::new(-10.0, -10.0, -10.0), Vec3::<T>::UNIT_X);

        let translated = plane.translate(Vec3::<T>::new(10.0, 10.0, 10.0));

        translated.assert_near(
            Plane::<T>::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X),
            T::TEST_EPS,
        );
    });

    scalar_test!(test_translate_translate_inverse, |T| {
        let a = Plane::<T>::new(Pt3::new(-1.0, 4.0, 2.0), Vec3::UNIT_X);

        let v = Vec3::new(3.0, -5.0, 1.0);

        let moved = a.translate(v);
        let back = moved.translate(-v);

        back.origin.assert_near(a.origin, T::TEST_EPS);
        back.normal.assert_near(a.normal, T::TEST_EPS);
    });

    scalar_test!(test_normal_translate_inverse, |T| {
        let a = Plane::<T>::new(Pt3::new(-1.0, 4.0, 2.0), Vec3::UNIT_X);

        let v = Vec3::new(3.0, -5.0, 1.0);

        let moved = a.translate(v);
        let back = moved.translate(-v);

        back.origin.assert_near(a.origin, T::TEST_EPS);
        back.normal.assert_near(a.normal, T::TEST_EPS);
    });

    scalar_test!(test_plane_rotate_identity, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Plane::<T>::new(Pt3::new(1.0, 2.0, 3.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::ZERO);

        rotated.origin.assert_near(a.origin, T::TEST_EPS);
        rotated.normal.assert_near(a.normal, T::TEST_EPS);
    });
    scalar_test!(test_plane_rotate_90deg_z, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Plane::<T>::new(Pt3::new(1.0, 0.0, 0.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        rotated
            .origin
            .assert_near(Pt3::new(0.0, 1.0, 0.0), T::TEST_EPS);

        rotated.normal.assert_near(Vec3::UNIT_Y, T::TEST_EPS);
    });

    scalar_test!(test_plane_rotate_offset_axis, |T| {
        let axis = Axis3::<T>::new(
            Pt3::new(1.0, 0.0, 0.0), // shifted axis
            Vec3::UNIT_Z,
        );

        let a = Plane::<T>::new(Pt3::new(2.0, 0.0, 0.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        // point rotates around (1,0,0)
        rotated
            .origin
            .assert_near(Pt3::new(1.0, 1.0, 0.0), T::TEST_EPS);

        // direction still rotates normally
        rotated.normal.assert_near(Vec3::UNIT_Y, T::TEST_EPS);
    });

    scalar_test!(test_plane_rotate_about_self, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Plane::<T>::new(Pt3::new(1.0, 0.0, 0.0), Vec3::UNIT_Z);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        rotated.normal.assert_near(Vec3::UNIT_Z, T::TEST_EPS);
    });
    scalar_test!(test_plane_rotate_direction_normalized, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Y);

        let a = Plane::<T>::new(
            Pt3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
        );

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        assert!((rotated.normal.length() - T::ONE).abs() < T::TEST_EPS);
    });

    scalar_test!(test_plane_rotate_round_trip, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Y);

        let a = Plane::<T>::new(Pt3::new(1.0, 2.0, 3.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);
        let back = rotated.rotate_about_axis(axis, -T::PI_OVER_2);

        back.origin.assert_near(a.origin, T::TEST_EPS);
        back.normal.assert_near(a.normal, T::TEST_EPS);
    });

    scalar_test!(test_print, |T| {
        let a = Plane::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}
