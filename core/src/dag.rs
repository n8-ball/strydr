
use geom::{Pt3, Scalar, bbox3::Bbox3};

use crate::{
    Sphere,
    ops::{self, BinaryOp},
    sdf::Sdf,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId(pub usize);

#[derive(Debug)]
enum Node<T: Scalar> {
    Background,
    Sphere(Sphere<T>),
    Union(BinaryOp),
    Difference(BinaryOp),
    Intersection(BinaryOp),
}

pub struct Graph<T: Scalar> {
    nodes: Vec<Node<T>>,
}

impl<T: Scalar> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::Background],
        }
    }

    pub fn sphere(&mut self, origin: Pt3<T>, radius: T) -> Sphere<T> {
        let id = self.nodes.len();
        let node = Sphere::new(id, origin, radius);
        self.nodes.push(Node::Sphere(node.clone()));
        node
    }

    pub fn union<A: Sdf<T>, B: Sdf<T>>(&mut self, a: A, b: B) -> BinaryOp {
        let id = self.nodes.len();
        let node = BinaryOp{
            id: NodeId(id),
            left: a.id(),
            right: b.id(),
        };
        self.nodes.push(Node::Union(node.clone()));
        node
    }

    pub fn difference<A: Sdf<T>, B: Sdf<T>>(&mut self, a: A, b: B) -> BinaryOp {
        let id = self.nodes.len();
        let node = BinaryOp{
            id: NodeId(id),
            left: a.id(),
            right: b.id(),
        };
        self.nodes.push(Node::Difference(node.clone()));
        node
    }

    pub fn intersection<A: Sdf<T>, B: Sdf<T>>(&mut self, a: A, b: B) -> BinaryOp {
        let id = self.nodes.len();
        let node = BinaryOp{
            id: NodeId(id),
            left: a.id(),
            right: b.id(),
        };
        self.nodes.push(Node::Intersection(node.clone()));
        node
    }

    pub fn eval(&self, p: Pt3<T>) -> T {
        let mut values = vec![T::ZERO; self.nodes.len()];

        for (i, node) in self.nodes.iter().enumerate() {
            values[i] = match node {
                Node::Background => T::ZERO,
                Node::Sphere(sphere) => sphere.signed_distance(p),
                Node::Union(node) => ops::union(values[node.left.0], values[node.right.0]),
                Node::Difference(node) => ops::difference(values[node.left.0], values[node.right.0]),
                Node::Intersection(node) => ops::intersection(values[node.left.0], values[node.right.0]),
            }
        }

        values[self.nodes.len() - 1]
    }

    #[must_use]
    pub fn estimate_bbox(&self) -> Bbox3<T> {
        let mut bbox3s= vec![Bbox3::<T>::ZERO; self.nodes.len()];

        for (i, node) in self.nodes.iter().enumerate() {
            bbox3s[i] = match node {
                Node::Background => Bbox3::<T>::ZERO,
                Node::Sphere(sphere) => sphere.calculate_bbox(),
                Node::Union(node) => Bbox3::union(bbox3s[node.left.0], bbox3s[node.right.0]),
                Node::Difference(node) => Bbox3::difference(bbox3s[node.left.0], bbox3s[node.right.0]),
                Node::Intersection(node) => Bbox3::intersection(bbox3s[node.left.0], bbox3s[node.right.0]),
            }
        };

        bbox3s[self.nodes.len() - 1]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn dump(&self) {
        for node in &self.nodes {
            println!("{node:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom::{pt3::{Pt3}, scalar_test};

    scalar_test!(test_new, |T| {
        let g = Graph::<T>::new();

        // the graph has a background node on initialization.
        assert_eq!(g.node_count(), 1);
    });

    scalar_test!(test_add_sphere_and_eval, |T| {
        let mut g = Graph::<T>::new();

        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let _ = g.sphere(origin, radius);

        assert_eq!(g.node_count(), 2);

        let sample_origin = Pt3::<T>::ZERO;
        let sample_boundary = Pt3::<T>::new(radius, 0.0, 0.0);
        let sample_between = Pt3::<T>::new(radius * 0.5, 0.0, 0.0);

        assert_eq!(g.eval(sample_origin), -radius);
        assert_eq!(g.eval(sample_boundary), 0.0);
        assert_eq!(g.eval(sample_between), -radius * 0.5);
    });

    scalar_test!(test_bool_add_basic, |T| {
        let mut g = Graph::<T>::new();
        let radius = T::from_f32(10.0);

        let origin1 = Pt3::<T>::new(-radius, T::ZERO, T::ZERO);
        let origin2 = Pt3::<T>::new(radius, T::ZERO, T::ZERO);

        let sphere1 = g.sphere(origin1, radius);
        let sphere2 = g.sphere(origin2, radius);

        let _union = g.union(sphere1, sphere2);

        assert_eq!(g.node_count(), 4);

        // Center of first sphere
        assert_eq!(g.eval(origin1), -radius);

        // Center of second sphere
        assert_eq!(g.eval(origin2), -radius);

        // Touching point between spheres
        assert_eq!(g.eval(Pt3::ZERO), T::ZERO);

        // Inside first sphere only
        let p = Pt3::<T>::new(-radius * T::from_f32(1.5), T::ZERO, T::ZERO);

        assert_eq!(g.eval(p), -radius * T::from_f32(0.5));

        // Outside both
        let outside = Pt3::<T>::new(radius * T::from_f32(3.0), T::ZERO, T::ZERO);

        assert_eq!(g.eval(outside), radius);
    });

    scalar_test!(test_add_sphere_and_calculate_bbox, |T| {
        let mut g = Graph::<T>::new();

        let _ = g.sphere(Pt3::<T>::ZERO, 10.0);

        assert_eq!(g.node_count(), 2);

        let bbox = g.estimate_bbox();

        assert_eq!(bbox.min, Pt3::<T>::new(-10.0, -10.0, -10.0));
        assert_eq!(bbox.max, Pt3::<T>::new(10.0, 10.0, 10.0));
    });

    scalar_test!(test_union_spheres_and_calculate_bbox, |T| {
        let mut g = Graph::<T>::new();

        let sphere1 = g.sphere(Pt3::<T>::ZERO, 10.0);
        let sphere2 = g.sphere(Pt3::<T>::new(10.0, 0.0, 0.0), 10.0);

        let _ = g.union(sphere1, sphere2);

        let bbox = g.estimate_bbox();

        assert_eq!(bbox.min, sphere1.calculate_bbox().min);
        assert_eq!(bbox.max, sphere2.calculate_bbox().max);
    });

    scalar_test!(test_subtract_spheres_and_calculate_bbox, |T| {
        let mut g = Graph::<T>::new();

        let sphere1 = g.sphere(Pt3::<T>::ZERO, 10.0);
        let sphere2 = g.sphere(Pt3::<T>::new(10.0, 0.0, 0.0), 10.0);

        let _ = g.difference(sphere1, sphere2);

        let bbox = g.estimate_bbox();

        assert_eq!(bbox.min, sphere1.calculate_bbox().min);
        assert_eq!(bbox.max, sphere1.calculate_bbox().max);
    });

    scalar_test!(test_intersect_spheres_and_calculate_bbox, |T| {
        let mut g = Graph::<T>::new();

        let sphere1 = g.sphere(Pt3::<T>::ZERO, 10.0);
        let sphere2 = g.sphere(Pt3::<T>::new(10.0, 0.0, 0.0), 10.0);

        let _ = g.intersection(sphere1, sphere2);

        let bbox = g.estimate_bbox();

        assert_eq!(bbox.min, sphere2.calculate_bbox().min);
        assert_eq!(bbox.max, sphere1.calculate_bbox().max);
    });

    scalar_test!(test_print, |T| {
        let mut g = Graph::<T>::new();

        let radius = 10.0;
        let origin = Pt3::<T>::ZERO;

        let _ = g.sphere(origin, radius);

        g.dump();
    });
}
