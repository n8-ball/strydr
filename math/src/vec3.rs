use std::ops::{Add, Sub, Mul, Div};


/// A 3d vector represented by f32 fields, x, y, and z.
/// 
/// Generics considered here but it's unlikley for the benefits to be worth the complication at this stage.
/// Additionally, if for example we wanted to use a vec3 to iterate through a grid using u32s or similar, 
/// that would be a different object entirley anyway (Coord)
/// It's unlikley that more precision will be necessary anyway. 
/// 
/// Why not use nalgebra/glam?
/// 1) I want control
/// 2) I want to learn
/// 3) It's not that much work
/// 4) There's good chance I test/implement tolerances and material conditions,
/// and building wrapper implementations would probably be annoying
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z, }
    }

    pub const fn zero() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0, }
    }

    pub const fn unit_x() -> Self {
        Self {x: 1.0, y: 0.0, z: 0.0, }
    }

    pub const fn unit_y() -> Self {
        Self {x: 0.0, y: 1.0, z: 0.0, }
    }

    pub const fn unit_z() -> Self {
        Self {x: 0.0, y: 0.0, z: 1.0, }
    }

    pub const fn max() -> Self {
        Self {x: f32::MAX, y: f32::MAX, z: f32::MAX, }
    }

    pub const fn min() -> Self {
        Self {x: f32::MIN, y: f32::MIN, z: f32::MIN, }
    }

    pub fn vec_add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn vec_sub(&self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    pub fn vec_mul(&self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn vec_div(&self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }

    pub fn scalar_add(&self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }

    pub fn scalar_sub(&self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }

    pub fn scalar_mul(&self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    pub fn scalar_div(&self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    pub fn abs(&self) -> Self {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + 
        self.y * other.y + 
        self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len_sq(&self) -> f32 {
        self.dot(*self)
         
    }

    pub fn len(self) -> f32 {
        self.len_sq().sqrt() 
    }

    pub fn normalize(self) -> Self {
        let len = self.len();
        if len == 0.0 {
            return self;
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3)  -> Vec3 {
    Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, s: f32)  -> Vec3 {
    Vec3 {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, v: Vec3)  -> Vec3 {
    Vec3 {
            x: v.x + self,
            y: v.y + self,
            z: v.z + self,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3)  -> Vec3 {
    Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32)  -> Vec3 {
    Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3)  -> Vec3 {
    Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32)  -> Vec3 {
    Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3)  -> Vec3 {
    Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3)  -> Vec3 {
    Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32)  -> Vec3 {
    Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// TODO: move into 'test' crate. 
// Note: these test are exhaustive, not necessarily to test that the first implementation is correct, 
// but more so for assurance that I didn't break anything with later work.
// Additionaly, when inevitably when debugging a real cem, I don't need to manually check the math.
#[cfg(test)]
mod test {
    use super::*;

    const A: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    const B: Vec3  = Vec3::new(4.0, 5.0, 6.0);

    #[test]
    fn get_elements() {
        assert_eq!(A.x, 1.0);
        assert_eq!(A.y, 2.0);
        assert_eq!(A.z, 3.0);
    }

    #[test]
    fn zero() {
        let v = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn x() {
        let v = Vec3::unit_x();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn y() {
        let v = Vec3::unit_y();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn z() {
        let v = Vec3::unit_z();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 1.0);
    }  

    #[test]
    fn max() {
        let v = Vec3::max();
        assert_eq!(v.x, f32::MAX);
        assert_eq!(v.y, f32::MAX);
        assert_eq!(v.z, f32::MAX);
    }  

    #[test]
    fn min() {
        let v = Vec3::min();
        assert_eq!(v.x, f32::MIN);
        assert_eq!(v.y, f32::MIN);
        assert_eq!(v.z, f32::MIN);
    }  


    #[test]
    fn vec_add_fn() {
        let sum = A.vec_add(B);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn vec_add_op() {
        let sum = A + B;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn vec_sub_fn() {
        let diff = A.vec_sub(B);

        assert_eq!(diff.x, -3.0);
        assert_eq!(diff.y, -3.0);
        assert_eq!(diff.z, -3.0);
    }

    #[test]
    fn vec_sub_op() {
        let diff = A - B;

        assert_eq!(diff.x, -3.0);
        assert_eq!(diff.y, -3.0);
        assert_eq!(diff.z, -3.0);
    }

    #[test]
    fn vec_mul_fn() {
        let prod = A.vec_mul(B);

        assert_eq!(prod.x, 4.0);
        assert_eq!(prod.y, 10.0);
        assert_eq!(prod.z, 18.0);
    }

    #[test]
    fn vec_mul_op() {
        let prod = A * B;

        assert_eq!(prod.x, 4.0);
        assert_eq!(prod.y, 10.0);
        assert_eq!(prod.z, 18.0);
    }

    #[test]
    fn vec_div_fn() {
        let quot= A.vec_div(B);

        assert_eq!(quot.x, 0.25);
        assert_eq!(quot.y, 0.40);
        assert_eq!(quot.z, 0.50);
    }
    #[test]
    fn vec_div_op() {
        let quot= A / B;

        assert_eq!(quot.x, 0.25);
        assert_eq!(quot.y, 0.40);
        assert_eq!(quot.z, 0.50);
    }

    #[test]
    fn scalar_add_fn() {
        let s = 10.0;
        let sum = A.scalar_add(s);

        assert_eq!(sum.x, 11.0);
        assert_eq!(sum.y, 12.0);
        assert_eq!(sum.z, 13.0);
    }

    #[test]
    fn scalar_rhs_add_op() {
        let s = 10.0;
        let sum = A + s;

        assert_eq!(sum.x, 11.0);
        assert_eq!(sum.y, 12.0);
        assert_eq!(sum.z, 13.0);
    }

    #[test]
    fn scalar_lhs_add_op() {
        let s = 10.0;
        let sum = s + A;

        assert_eq!(sum.x, 11.0);
        assert_eq!(sum.y, 12.0);
        assert_eq!(sum.z, 13.0);
    }

    #[test]
    fn scalar_sub_fn() {
        let s = 10.0;
        let diff = A.scalar_sub(s);

        assert_eq!(diff.x, -9.0);
        assert_eq!(diff.y, -8.0);
        assert_eq!(diff.z, -7.0);
    }

    #[test]
    fn scalar_rhs_sub_op() {
        let s = 10.0;
        let diff = A - s;

        assert_eq!(diff.x, -9.0);
        assert_eq!(diff.y, -8.0);
        assert_eq!(diff.z, -7.0);
    }

    #[test]
    fn scalar_mul_fn() {
        let s = 10.0;
        let prod = A.scalar_mul(s);

        assert_eq!(prod.x, 10.0);
        assert_eq!(prod.y, 20.0);
        assert_eq!(prod.z, 30.0);
    }

    #[test]
    fn scalar_rhs_mul_op() {
        let s = 10.0;
        let prod = A * s;

        assert_eq!(prod.x, 10.0);
        assert_eq!(prod.y, 20.0);
        assert_eq!(prod.z, 30.0);
    }

    #[test]
    fn scalar_lhs_mul_op() {
        let s = 10.0;
        let prod = s * A;

        assert_eq!(prod.x, 10.0);
        assert_eq!(prod.y, 20.0);
        assert_eq!(prod.z, 30.0);
    }

    #[test]
    fn scalar_div_fn() {
        let s = 10.0;
        let quot = A.scalar_div(s);

        assert_eq!(quot.x, 0.1);
        assert_eq!(quot.y, 0.2);
        assert_eq!(quot.z, 0.3);
    }

    #[test]
    fn scalar_rhs_div_op() {
        let s = 10.0;
        let quot = A / s;

        assert_eq!(quot.x, 0.1);
        assert_eq!(quot.y, 0.2);
        assert_eq!(quot.z, 0.3);
    }

    #[test]
    fn abs() {
        let v = Vec3::new(-1.0, 1.0, -12.5);
        let abs = v.abs();
        assert_eq!(abs.x, 1.0);
        assert_eq!(abs.y, 1.0);
        assert_eq!(abs.z, 12.5);
    }

    #[test]
    fn dot() {
        let dot = A.dot(B);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn dot_perpendicular() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let dot = a.dot(b);

        assert!(dot.abs() < 1e-6);
    }

    #[test]
    fn cross_known_case() {
        let a = Vec3::new(2.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 3.0, 0.0);

        let c = a.cross(&b);

        assert!((c.z - 6.0).abs() < 1e-6);
    }

    #[test]
    fn cross_is_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let c = a.cross(&b);

        let dot1 = c.dot(a);
        let dot2 = c.dot(b);

        assert!(dot1.abs() < 1e-6);
        assert!(dot2.abs() < 1e-6);
    }

    #[test]
    fn cross_self_is_zero() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let c = a.cross(&a);

        assert!((c.x).abs() < 1e-6);
        assert!((c.y).abs() < 1e-6);
        assert!((c.z).abs() < 1e-6);
    }

    #[test]
    fn cross_anticommutative() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let ab = a.cross(&b);
        let ba = b.cross(&a);

        assert!((ab.x + ba.x).abs() < 1e-6);
        assert!((ab.y + ba.y).abs() < 1e-6);
        assert!((ab.z + ba.z).abs() < 1e-6);
    }

    #[test]
    fn cross_basic_axes() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);

        let z = x.cross(&y);

        assert!((z.x - 0.0).abs() < 1e-6);
        assert!((z.y - 0.0).abs() < 1e-6);
        assert!((z.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn len_sq() {
        let v = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(v.len_sq(), 25.0);
    }

    #[test]
    fn len() {
        let v = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(v.len(), 5.0);
    }

    #[test]
    fn length_zero_vector() {
        let v = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(v.len_sq(), 0.0);
        assert_eq!(v.len(), 0.0);
    }

    #[test]
    fn normalize_basic() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalize();

        assert!((n.len() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn normalize_3_4_5() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalize();

        assert!((n.len() - 1.0).abs() < 1e-6);

        assert!((n.x - 0.6).abs() < 1e-6);
        assert!((n.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn normalize_zero() {
        let n = Vec3::new(0.0, 0.0, 0.0).normalize();
        assert_eq!(n, Vec3::zero())
    }

    #[test]
    fn normalize_idempotent() {
        let v = Vec3::new(2.0, 3.0, 6.0);
        let n1 = v.normalize();
        let n2 = n1.normalize();

        assert!((n1.x - n2.x).abs() < 1e-6);
        assert!((n1.y - n2.y).abs() < 1e-6);
        assert!((n1.z - n2.z).abs() < 1e-6);
    }
}
