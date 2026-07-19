use geom::Scalar;

use crate::dag::NodeId;

#[derive(Debug, Clone, Copy)]
pub struct BoolAdd {
    pub id: NodeId,
    pub left: NodeId,
    pub right: NodeId,
}

pub fn bool_add<T: Scalar>(a: T, b: T) -> T {
    a.min(b)
}

pub fn bool_sub<T: Scalar>(a: T, b: T) -> T {
    a.max(-b)
}
