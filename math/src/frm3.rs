use std::{fmt::{Formatter, Display}, fmt};
use crate::{vec3::Vec3, pt3::Pt3, scalar::Scalar, axis3::Axis3, quat::Quat};


/// Standard representation of all degrees of freedom in 3d space.
/// 
/// Unlike other apis, this is highly opinionated.
/// It will panic if the input axes are not normalized.
/// It will panic if input axes are not perpendicular.
/// It will panic if the provided input axes do not result in a right-handed frame.
/// 
/// The reason being: the amount of bugs stemming from incorrectly assuming a frame's axis are perpendicular 
/// and normalized far outweighs the benefits of letting me slap together random axes.
/// 
/// Some other apis also let the caller specify a position and only one input axes, and internally guess the other two.
/// This implmentation does not, as that object is called an axis, as opposed to a frame.
/// 
/// Reinforcing the same point, all fields are private to ensure the reliability of a frame. If I call a frame, 
/// I'm sure it's not possible that I did something stupid with one of its fields in earlier code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frm3<T: Scalar> {
    pos: Pt3<T>,
    lx: Vec3<T>,
    ly: Vec3<T>,
    lz: Vec3<T>,
}

const HANDEDNESS_EPS: f64 = 1e-6;

impl<T: Scalar> Frm3<T> {
    pub const IDENTITY: Self = Self { pos: Pt3::ZERO, lx: Vec3::UNIT_X, ly: Vec3::UNIT_Y, lz: Vec3::UNIT_Z, };

    /// Provides the position of the frame.
    pub fn pos(&self) -> Pt3<T> {
        self.pos
    }

    /// Provides the local x axis of the frame.
    pub fn lx(&self) -> Vec3<T> {
        self.lx 
    }

    /// Provides the local y axis of the frame.
    pub fn ly(&self) -> Vec3<T> {
        self.ly
    }

    /// Provides the local z axis of the frame.
    pub fn lz(&self) -> Vec3<T> {
        self.lz
    }   

    pub fn from_xy(pos: Pt3<T>, lx: Vec3<T>, ly: Vec3<T>) -> Self {
        let lz = lx.cross(ly).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub fn from_xz(pos: Pt3<T>, lx: Vec3<T>, lz: Vec3<T>) -> Self {
        let ly = lz.cross(lx).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub fn from_yz(pos: Pt3<T>, ly: Vec3<T>, lz: Vec3<T>) -> Self {
        let lx = ly.cross(lz).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    /// This method is not intended as the standard method for instantiating a frame. 
    /// 
    /// More so if bridging to another 3d related package, 
    /// this can validate that the external frame conforms to the gaurantees provided by this api.
    pub fn from_xyz(pos: Pt3<T>, lx: Vec3<T>, ly: Vec3<T>, lz: Vec3<T>) -> Self {
        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    /// All frame-building functions run their result through these checks before returning the frame to the caller.
    /// 
    /// Ensures the local axes are unit-length, and the frame is orthonormal and right handed.
    fn validate_orthonormal_right_handed(&self) {
        let lx = self.lx();
        let ly = self.ly();
        let lz = self.lz();

        assert!(lx.is_unit(), 
            "x axis must be unit-length, recieved {lx}. (all Frm3 axes must be unit-length Vec3s!)"
        );

        assert!(ly.is_unit(), 
            "y axis must be unit-length, recieved {ly}. (all Frm3 axes must be unit-length Vec3s!)"
        );

        assert!(lz.is_unit(), 
            "z axis must be unit-length, recieved {lz}. (all Frm3 axes must be unit-length Vec3s!)"
        );

        assert!(lx.is_perpendicular_to(ly),
            "the local x and y axes are not perpendicular! Recieved local x: {lx}, local y: {ly}"
        ); 

        assert!(ly.is_perpendicular_to(lz),
            "the local y and z axes are not perpendicular! Recieved local y: {ly}, local z: {lz}"
        ); 

        assert!(lz.is_perpendicular_to(lx),
            "the local z and x axes are not perpendicular! Recieved local z: {lz}, local x: {lx}"
        ); 

        let handedness = lx.cross(ly).dot(lz);

        assert!(!(handedness < T::ZERO),
            "frame is not right-handed!"
        ); 

        assert!(!((handedness - T::ONE) > T::from_f64(HANDEDNESS_EPS)),
            "frame is not orthonormal and consistently right-handed!"
        ); 
    }


    pub const fn identity() -> Self {
        Self {
            pos: Pt3::ZERO,
            lx: Vec3::UNIT_X,
            ly: Vec3::UNIT_Y,
            lz: Vec3::UNIT_Z,
        }
    }

    pub fn translate(&self, v: Vec3<T>) -> Self {
        Self {
           pos: Pt3::new(
            self.pos.x + v.x,
            self.pos.y + v.y,
            self.pos.z + v.z,
           ),
           lx: self.lx,
           ly: self.ly,
           lz: self.lz,
        }
    }

    pub fn rotate_about_axis(self, axis: Axis3<T>, angle_radians: T) -> Self {
        let quat = Quat::from_axis_angle(axis.dir, angle_radians);

        let frm = Frm3::from_xyz(
            self.pos.rotate_about_axis(axis, angle_radians), 
            self.lx.rotate(quat), 
            self.ly.rotate(quat), 
            self.lz.rotate(quat));
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub fn assert_near(self, other: Self, eps: T) {
            self.pos.assert_near(other.pos, eps);
            self.lx.assert_near(other.lx, eps);
            self.ly.assert_near(other.ly, eps);
            self.lz.assert_near(other.lz, eps);
    }

    
}

impl<T: Scalar> Display for Frm3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "position: {}\nlocal x: {}\nlocal y: {}\nlocal z: {}", self.pos, self.lx, self.ly, self.lz)
    }
}

mod tests {
    use super::*;

    #[allow(unused_imports)] // This is needed to prevent an unused import warning on use super::*; for some reason.
    use crate::{scalar_test, scalar::TestScalar};


    scalar_test!(test_from_xy_basic, |T| {
        let pos = Pt3::<T>::ZERO;
        let lx = Vec3::<T>::UNIT_X;
        let ly = Vec3::<T>::UNIT_Y;

        let frm = Frm3::<T>::from_xy(pos, lx, ly);

        assert_eq!(frm.pos, pos);
        assert_eq!(frm.lx, Vec3::<T>::UNIT_X);
        assert_eq!(frm.ly, Vec3::<T>::UNIT_Y);
        assert_eq!(frm.lz, Vec3::<T>::UNIT_Z);
    });

    scalar_test!(test_from_xz_basic, |T| {
        let pos = Pt3::<T>::ZERO;
        let lx = Vec3::<T>::UNIT_X;
        let ly = Vec3::<T>::UNIT_Y;

        let frm = Frm3::<T>::from_xy(pos, lx, ly);

        assert_eq!(frm.pos, pos);
        assert_eq!(frm.lx, Vec3::<T>::UNIT_X);
        assert_eq!(frm.ly, Vec3::<T>::UNIT_Y);
        assert_eq!(frm.lz, Vec3::<T>::UNIT_Z);
    });

    scalar_test!(test_from_yz_basic, |T| {
        let pos = Pt3::<T>::ZERO;
        let ly = Vec3::<T>::UNIT_Y;
        let lz = Vec3::<T>::UNIT_Z;

        let frm = Frm3::<T>::from_yz(pos, ly, lz);

        assert_eq!(frm.pos, pos);
        assert_eq!(frm.lx, Vec3::<T>::UNIT_X);
        assert_eq!(frm.ly, Vec3::<T>::UNIT_Y);
        assert_eq!(frm.lz, Vec3::<T>::UNIT_Z);
    });
    scalar_test!(test_from_xyz_basic, |T| {
        let pos = Pt3::<T>::ZERO;
        let lx = Vec3::<T>::UNIT_X;
        let ly = Vec3::<T>::UNIT_Y;
        let lz = Vec3::<T>::UNIT_Z;

        let frm = Frm3::<T>::from_xyz(pos, lx, ly, lz);

        assert_eq!(frm.pos, pos);
        assert_eq!(frm.lx, Vec3::<T>::UNIT_X);
        assert_eq!(frm.ly, Vec3::<T>::UNIT_Y);
        assert_eq!(frm.lz, Vec3::<T>::UNIT_Z);
    });

    scalar_test!(test_getters, |T| {
        let pos = Pt3::<T>::ZERO;
        let lx = Vec3::<T>::UNIT_X;
        let ly = Vec3::<T>::UNIT_Y;
        let lz = Vec3::<T>::UNIT_Z;

        let frm = Frm3::<T>::from_xyz(pos, lx, ly, lz);

        assert_eq!(frm.pos, pos);
        assert_eq!(frm.lx(), Vec3::<T>::UNIT_X);
        assert_eq!(frm.ly(), Vec3::<T>::UNIT_Y);
        assert_eq!(frm.lz(), Vec3::<T>::UNIT_Z);
    });


    scalar_test!(
        #[should_panic(expected = "x axis must be unit-length")]
        test_validate_orthonormal_right_handed_zero_lx,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::ZERO,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "y axis must be unit-length")]
        test_validate_orthonormal_right_handed_zero_ly,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO, 
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::ZERO,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "z axis must be unit-length")]
        test_validate_orthonormal_right_handed_zero_lz,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::ZERO,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "x axis must be unit-length")]
        test_validate_orthonormal_right_handed_nonunit_lx,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::new(10.0, 0.0, 0.0),
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "y axis must be unit-length")]
        test_validate_orthonormal_right_handed_nonunit_ly,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::new(0.0, 10.0, 0.0),
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "z axis must be unit-length")]
        test_validate_orthonormal_right_handed_nonunit_lz,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::new(0.0, 0.0, 10.0),
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "the local x and y axes are not perpendicular!")] 
        test_validate_orthonormal_right_handed_xy_not_perpendicular,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO, 
            lx: Vec3::<T>::new(1.0, 1.0, 0.0).normalize(),
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "the local y and z axes are not perpendicular!")] 
        test_validate_orthonormal_right_handed_yz_not_perpendicular,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::new(1.0, 2.0, 3.0).normalize(),
            ly: Vec3::<T>::new(4.0, -5.0, 2.0).normalize(),
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "the local z and x axes are not perpendicular!")] 
        test_validate_orthonormal_right_handed_zx_not_perpendicular,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_X,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "frame is not right-handed!")] 
        test_validate_orthonormal_right_handed_not_right_handed_neg_x,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X * T::NEG_ONE,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "frame is not right-handed!")] 
        test_validate_orthonormal_right_handed_not_right_handed_neg_y,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y * T::NEG_ONE,
            lz: Vec3::<T>::UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(
        #[should_panic(expected = "frame is not right-handed!")] 
        test_validate_orthonormal_right_handed_not_right_handed_neg_z,
        |T| {
        let frm = Frm3 {
            pos: Pt3::<T>::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z * T::NEG_ONE,
        };

        frm.validate_orthonormal_right_handed();
    });

    scalar_test!(test_identity, |T| {
        let identity = Frm3::<T>::identity();
        let expected = Frm3 {
            pos: Pt3::ZERO,
            lx: Vec3::<T>::UNIT_X,
            ly: Vec3::<T>::UNIT_Y,
            lz: Vec3::<T>::UNIT_Z,
        };

        assert_eq!(identity.pos(), expected.pos());
        assert_eq!(identity.lx(), expected.lx());
        assert_eq!(identity.ly(), expected.ly());
        assert_eq!(identity.lz(), expected.lz());
    });

    fn test_translate<T: Scalar>(frm: Frm3<T>, v: Vec3<T>) {
        let translated = frm.translate(v);
        let expected = Frm3 {
            pos: Pt3 {
                x: frm.pos().x + v.x,
                y: frm.pos().y + v.y,
                z: frm.pos().z + v.z,
            },
            lx: frm.lx(),
            ly: frm.ly(),
            lz: frm.lz(),
        };
        assert_eq!(translated.pos(), expected.pos());
        assert_eq!(translated.lx(), expected.lx());
        assert_eq!(translated.ly(), expected.ly());
        assert_eq!(translated.lz(), expected.lz());
    }

    scalar_test!(test_translate_scalar, |T| {
        let frm = Frm3::<T>::identity();          
        let v = Vec3::<T>::new(10.0, -10.0, 0.0);
        test_translate(frm, v);

        // TODO: add some more tests
    });

    scalar_test!(test_rotate_frm_basic, |T| {
        let frm = Frm3::<T>::identity();          
        let axis_dir = Vec3::<T>::UNIT_Z;
        let axis = Axis3::<T>::new(
            frm.pos, axis_dir
        );
        let angle_radians = T::PI;

        let rotated = frm.rotate_about_axis(axis, angle_radians);
        
        let expected = Frm3::from_xy(
            frm.pos, 
            frm.lx * T::NEG_ONE, 
            frm.ly * T::NEG_ONE, 
        );

        rotated.assert_near(expected, T::TEST_EPS);

    });

    scalar_test!(test_print, |T| {
        let f = Frm3::<T>::IDENTITY;
        println!("{f}");
    });
}
