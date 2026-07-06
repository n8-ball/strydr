use std::ops::{Add, Sub};
use crate::vec3::Vec3;

/// A 3d point represented by f32 fields, x, y, and z.
/// 
/// It is possible to just use a Vec3 for a point and skip the concept of Pt3 entirely. 
/// The issue I've experienced with that approach is it makes complex transforms hard to understand when reviewing later, 
/// and let's you call functions like dot, cross, and normalize on Vec3s that represent points, resulting in bugs that compile
/// but are difficult to track down later. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt3 {
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
}

impl Pt3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, }
    } 

    const fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, }
    } 

    pub const fn max() -> Self {
        Self {x: f32::MAX, y: f32::MAX, z: f32::MAX, }
    }

    pub const fn min() -> Self {
        Self {x: f32::MIN, y: f32::MIN, z: f32::MIN, }
    }

    pub fn translate(self, v: Vec3) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn distance_to(self, other: Self) -> f32 {
        (self - other).len()
    }

    pub fn distance_sq_to(self, other: Self) -> f32 {
        (self - other).len_sq()
    }

    pub fn midpoint(self, other: Self) -> Pt3 {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }

    pub fn lerp(self, b: Self, v: f32) -> Pt3 {
        Self {
            x: (b.x - self.x) * v,
            y: (b.y - self.y) * v,
            z: (b.z - self.z) * v,
        }
    }
}

impl Add<Vec3> for Pt3 {
    type Output = Pt3;

    fn add(self, v: Vec3) -> Pt3 {
        Pt3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Add<Pt3> for Vec3{
    type Output = Pt3;

    fn add(self, p: Pt3) -> Pt3 {
        Pt3 {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }
}

impl Sub<Pt3> for Pt3 {
    type Output = Vec3;

    fn sub(self, rhs: Pt3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for Pt3 {
    type Output = Pt3;

    fn sub(self, rhs: Vec3) -> Pt3 {
        Pt3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const A: Pt3 = Pt3::new(1.0, 2.0, 3.0);
    const B: Pt3 = Pt3::new(4.0, 5.0, 6.0);

    const O: Pt3 = Pt3::origin();
    const POS_X: Pt3 = Pt3::new(10.0, 0.0, 0.0);
    const POS_Y: Pt3 = Pt3::new(0.0, 10.0, 0.0);
    const POS_Z: Pt3 = Pt3::new(0.0, 0.0, 10.0);

    const NEG_X: Pt3 = Pt3::new(-10.0, 0.0, 0.0);
    const NEG_Y: Pt3 = Pt3::new(0.0, -10.0, 0.0);
    const NEG_Z: Pt3 = Pt3::new(0.0, 0.0, -10.0);

    const POS_COMP: Pt3 = Pt3::new(10.0, 10.0, 10.0);
    const NEG_COMP: Pt3 = Pt3::new(-10.0, -10.0, -10.0);

    #[test]
    fn get_elements() {
        assert_eq!(A.x, 1.0);
        assert_eq!(A.y, 2.0);
        assert_eq!(A.z, 3.0);
    }

    #[test]
    fn origin() {
        let p = Pt3::origin();

        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn max() {
        let p = Pt3::max();
        assert_eq!(p.x, f32::MAX);
        assert_eq!(p.y, f32::MAX);
        assert_eq!(p.z, f32::MAX);
    }  

    #[test]
    fn min() {
        let p = Pt3::min();
        assert_eq!(p.x, f32::MIN);
        assert_eq!(p.y, f32::MIN);
        assert_eq!(p.z, f32::MIN);
    }  

    #[test]
    fn translate() {
        let v = Vec3::new(10.0, 10.0, 10.0);
        let trans = A.translate(v);
        assert_eq!(trans.x, 11.0);
        assert_eq!(trans.y, 12.0);
        assert_eq!(trans.z, 13.0);       
    }  

    #[test]
    fn vec_add_rhs_op() {
        let v = Vec3::new(10.0, 10.0, 10.0);
        let trans = A + v;
        assert_eq!(trans.x, 11.0);
        assert_eq!(trans.y, 12.0);
        assert_eq!(trans.z, 13.0);       
    }  

    #[test]
    fn vec_add_lhs_op() {
        let v = Vec3::new(10.0, 10.0, 10.0);
        let trans = v + A;
        assert_eq!(trans.x, 11.0);
        assert_eq!(trans.y, 12.0);
        assert_eq!(trans.z, 13.0);       
    }  

    #[test]
    fn vec_sub_op() {
        let v = Vec3::new(10.0, 10.0, 10.0);
        let trans = A - v;
        assert_eq!(trans.x, -9.0);
        assert_eq!(trans.y, -8.0);
        assert_eq!(trans.z, -7.0);       
    }

    #[test]
    fn displacement() {
        let disp = A - B;
        assert_eq!(disp.x, -3.0);
        assert_eq!(disp.y, -3.0);
        assert_eq!(disp.z, -3.0);       
    }

    #[test]
    fn distance_to() {
        assert!(O.distance_to(POS_X) - 10.0 < 1e-6);
        assert!(O.distance_to(POS_Y) - 10.0 < 1e-6);
        assert!(O.distance_to(POS_Z) - 10.0 < 1e-6);

        assert!(O.distance_to(NEG_X) - 10.0 < 1e-6);
        assert!(O.distance_to(NEG_Y) - 10.0 < 1e-6);
        assert!(O.distance_to(NEG_Z) - 10.0 < 1e-6);

        assert!((O.distance_to(POS_COMP) - 17.320509) < 1e-6);
        assert!((O.distance_to(NEG_COMP) - 17.320509) < 1e-6);
    }

    #[test]
    fn distance_sq_to() {
        assert!(O.distance_sq_to(POS_X) - 100.0 < 1e-6);
        assert!(O.distance_sq_to(POS_Y) - 100.0 < 1e-6);
        assert!(O.distance_sq_to(POS_Z) - 100.0 < 1e-6);

        assert!(O.distance_sq_to(NEG_X) - 100.0 < 1e-6);
        assert!(O.distance_sq_to(NEG_Y) - 100.0 < 1e-6);
        assert!(O.distance_sq_to(NEG_Z) - 100.0 < 1e-6);

        assert!((O.distance_sq_to(POS_COMP) - 300.0) < 1e-6);
        assert!((O.distance_sq_to(NEG_COMP) - 300.0) < 1e-6);
    }

    #[test]
    fn midpoint() {
        assert_eq!(O.midpoint(POS_X), Pt3::new(5.0, 0.0, 0.0));
        assert_eq!(O.midpoint(POS_Y), Pt3::new(0.0, 5.0, 0.0));
        assert_eq!(O.midpoint(POS_Z), Pt3::new(0.0, 0.0, 5.0));

        assert_eq!(O.midpoint(NEG_X), Pt3::new(-5.0, 0.0, 0.0));
        assert_eq!(O.midpoint(NEG_Y), Pt3::new(0.0, -5.0, 0.0));
        assert_eq!(O.midpoint(NEG_Z), Pt3::new(0.0, 0.0, -5.0));

        assert_eq!(O.midpoint(POS_COMP), Pt3::new(5.0, 5.0, 5.0));
        assert_eq!(O.midpoint(NEG_COMP), Pt3::new(-5.0, -5.0, -5.0));
    }

    #[test]
    fn lerp() {
        let v = 0.1;
        assert_eq!(O.lerp(POS_X, v), Pt3::new(1.0, 0.0, 0.0));
        assert_eq!(O.lerp(POS_Y, v), Pt3::new(0.0, 1.0, 0.0));
        assert_eq!(O.lerp(POS_Z, v), Pt3::new(0.0, 0.0, 1.0));

        assert_eq!(O.lerp(NEG_X, v), Pt3::new(-1.0, 0.0, 0.0));
        assert_eq!(O.lerp(NEG_Y, v), Pt3::new(0.0, -1.0, 0.0));
        assert_eq!(O.lerp(NEG_Z, v), Pt3::new(0.0, 0.0, -1.0));

        assert_eq!(O.lerp(POS_COMP, v), Pt3::new(1.0, 1.0, 1.0));
        assert_eq!(O.lerp(NEG_COMP, v), Pt3::new(-1.0, -1.0, -1.0));
    }
}
