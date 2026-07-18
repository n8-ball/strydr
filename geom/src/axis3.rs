use crate::{pt3::Pt3, scalar::Scalar, vec3::Vec3};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

//      ^
//      |
//      |
//      |
//      |
//      *

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axis3<T: Scalar> {
    origin: Pt3<T>,
    direction: Vec3<T>,
}

impl<T: Scalar> Axis3<T> {
    pub fn new(origin: Pt3<T>, direction: Vec3<T>) -> Self {
        assert!(direction.is_unit(), "axis direction must be unit!");

        Self { origin, direction }
    }

    pub fn origin(self) -> Pt3<T> {
        self.origin
    }

    pub fn direction(self) -> Vec3<T> {
        self.direction
    }

    pub fn translate(self, v: Vec3<T>) -> Self {
        Self {
            origin: Pt3::new(
                self.origin.x + v.x,
                self.origin.y + v.y,
                self.origin.z + v.z,
            ),
            direction: self.direction,
        }
    }

    pub fn rotate_about_axis(self, axis: Self, angle_radians: T) -> Self {
        Axis3::new(
            self.origin.rotate_about_axis(axis, angle_radians),
            self.direction
                .rotate_axis_angle(axis.direction, angle_radians),
        )
    }

    pub fn assert_near(self, other: Self, eps: T) {
        self.origin.assert_near(other.origin, eps);
        self.direction.assert_near(other.direction, eps);
    }
}

impl<T: Scalar> Display for Axis3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "position: {}\n direction: {}",
            self.origin, self.direction
        )
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
        }
    );

    scalar_test!(test_getters, |T| {
        let pos = Pt3::<T>::ZERO;
        let dir = Vec3::<T>::UNIT_X;
        let axis = Axis3::new(pos, dir);

        assert_eq!(axis.origin, pos);
        assert_eq!(axis.direction, dir);
    });

    scalar_test!(test_translate, |T| {
        let axis = Axis3::<T>::new(Pt3::<T>::new(-10.0, -10.0, -10.0), Vec3::<T>::UNIT_X);

        let translated = axis.translate(Vec3::<T>::new(10.0, 10.0, 10.0));

        translated.assert_near(
            Axis3::<T>::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X),
            T::TEST_EPS,
        );
    });

    scalar_test!(test_axis_translate_inverse, |T| {
        let a = Axis3::<T>::new(Pt3::new(-1.0, 4.0, 2.0), Vec3::UNIT_X);

        let v = Vec3::new(3.0, -5.0, 1.0);

        let moved = a.translate(v);
        let back = moved.translate(-v);

        back.origin.assert_near(a.origin, T::TEST_EPS);
        back.direction.assert_near(a.direction, T::TEST_EPS);
    });

    scalar_test!(test_axis_rotate_identity, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Axis3::<T>::new(Pt3::new(1.0, 2.0, 3.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::ZERO);

        rotated.origin.assert_near(a.origin, T::TEST_EPS);
        rotated.direction.assert_near(a.direction, T::TEST_EPS);
    });
    scalar_test!(test_axis_rotate_90deg_z, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Axis3::<T>::new(Pt3::new(1.0, 0.0, 0.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        rotated
            .origin
            .assert_near(Pt3::new(0.0, 1.0, 0.0), T::TEST_EPS);

        rotated.direction.assert_near(Vec3::UNIT_Y, T::TEST_EPS);
    });

    scalar_test!(test_axis_rotate_offset_axis, |T| {
        let axis = Axis3::<T>::new(
            Pt3::new(1.0, 0.0, 0.0), // shifted axis
            Vec3::UNIT_Z,
        );

        let a = Axis3::<T>::new(Pt3::new(2.0, 0.0, 0.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        // point rotates around (1,0,0)
        rotated
            .origin
            .assert_near(Pt3::new(1.0, 1.0, 0.0), T::TEST_EPS);

        // direction still rotates normally
        rotated.direction.assert_near(Vec3::UNIT_Y, T::TEST_EPS);
    });

    scalar_test!(test_axis_rotate_about_self, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Z);

        let a = Axis3::<T>::new(Pt3::new(1.0, 0.0, 0.0), Vec3::UNIT_Z);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        rotated.direction.assert_near(Vec3::UNIT_Z, T::TEST_EPS);
    });
    scalar_test!(test_axis_rotate_direction_normalized, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Y);

        let a = Axis3::<T>::new(
            Pt3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
        );

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);

        assert!((rotated.direction.length() - T::ONE).abs() < T::TEST_EPS);
    });

    scalar_test!(test_axis_rotate_round_trip, |T| {
        let axis = Axis3::<T>::new(Pt3::new(0.0, 0.0, 0.0), Vec3::UNIT_Y);

        let a = Axis3::<T>::new(Pt3::new(1.0, 2.0, 3.0), Vec3::UNIT_X);

        let rotated = a.rotate_about_axis(axis, T::PI_OVER_2);
        let back = rotated.rotate_about_axis(axis, -T::PI_OVER_2);

        back.origin.assert_near(a.origin, T::TEST_EPS);
        back.direction.assert_near(a.direction, T::TEST_EPS);
    });

    scalar_test!(test_print, |T| {
        let a = Axis3::new(Pt3::<T>::ZERO, Vec3::<T>::UNIT_X);
        println!("{a}");
    });
}
