use core::dag::Graph;

use geom::{Pt3, Scalar, Vec3};

use crate::qef::Qef;

//
//         6 -------- 7
//        /          /       Z
//       / |        / |      ^  _ Y
//      4----------5  |      | /
//      |  |       |  |      |/
//      |  2-------|--3      ---> X
//      | /        | /
//      |/         |/
//      0----------1
//

pub struct Cell<T: Scalar> {
    center: Pt3<T>,
    lwh: T,
    corners: [Pt3<T>; 8],
}

impl<T: Scalar> Cell<T> {
    fn new(center: Pt3<T>, lwh: T) -> Self {
        let s = lwh * T::HALF;

        let (xm, xp) = (center.x - s, center.x + s);
        let (ym, yp) = (center.y - s, center.y + s);
        let (zm, zp) = (center.z - s, center.z + s);
        Self { 
            center, lwh, corners:
        [
            Pt3 {
                x: xm,
                y: ym,
                z: zm,
            },
            Pt3 {
                x: xp,
                y: ym,
                z: zm,
            },
            Pt3 {
                x: xm,
                y: yp,
                z: zm,
            },
            Pt3 {
                x: xp,
                y: yp,
                z: zm,
            },
            Pt3 {
                x: xm,
                y: ym,
                z: zp,
            },
            Pt3 {
                x: xp,
                y: ym,
                z: zp,
            },
            Pt3 {
                x: xm,
                y: yp,
                z: zp,
            },
            Pt3 {
                x: xp,
                y: yp,
                z: zp,
            },
        ]
        }
    }

    fn try_activate(&self, graph: &Graph<T>, resolution: T) -> Option<Pt3<T>> {
        let mut qef = Qef::<T>::new();
        let mut crossing_points = Vec::new();

        // Eval all 12 edges of the cell. 
        for edge in self.edges() {

            let a = *edge[0];
            let b = *edge[1];


            let a_sd = graph.excecute(a);
            let b_sd = graph.excecute(b);

            // Check for crossing
            if a_sd * b_sd >= T::ZERO {
                continue;
            }

            let t = a_sd / (a_sd - b_sd);

            // Locate the crossing on the edge
            let p = a.lerp_to(b, t);
            crossing_points.push(p);
            let n = graph.calculate_normal_finite_difference(p, resolution);
            qef.push_sample(n, Vec3::<T>::new(p.x, p.y, p.z).dot(n));
        }

        if crossing_points.is_empty() {
            None
        } else {
            match qef.solve_cramers() {
                Some(p) => 
                    if self.is_inside(p) {
                        return Some(p)
                    } else {
                        return Some(Pt3::<T>::average(&crossing_points))
                    },
            None => Some(Pt3::<T>::average(&crossing_points)),
            }
        }
    }

    fn edges(&self) -> [[&Pt3<T>; 2]; 12] {
        [
            // Bottom face
            [ &self.corners[0], &self.corners[1] ],
            [ &self.corners[1], &self.corners[3] ],
            [ &self.corners[3], &self.corners[2] ],
            [ &self.corners[2], &self.corners[0] ],

            // Top face
            [ &self.corners[4], &self.corners[5] ],
            [ &self.corners[5], &self.corners[7] ],
            [ &self.corners[7], &self.corners[6] ],
            [ &self.corners[6], &self.corners[4] ],

            // Verticals 
            [ &self.corners[0], &self.corners[4] ],
            [ &self.corners[1], &self.corners[5] ],
            [ &self.corners[2], &self.corners[6] ],
            [ &self.corners[3], &self.corners[7] ],
        ]
    }

    fn is_inside(&self, p: Pt3<T>) -> bool {
        let corners = self.corners;
        p.x >= corners[0].x
            && p.x <= corners[7].x
            && p.y >= corners[0].y
            && p.y <= corners[7].y
            && p.z >= corners[0].z
            && p.z <= corners[7].z
    }
}
