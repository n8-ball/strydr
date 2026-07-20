use core::dag::Graph;

use geom::Scalar;

pub fn dual_contour<T: Scalar>(
    graph: &Graph<T>,
    resolution: T,
) {
    let bbox = graph.estimate_bbox();

    let i_dim = T::to_f64(bbox.max.x - bbox.min.x).ceil() as usize;
    let j_dim = T::to_f64(bbox.max.x - bbox.min.x).ceil() as usize;
    let k_dim = T::to_f64(bbox.max.x - bbox.min.x).ceil() as usize;
}