use crate::{vec3::Vec3, pt3::Pt3};


/// Standard representation of all degrees of freedom in 3d space.
/// 
/// Unlike other apis, this is opinionated.
/// It will panic if the input axes are not normalized.
/// It will panic if input axes are not perpendicular.
/// The reason being: the amount of bugs stemming from incorrectly assuming a frame's axis are perpendicular 
/// and normalized far outweighs the benefits of letting me slap together random axes.
/// 
/// Some other apis also let the caller specify a position and only one input axes, and internally guess the other two.
/// This implmentation does not, as that object is called an axis, as opposed to a frame.
/// 
/// Reinforcing the same point, all fields are private to ensure the reliability of a frame. If I call a frame, 
/// I'm sure it's not possible that I did something stupid with one of its fields in earlier code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frm3 {
    pos: Pt3,
    lx: Vec3,
    ly: Vec3,
    lz: Vec3,
}

const HANDEDNESS_EPS: f32 = 1e-6;

impl Frm3 {
    pub fn pos(&self) -> Pt3 {
        self.pos
    }

    pub fn lx(&self) -> Vec3 {
        self.lx 
    }

    pub fn ly(&self) -> Vec3 {
        self.ly
    }

    pub fn lz(&self) -> Vec3 {
        self.lz
    }   

    pub fn from_xy(pos: Pt3, lx: Vec3, ly: Vec3) -> Self {
        let lz = lx.cross(ly).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub fn from_xz(pos: Pt3, lx: Vec3, lz: Vec3) -> Self {
        let ly = lz.cross(lx).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub fn from_yz(pos: Pt3, ly: Vec3, lz: Vec3) -> Self {
        let lx = ly.cross(lz).normalize();

        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }


    fn validate_orthonormal_right_handed(&self) {
        let lx = self.lx();
        let ly = self.ly();
        let lz = self.lz();

        if lx.is_near_zero() {
            panic!("length of local x axis is near zero!");
        }

        if ly.is_near_zero() {
            panic!("length of local y axis is near zero!");
        }

        if lz.is_near_zero() {
            panic!("length of local z axis is near zero!");
        }
         
        if !lx.is_unit() {
            panic!("x axis must be unit length!");
        }

        if !ly.is_unit() {
            panic!("y axis must be unit length!");
        }

        if !lz.is_unit() {
            panic!("z axis must be unit length!");
        }

        if !lx.is_perpendicular_to(ly) {
            panic!("the local x and y axes are not perpendicular!")
        }

        if !ly.is_perpendicular_to(lz) {
            panic!("the local y and z axes are not perpendicular!")
        }

        if !lz.is_perpendicular_to(lx) {
            panic!("the local z and x axes are not perpendicular!")
        }

        let handedness = lx.cross(ly).dot(lz);

        if handedness < 0.0 {
            panic!("frame is not right-handed!")
        }

        if (handedness - 1.0) > HANDEDNESS_EPS {
            panic!("frame is not orthonormal and consistently right-handed!")
        }
    }


    /// This method is not intended as the standard method for instantiating a frame. 
    /// 
    /// More so if bridging to another 3d related package, this can validate that the external frame conforms 
    /// to the gaurantees provided by this api.
    pub fn from_xyz(pos: Pt3, lx: Vec3, ly: Vec3, lz: Vec3) -> Self {
        let frm = Self { pos, lx, ly, lz };
        frm.validate_orthonormal_right_handed();
        frm
    }

    pub const fn identity() -> Self {
        Self {
            pos: Pt3::zero(),
            lx: Vec3::unit_x(),
            ly: Vec3::unit_y(),
            lz: Vec3::unit_z(),
        }
    }

    pub fn translate(&self, v: Vec3) -> Self {
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
}

mod tests {
    use super::*;

    const IDENTITY: Frm3 = Frm3::identity();
    const P0: Pt3 = Pt3::zero();
    const UNIT_X: Vec3 = Vec3::unit_x();
    const UNIT_Y: Vec3 = Vec3::unit_y();
    const UNIT_Z: Vec3 = Vec3::unit_z();

    #[test]
    fn from_xy_basic() {
        let f = Frm3::from_xy(P0, UNIT_X, UNIT_Y);

        assert_eq!(f.pos, P0);
        assert_eq!(f.lx, UNIT_X);
        assert_eq!(f.ly, UNIT_Y);
        assert_eq!(f.lz, UNIT_Z); 
    }

    #[test]
    fn from_xz_basic() {
        let f = Frm3::from_xz(P0, UNIT_X, UNIT_Z);

        assert_eq!(f.pos, P0);
        assert_eq!(f.lx, UNIT_X);
        assert_eq!(f.ly, UNIT_Y);
        assert_eq!(f.lz, UNIT_Z);
    }

    #[test]
    fn from_yz_basic() {
        let f = Frm3::from_yz(P0, UNIT_Y, UNIT_Z);

        assert_eq!(f.pos, P0);
        assert_eq!(f.lx, UNIT_X);
        assert_eq!(f.ly, UNIT_Y);
        assert_eq!(f.lz, UNIT_Z);
    }

    #[test]
    fn from_xyz_basic() {
        let f = Frm3::from_xyz(P0, UNIT_X, UNIT_Y, UNIT_Z);

        assert_eq!(f.pos, P0);
        assert_eq!(f.lx, UNIT_X);
        assert_eq!(f.ly, UNIT_Y);
        assert_eq!(f.lz, UNIT_Z);
    }

    #[test]
    fn getters() {
        assert_eq!(IDENTITY.pos(), IDENTITY.pos);
        assert_eq!(IDENTITY.lx(), IDENTITY.lx); 
        assert_eq!(IDENTITY.ly(), IDENTITY.ly); 
        assert_eq!(IDENTITY.lz(), IDENTITY.lz); 
    }


    #[test]
    #[should_panic(expected = "local x axis is near zero!")]
    fn validate_orthonormal_right_handed_zero_x() {
        let frm = Frm3 {
            pos: P0, 
            lx: Vec3::zero(), 
            ly: UNIT_Y,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "local y axis is near zero!")]
    fn validate_orthonormal_right_handed_zero_y() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: Vec3::zero(),
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "local z axis is near zero!")]
    fn validate_orthonormal_right_handed_zero_z() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: UNIT_Y,
            lz: Vec3::zero(),
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "x axis must be unit length!")]
    fn validate_orthonormal_right_handed_nonunit_x() {
        let frm = Frm3 {
            pos: P0, 
            lx: Vec3::new(10.0, 0.0, 0.0),
            ly: UNIT_Y,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "y axis must be unit length!")]
    fn validate_orthonormal_right_handed_nonunit_y() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: Vec3::new(10.0, 0.0, 0.0),
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }


    #[test]
    #[should_panic(expected = "z axis must be unit length!")]
    fn validate_orthonormal_right_handed_nonunit_z() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: UNIT_Y,
            lz: Vec3::new(10.0, 0.0, 0.0),
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "the local x and y axes are not perpendicular!")] 
    fn validate_orthonormal_right_handed_xy_not_perp() {
        let x = Vec3::new(1.0, 1.0, 0.0).normalize();
        let frm = Frm3 {
            pos: P0, 
            lx: x,
            ly: UNIT_Y,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "the local y and z axes are not perpendicular!")] 
    fn validate_orthonormal_right_handed_yz_not_perp() {
        let x = Vec3::new(1.0, 2.0, 3.0).normalize();
        let y = Vec3::new(4.0, -5.0, 2.0).normalize();
        let frm = Frm3 {
            pos: P0, 
            lx: x,
            ly: y,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "the local z and x axes are not perpendicular!")] 
    fn validate_orthonormal_right_handed_zx_not_perp() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: UNIT_Y,
            lz: UNIT_X,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "frame is not right-handed!")] 
    fn validate_orthonormal_right_handed_not_right_handed_neg_x() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X * -1.0,
            ly: UNIT_Y,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "frame is not right-handed!")] 
    fn validate_orthonormal_right_handed_not_right_handed_neg_y() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: UNIT_Y * -1.0,
            lz: UNIT_Z,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    #[should_panic(expected = "frame is not right-handed!")] 
    fn validate_orthonormal_right_handed_not_right_handed_neg_z() {
        let frm = Frm3 {
            pos: P0, 
            lx: UNIT_X,
            ly: UNIT_Y,
            lz: UNIT_Z * -1.0,
        };

        frm.validate_orthonormal_right_handed();
    }

    #[test]
    fn identity() {
        assert_eq!(IDENTITY.pos, Pt3 { x: 0.0, y: 0.0, z: 0.0});
        assert_eq!(IDENTITY.lx, Vec3{ x: 1.0, y: 0.0, z: 0.0});
        assert_eq!(IDENTITY.ly, Vec3{ x: 0.0, y: 1.0, z: 0.0});
        assert_eq!(IDENTITY.lz, Vec3{ x: 0.0, y: 0.0, z: 1.0});
    }
     
    #[test]
    fn translate() {
        let trans = IDENTITY.translate(Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(trans.pos.x, 1.0);
        assert_eq!(trans.pos.y, 2.0);
        assert_eq!(trans.pos.z, 3.0);
        assert_eq!(trans.lx, IDENTITY.lx);
        assert_eq!(trans.ly, IDENTITY.ly);
        assert_eq!(trans.lz, IDENTITY.lz);
    }
}
