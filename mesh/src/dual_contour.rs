use core::dag::Graph;

use geom::{Pt3, Scalar};

pub fn dual_contour<T: Scalar>(graph: &Graph<T>, resolution: T) {
    let bbox = graph.estimate_bbox();

    let i_dim = (T::to_f64(bbox.max.x - bbox.min.x).ceil() / T::to_f64(resolution)) as usize;
    let j_dim = (T::to_f64(bbox.max.x - bbox.min.x).ceil() / T::to_f64(resolution)) as usize;
    let k_dim = (T::to_f64(bbox.max.x - bbox.min.x).ceil() / T::to_f64(resolution)) as usize;

    let stride_thresh = resolution / T::HALF;

    for i in 0..i_dim {
        let x = bbox.min.x + (resolution * T::from_f64(i as f64));
        for j in 0..j_dim {
            let y = bbox.min.y + (resolution * T::from_f64(j as f64));
            let mut k = 0;
            while k < k_dim {
                let z = bbox.min.z + (resolution * T::from_f64(k as f64));

                let p = Pt3::<T>::new(x, y, z);
                let sd = graph.excecute(p);

                if sd < stride_thresh {
                    k += T::to_f64(sd).floor() as usize;
                }
            }
        }
    }
}
