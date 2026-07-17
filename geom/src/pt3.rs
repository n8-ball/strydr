use std::{fmt::{Formatter, Display}, fmt, ops::{Add, Sub}};
use crate::{vec3::Vec3, frame3::Frame3, scalar::Scalar, axis3::Axis3};

/// A 3d point represented by f32 fields, x, y, and z.
/// 
/// It is possible to just use a Vec3 for a point and skip the concept of Pt3 entirely. 
/// The issue I've experienced with that approach is it makes complex transforms hard to understand when reviewing later, 
/// and let's you call functions like dot, cross, and normalize on Vec3s that represent points, resulting in bugs that compile
/// but are difficult to track down later. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt3<T: Scalar>{
    pub x: T, 
    pub y: T, 
    pub z: T,
}

impl<T: Scalar> Pt3<T> {

    pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO };
    pub const MAX: Self = Self {x: T::MAX, y: T::MAX, z: T::MAX, };
    pub const MIN: Self = Self {x: T::MIN, y: T::MIN, z: T::MIN, };

    pub const fn new(x: T , y: T , z: T) -> Self {
        Self { x, y, z, }
    } 

    pub fn translate(self, v: Vec3<T>) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn displacement_to(self, rhs: Self) -> Vec3<T> {
        Vec3 {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z,
        }
    }

    pub fn distance_to(self, other: Self) -> T {
        (self - other).length()
    }

    pub fn distance_sq_to(self, other: Self) -> T {
        (self - other).length_squared()
    }

    pub fn midpoint_to(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) * T::HALF,
            y: (self.y + other.y) * T::HALF,
            z: (self.z + other.z) * T::HALF,
        }
    }

    pub fn lerp_to(self, b: Self, t: T) -> Self {
        assert!(t >= T::ZERO,
            "t is less than zero!");

        assert!(t <= T::ONE,
            "t is greater than one!");

        Self {
            x: self.x + (b.x - self.x) * t,
            y: self.y + (b.y - self.y) * t,
            z: self.z + (b.z - self.z) * t,
        }
    }

    pub fn transform_between_frames(self, source: Frame3<T>, destination: Frame3<T>) -> Self {
        let world = source.origin() 
        + source.lx() * self.x
        + source.ly() * self.y 
        + source.lz() * self.z;
        
        let delta = world - destination.origin();

        Self {
            x: delta.dot(destination.lx()),
            y: delta.dot(destination.ly()),
            z: delta.dot(destination.lz()),
        } 
    }

    pub fn rotate_about_axis(self, axis: Axis3<T>, angle_radians: T) -> Self {
        let from_axis = self - axis.origin();
        if from_axis.is_near_zero() {
            return self;
        }

        let rotated_dir = from_axis.rotate_axis_angle(axis.direction(), angle_radians);

        axis.origin() + rotated_dir
    }

    pub fn assert_near(self, b: Pt3<T>, eps: T) {
        assert!((self.x - b.x).abs() < eps, 
            "left x: {} != right x: {}", self.x, b.x);
        assert!((self.y - b.y).abs() < eps,
            "left y: {} != right y: {}", self.y, b.y);
        assert!((self.z - b.z).abs() < eps,
            "left z: {} != right z: {}", self.z, b.z);
    }
}

impl<T: Scalar> Add<Vec3<T>> for Pt3<T> {
    type Output = Self;

    fn add(self, v: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<T: Scalar> Add<Pt3<T>> for Vec3<T> {
    type Output = Pt3<T>;

    fn add(self, p: Pt3<T>) -> Self::Output {
        Pt3 {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }
}

impl<T: Scalar> Sub<Pt3<T>> for Pt3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Pt3<T>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Scalar> Sub<Vec3<T>> for Pt3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Scalar> Display for Pt3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scalar::TestScalar, scalar_test};

    scalar_test!(test_new, |T| {

        let p = Pt3::<T>::new(1.0, 2.0, 3.0);

        assert_eq!(p.x, T::from_f64(1.0));
        assert_eq!(p.y, T::from_f64(2.0));
        assert_eq!(p.z, T::from_f64(3.0));
    });

    scalar_test!(test_zero, |T| {

        let p = Pt3::<T>::ZERO;

        assert_eq!(p.x, T::ZERO);
        assert_eq!(p.y, T::ZERO);
        assert_eq!(p.z, T::ZERO);
    });

    scalar_test!(test_max, |T| {

        let p = Pt3::<T>::MAX;

        assert_eq!(p.x, T::MAX);
        assert_eq!(p.y, T::MAX);
        assert_eq!(p.z, T::MAX);
    });

    scalar_test!(test_min, |T| {

        let p = Pt3::<T>::MIN;

        assert_eq!(p.x, T::MIN);
        assert_eq!(p.y, T::MIN);
        assert_eq!(p.z, T::MIN);
    });

    fn test_translate_add<T: TestScalar>(p: Pt3<T>, v: Vec3<T>) 
    {
        let trans_fn = p.translate(v);
        let trans_vec_rhs_op = p + v;
        let trans_vec_lhs_op = v + p;

        let expected = Pt3 {
            x: p.x + v.x,
            y: p.y + v.y,
            z: p.z + v.z,
        };

        assert_eq!(trans_fn, expected);
        assert_eq!(trans_vec_rhs_op, expected);
        assert_eq!(trans_vec_lhs_op, expected);
        assert_eq!(trans_fn, trans_vec_rhs_op);
        assert_eq!(trans_fn, trans_vec_lhs_op);
    }

    scalar_test!(test_translate_add_scalar, |T| {
        let p = Pt3::<T>::new(1.0, 2.0, 3.0);
        let v = Vec3::<T>::new(10.0, 10.0, 10.0);
        
        test_translate_add(p, v);

        // TODO: Add some more tests 
    });

    fn test_translate_sub<T: TestScalar>(p: Pt3<T>, v: Vec3<T>) 
    {
        let trans_fn = p.translate(-v);
        let trans_vec_rhs_op = p - v;

        let expected = Pt3 {
            x: p.x - v.x,
            y: p.y - v.y,
            z: p.z - v.z,
        };

        assert_eq!(trans_fn, expected);
        assert_eq!(trans_vec_rhs_op, expected);
        assert_eq!(trans_fn, trans_vec_rhs_op);
    }

    scalar_test!(test_translate_sub_scalar, |T| {
        let p = Pt3::<T>::new(1.0, 2.0, 3.0);
        let v = Vec3::<T>::new(10.0, 10.0, 10.0);

        test_translate_sub(p, v); 

        // TODO: add some more tests
    });
    
    fn test_displacement<T: TestScalar>(a: Pt3<T>, b: Pt3<T>) 
    {
        let disp_fn = a.displacement_to(b);
        let disp_op = b - a;

        let expected = Vec3 {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        };

        assert_eq!(disp_fn, expected);
        assert_eq!(disp_op, expected);
        assert_eq!(disp_fn, disp_op);
    }

    scalar_test!(test_displacement, |T| {
        let a = Pt3::<T>::new(1.0, 2.0, 3.0);
        let b = Pt3::<T>::new(4.0, 5.0, 6.0);

        test_displacement(a, b);

        // TODO: add some more tests
    });

    fn test_distance_sq_to<T: TestScalar>(a: Pt3<T>, b: Pt3<T>) 
    {
        let expected = 
            (b.x - a.x) * (b.x - a.x)
            + (b.y - a.y) * (b.y - a.y)
            + (b.z - a.z) * (b.z - a.z);

        assert!((a.distance_sq_to(b) - expected).abs() < T::TEST_EPS);
    }

    scalar_test!(test_distance_sq_to_scalar, |T| {
        let origin = Pt3::<T>::ZERO;
        let pos_x = Pt3::<T>::new(10.0, 0.0, 0.0); 
        test_distance_sq_to(origin, pos_x);

        // TODO: add some more tests
    });

    fn test_distance_to<T: TestScalar>(a: Pt3<T>, b: Pt3<T>) 
    {
        let expected = 
            (b.x - a.x) * (b.x - a.x)
            + (b.y - a.y) * (b.y - a.y)
            + (b.z - a.z) * (b.z - a.z);

        assert!((a.distance_to(b) - expected.sqrt()).abs() < T::TEST_EPS);
    }

    scalar_test!(test_distance_to_scalar, |T| {
        let origin = Pt3::<T>::ZERO;
        let pos_x = Pt3::<T>::new(10.0, 0.0, 0.0); 
        test_distance_to(origin, pos_x);

        // TODO: add some more tests
    });

    fn test_midpoint_to<T: TestScalar>(a: Pt3<T>, b: Pt3<T>) 
    {
        let expected = Pt3 {
            x: (b.x - a.x) * T::HALF,
            y: (b.y - a.y) * T::HALF,
            z: (b.z - a.z) * T::HALF,
        };

        a.midpoint_to(b).assert_near(expected, T::TEST_EPS);
    }

    scalar_test!(test_midpoint_basic, |T| {
        let origin = Pt3::<T>::ZERO;
        let pos_x = Pt3::<T>::new(10.0, 0.0, 0.0); 
        test_midpoint_to(origin, pos_x);
    });

    scalar_test!(test_midpoint_is_symmetric_and_halfway, |T| {
        let a = Pt3::<T>::new(3.0, -2.0, 8.0);
        let b = Pt3::<T>::new(11.0, 6.0, -4.0);
        let m = a.midpoint_to(b);

        m.assert_near(b.midpoint_to(a), T::TEST_EPS);
        assert!((a.distance_to(m) - m.distance_to(b)).abs() <= T::TEST_EPS);
    });

    scalar_test!(test_lerp_to_non_origin, |T| {
        let a = Pt3::<T>::new(10.0, 20.0, 30.0);
        let b = Pt3::<T>::new(20.0, 40.0, 50.0);

        a.lerp_to(b, T::ZERO).assert_near(a, T::TEST_EPS);
        a.lerp_to(b, T::ONE).assert_near(b, T::TEST_EPS);
        a.lerp_to(b, T::HALF)
        .assert_near(Pt3::<T>::new(15.0, 30.0, 40.0), T::TEST_EPS);
    });

    scalar_test!(
        #[should_panic(expected = "t is less than zero!")]
        test_lerp_to_negative_t, 
        |T| {

        Pt3::<T>::ZERO.lerp_to(Pt3::<T>::new(10.0, 0.0, 0.0), -1.0);
    });

    scalar_test!(
        #[should_panic(expected = "t is greater than one!")]
        test_lerp_to_t_greater_than_one, 
        |T| {
        Pt3::<T>::ZERO.lerp_to(Pt3::<T>::new(10.0, 0.0, 0.0), 2.0);
    });

    scalar_test!(test_transform_between_frames_basic, |T| {
        let source = Frame3::<T>::IDENTITY;
        let destination = Frame3::<T>::from_xy(
            Pt3::<T>::ZERO,
            -Vec3::<T>::UNIT_X,
            -Vec3::<T>::UNIT_Y,
        );

        let p = Pt3::<T>::new(1.0, 1.0, 1.0);
        let expected = Pt3::<T>::new(-1.0, -1.0, 1.0);

        p.transform_between_frames(source, destination)
            .assert_near(expected, T::TEST_EPS);
    });

    scalar_test!(test_transform_between_rotated_frames, |T| {
        let source = Frame3::<T>::IDENTITY;
        let destination = Frame3::<T>::from_xy(
            Pt3::<T>::new(10.0, 0.0, 0.0),
            Vec3::<T>::UNIT_Y,
            -Vec3::<T>::UNIT_X,
        );

        let p = Pt3::<T>::new(2.0, 3.0, 4.0);
        let expected = Pt3::<T>::new(3.0, 8.0, 4.0);

        p.transform_between_frames(source, destination)
            .assert_near(expected, T::TEST_EPS);
    });

    scalar_test!(test_frame_round_trip_world_local_world, |T| {
        let world = Frame3::IDENTITY;
        let frame = Frame3::from_xy(
            Pt3::<T>::new(10.0, -4.0, 2.0),
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(-1.0, 1.0, 0.0).normalize(),
        );

        let p = Pt3::new(12.0, 3.0, -8.0);

        let t1 = p.transform_between_frames(world, frame);
        let t2 = t1.transform_between_frames(frame, world);

        t2.assert_near(p, T::TEST_EPS);
    });

    scalar_test!(test_rotate_point_basic, |T| {
        let p = Pt3::<T>::new(1.0, 0.0, 0.0);

        let axis = Axis3::new(
            Pt3::<T>::ZERO,
            Vec3::<T>::UNIT_Z
        );

        let angle_radians = T::PI;

        let rotated = p.rotate_about_axis(axis, angle_radians);
        let expected = Pt3::new(-1.0, 0.0, 0.0); 

        rotated.assert_near(expected, T::TEST_ROTATION_EPS);
    });

    scalar_test!(test_rotate_point_xz, |T| {
        let p = Pt3::<T>::new(10.0, 0.0, 10.0);

        let axis = Axis3::new(
            Pt3::<T>::ZERO,
            Vec3::<T>::UNIT_Z
        );

        let angle_radians = T::PI;

        let rotated = p.rotate_about_axis(axis, angle_radians);
        let expected = Pt3::new(-10.0, 0.0, 10.0); 

        rotated.assert_near(expected, T::TEST_ROTATION_EPS);
    });

    scalar_test!(test_print, |T| {
        let p = Pt3::<T>::new(1.0, 2.0, 3.0);
        println!("{p}");
    });
}
