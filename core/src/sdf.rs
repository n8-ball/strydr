use geom::{Scalar, pt3::Pt3};
use std::fmt::Debug;

use crate::dag::NodeId;

pub trait Sdf<T: Scalar>: Debug {
    fn signed_distance(&self, p: Pt3<T>) -> T;

    fn id(&self) -> NodeId;
}
