use geom::Scalar;

use crate::dag::NodeId;

#[derive(Debug, Clone, Copy)]
pub struct BinaryOp{
    pub id: NodeId,
    pub left: NodeId,
    pub right: NodeId,
}

pub fn union<T: Scalar>(a: T, b: T) -> T {
    a.min(b)
}

pub fn difference<T: Scalar>(a: T, b: T) -> T {
    a.max(-b)
}
pub fn intersection<T: Scalar>(a: T, b: T) -> T {
    a.max(b)
}


