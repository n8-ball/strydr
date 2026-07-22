use geom::{Pt3, Scalar, Vec3};

pub struct Qef<T: Scalar> {
    xx: T,
    xy: T,
    xz: T,
    yy: T,
    yz: T,
    zz: T,
    bx: T,
    by: T,
    bz: T,
}

impl<T: Scalar> Qef<T> {
    pub fn new() -> Self {
        Self {
            xx: T::ZERO,
            xy: T::ZERO,
            xz: T::ZERO,
            yy: T::ZERO,
            yz: T::ZERO,
            zz: T::ZERO,
            bx: T::ZERO,
            by: T::ZERO,
            bz: T::ZERO,
        }
    }

    pub fn push_sample(&mut self, n: Vec3<T>, d: T) {
        let nx = n.x;
        let ny = n.y;
        let nz = n.z;

        self.xx += nx * nx;
        self.xy += nx * ny;
        self.xz += nx * nz;
        self.yy += ny * ny;
        self.yz += ny * nz;
        self.zz += nz * nz;
        self.bx += nx * d;
        self.by += ny * d;
        self.bz += nz * d;
    }

    pub fn solve_cramers(&self) -> Option<Pt3<T>> {
        let (m00, m01, m02) = (self.xx, self.xy, self.xz);
        let (m10, m11, m12) = (self.xy, self.yy, self.yz);
        let (m20, m21, m22) = (self.xz, self.yz, self.zz);
        let (b0, b1, b2) = (self.bx, self.by, self.bz);

        let det = m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20)
            + m02 * (m10 * m21 - m11 * m20);

        if det.abs() < T::EPSILON {
            return None;
        }

        let det_x = b0 * (m11 * m22 - m12 * m21) - m01 * (b1 * m22 - m12 * b2)
            + m02 * (b1 * m21 - m11 * b2);

        let det_y = m00 * (b1 * m22 - m12 * b2) - b0 * (m10 * m22 - m12 * m20)
            + m02 * (m10 * b2 - b1 * m20);

        let det_z = m00 * (m11 * b2 - b1 * m21) - m01 * (m10 * b2 - b1 * m20)
            + b0 * (m10 * m21 - m11 * m20);

        let result = Pt3::new(det_x / det, det_y / det, det_z / det);

        if result.x.is_finite() && result.y.is_finite() && result.z.is_finite() {
            return Some(result);
        } else {
            None
        }
    }

    fn solve_svd() {
        todo!()
    }
}