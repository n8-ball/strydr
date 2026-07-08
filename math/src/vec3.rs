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

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
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

    pub fn is_unit(&self) -> bool {
        const UNIT_LENGTH_EPS: f32 = 1e-6;
        let len_sq = self.len_sq();
        (len_sq - 1.0).abs() < UNIT_LENGTH_EPS
    }

    pub fn is_exactly_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn is_near_zero(&self) -> bool {
        const LENGTH_NEAR_ZERO_EPS: f32 = 1e-8;
        self.len_sq() < LENGTH_NEAR_ZERO_EPS * LENGTH_NEAR_ZERO_EPS
    }

    pub fn approx_angle_to(&self, other: Vec3) -> f32 {
        // Too small to define an angle.
        // Consider returning a Result instead of panicking.
        if self.is_near_zero() {
            panic!("vector 'self' is near-zero, cannot compute angle!")
        }

        if other.is_near_zero() {
            panic!("vector 'other' is near-zero, cannot compute angle!")
        }

        let len = self.len() * other.len();
        let mut cos = self.dot(other) / len;
        cos = cos.clamp(-1.0, 1.0);
        cos.acos()
    }

    pub fn angle_to(&self, other: Vec3) -> f64 {
         // Too small to define an angle.
        // Consider returning a Result instead of panicking.
        if self.is_near_zero() {
            panic!("vector 'self' is near-zero, cannot compute angle!")
        }

        if other.is_near_zero() {
            panic!("vector 'other' is near-zero, cannot compute angle!")
        }

        let ax_d = self.x as f64;
        let ay_d = self.y as f64;
        let az_d = self.z as f64;

        let bx_d = other.x as f64;
        let by_d = other.y as f64;
        let bz_d = other.z as f64;

        let len_sq_a = ax_d * ax_d + ay_d * ay_d + az_d * az_d;
        let len_sq_b = bx_d * bx_d + by_d * by_d + bz_d * bz_d;

        let dot = 
            ax_d * bx_d + 
            ay_d * by_d +
            az_d * bz_d;

        let mut cos = dot / (len_sq_a * len_sq_b).sqrt();
        cos = cos.clamp(-1.0, 1.0);

        cos.acos() 
    }

    pub fn is_perpendicular_to(&self, other: Vec3) -> bool {
        const ORTHOGONAL_EPS: f32 = 1e-6; 

        // Too small to define an angle.
        if self.is_near_zero() || other.is_near_zero() {
            return false;
        }

        let len = self.len() * other.len();
        let cos = self.dot(other) / len;
        cos.abs() < ORTHOGONAL_EPS
    }

    pub fn is_parallel_to(&self, other: Vec3) -> bool {
        const ANGLE_TOLERANCE_RADIANS: f32 = 1e-6; 

        // Too small to define an angle.
        if self.is_near_zero() || other.is_near_zero() {
            return false;
        }

        let len_prod = (self.len() * other.len()).sqrt();
        let cross_len = self.cross(other).len();

        let sin_angle = cross_len / len_prod;
        let tolerance = ANGLE_TOLERANCE_RADIANS.sin();
        return sin_angle <= tolerance
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
mod tests {
    use super::*;

    const A: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    const B: Vec3  = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

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
    fn unit_x() {
        let v = Vec3::unit_x();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn unit_y() {
        let v = Vec3::unit_y();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn unit_z() {
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
        let x = Vec3::unit_x();
        let y = Vec3::unit_y();
        let dot = x.dot(y);

        assert!(dot.abs() < 1e-6);
    }

    #[test]
    fn cross_known_case() {
        let a = Vec3::new(2.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 3.0, 0.0);

        let c = a.cross(b);

        assert!((c.z - 6.0).abs() < 1e-6);
    }

    #[test]
    fn cross_is_perpendicular() {
        let c = A.cross(B);

        let dot1 = c.dot(A);
        let dot2 = c.dot(B);

        assert!(dot1.abs() < 1e-6);
        assert!(dot2.abs() < 1e-6);
    }

    #[test]
    fn cross_self_is_zero() {
        let c = A.cross(A);

        assert!((c.x).abs() < 1e-6);
        assert!((c.y).abs() < 1e-6);
        assert!((c.z).abs() < 1e-6);
    }

    #[test]
    fn cross_anticommutative() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let ab = a.cross(b);
        let ba = b.cross(a);

        assert!((ab.x + ba.x).abs() < 1e-6);
        assert!((ab.y + ba.y).abs() < 1e-6);
        assert!((ab.z + ba.z).abs() < 1e-6);
    }

    #[test]
    fn cross_basic_axes() {
        let x = Vec3::unit_x();
        let y = Vec3::unit_y();

        let z = x.cross(y);

        assert!((z.x - 0.0).abs() < 1e-6);
        assert!((z.y - 0.0).abs() < 1e-6);
        assert!((z.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn len_sq() {
        let v_pos = Vec3::new(3.0, 4.0, 0.0);
        let v_neg = Vec3::new(-3.0, -4.0, 0.0);
        let v_both = Vec3::new(3.0, -4.0, 0.0);

        assert_eq!(v_pos.len_sq(), 25.0);
        assert_eq!(v_neg.len_sq(), 25.0);
        assert_eq!(v_both.len_sq(), 25.0);
    }

    #[test]
    fn len() {
        let v_pos = Vec3::new(3.0, 4.0, 0.0);
        let v_neg = Vec3::new(-3.0, -4.0, 0.0);
        let v_both = Vec3::new(3.0, -4.0, 0.0);

        assert_eq!(v_pos.len(), 5.0);
        assert_eq!(v_neg.len(), 5.0);
        assert_eq!(v_both.len(), 5.0);
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
    fn is_unit() {
        let x_pos = Vec3::unit_x();
        let x_neg = Vec3::unit_x() * -1.0;
        let comp = Vec3::new(10.0, -1.0, 200.0).normalize();

        assert!(x_pos.is_unit());
        assert!(x_neg.is_unit());
        assert!(comp.is_unit());
    }
    #[test]
    fn is_not_unit() {
        let comp = Vec3::new(10.0, -1.0, 200.0);
        let small = Vec3::new(1e-2, 1e-2, 1e-2);

        assert!(!comp.is_unit());
        assert!(!small.is_unit());
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

    #[test]
    fn is_exactly_zero() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert!(v.is_exactly_zero());
    }

    #[test]
    fn is_not_exactly_zero() {
        let v = Vec3::new(1e-9, 0.0, 0.0);
        assert!(!v.is_exactly_zero());
    }

    #[test]
    fn is_near_zero() {
        let v1 = Vec3::new(9e-9, 0.0, 0.0);
        let v2 = Vec3::new(-9e-9, 0.0, 0.0);
        assert!(v1.is_near_zero());
        assert!(v2.is_near_zero());
    }

    #[test]
    fn is_not_near_zero() {
        let v1 = Vec3::new(1e-8, 0.0, 0.0);
        let v2 = Vec3::new(-1e-8, 0.0, 0.0);
        assert!(!v1.is_near_zero());
        assert!(!v2.is_near_zero());
    }

    #[test]
    fn approx_angle_to_same_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        assert!(a.approx_angle_to(b).abs() < 1e-3) 
    }

    #[test]
    fn approx_angle_to_opposite_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, -4.0, -6.0);
        assert!((a.approx_angle_to(b).abs() - std::f32::consts::PI).abs() < 1e-3) 
    }

    #[test]
    fn approx_angle_to_compound_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 2.0);
        assert!((a.approx_angle_to(b).abs() - std::f32::consts::PI * 0.5).abs() < 1e-3) 
    }

    #[test]
    fn approx_angle_to_45() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 0.0);
        assert!((a.approx_angle_to(b).abs() - std::f32::consts::PI * 0.25).abs() < 1e-3) 
    }

    #[test]
    #[should_panic]
    fn approx_angle_to_zero() {
        let a = Vec3::new(1e-9, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 0.0);
        a.approx_angle_to(b);
    }

    #[test]
    fn angle_to_same_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        assert!(a.angle_to(b).abs() < 1e-6) 
    }

    #[test]
    fn angle_to_opposite_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, -4.0, -6.0);
        assert!((a.angle_to(b).abs() - std::f64::consts::PI).abs() < 1e-6) 
    }

    #[test]
    fn angle_to_compound_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 2.0);
        assert!((a.angle_to(b).abs() - std::f64::consts::PI * 0.5).abs() < 1e-6) 
    }

    #[test]
    fn angle_to_45() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 0.0);
        assert!((a.angle_to(b).abs() - std::f64::consts::PI * 0.25).abs() < 1e-6) 
    }

    #[test]
    #[should_panic]
    fn angle_to_zero() {
        let a = Vec3::new(1e-9, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 0.0);
        a.approx_angle_to(b);
    }

    #[test]
    fn is_perpendicular_to_unit() {
        let unit_x_pos = Vec3::new(1.0, 0.0, 0.0);
        let unit_y_pos = Vec3::new(0.0, 1.0, 0.0);
        let unit_z_pos = Vec3::new(0.0, 0.0, 1.0);

        let unit_x_neg = Vec3::new(-1.0, 0.0, 0.0);
        let unit_y_neg = Vec3::new(0.0, -1.0, 0.0);
        let unit_z_neg = Vec3::new(0.0, 0.0, -1.0);

        assert!(unit_x_pos.is_perpendicular_to(unit_y_pos));
        assert!(unit_x_pos.is_perpendicular_to(unit_z_pos));
        assert!(unit_y_pos.is_perpendicular_to(unit_z_pos));

        assert!(unit_x_neg.is_perpendicular_to(unit_y_neg));
        assert!(unit_x_neg.is_perpendicular_to(unit_z_neg));
        assert!(unit_y_neg.is_perpendicular_to(unit_z_neg));

        assert!(unit_x_pos.is_perpendicular_to(unit_y_neg));
        assert!(unit_x_pos.is_perpendicular_to(unit_z_neg));
        assert!(unit_y_pos.is_perpendicular_to(unit_z_neg));
        assert!(unit_x_neg.is_perpendicular_to(unit_y_pos));
        assert!(unit_x_neg.is_perpendicular_to(unit_z_pos));
        assert!(unit_y_neg.is_perpendicular_to(unit_z_pos));
    }

    #[test]
    fn is_perpendicular_to_same() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        assert!(!a.is_perpendicular_to(b));
    }

    #[test]
    fn is_perpendicular_to_near_zero() {
        let a = Vec3::new(1e-9, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert!(!a.is_perpendicular_to(b));
    }

    #[test]
    fn is_perpendicular_to_not_unit() {
        let a = Vec3::new(10.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 10.0, 0.0);
        assert!(a.is_perpendicular_to(b));
    }

    #[test]
    fn is_perpendicular_to_compound() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 2.0);

        let c = Vec3::new(2.0, -3.0, 5.0);
        let d = Vec3::new(1.0, 4.0, 2.0);

        assert!(a.is_perpendicular_to(b));
        assert!(b.is_perpendicular_to(a));
        assert!(c.is_perpendicular_to(d));
        assert!(d.is_perpendicular_to(c));
    }

    #[test]
    fn is_not_perpendicular_to_compound() {
        assert!(!A.is_perpendicular_to(B));
        assert!(!B.is_perpendicular_to(A));
    }

    #[test]
    fn is_parallel_to_same() {
        assert!(A.is_parallel_to(A))
    }

#[test]
    fn is_parallel_to_same_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);

        assert!(a.is_parallel_to(b));
        assert!(b.is_parallel_to(a));
    }

    #[test]
    fn is_parallel_to_opposite_direction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, -4.0, -6.0);

        assert!(a.is_parallel_to(b));
        assert!(b.is_parallel_to(a));
    }

    #[test]
    fn is_parallel_to_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 2.0);

        assert!(!a.is_parallel_to(b));
        assert!(!b.is_parallel_to(a));
    }

    #[test]
    fn is_parallel_to_near_zero() {
        let a = Vec3::new(1e-9, 0.0, 0.0);

        assert!(!a.is_parallel_to(a));
        assert!(!a.is_parallel_to(a));
    }
}
