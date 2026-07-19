use geom::{bbox3::Bbox3, pt3::Pt3, scalar::Scalar};
use std::{collections::{HashMap, HashSet}, fs::File, io::{self, Write}};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Triangle(pub usize, pub usize, pub usize);

impl Triangle {
    pub fn canonical_triangle(mut tri: [usize; 3]) -> Triangle {
        tri.sort();
        Triangle(tri[0], tri[1], tri[2])
    }
}

pub struct Mesh<T: Scalar> {
    verticies: Vec<Pt3<T>>,
    triangles: Vec<Triangle>,
    vertex_lookup: HashMap<(i64, i64, i64), usize>,
    triangle_lookup: HashSet<Triangle>,
}

impl<T: Scalar> Mesh<T> {
    pub fn new() -> Self {
        Self {
            verticies: Vec::new(),
            triangles: Vec::new(),
            vertex_lookup: HashMap::new(),
            triangle_lookup: HashSet::new(),
        }
    }
    pub fn add_triangle(&mut self, verticies: [Pt3<T>; 3]) {
        let mut tri_idxs = [0usize; 3];

        for i in 0..verticies.len() {
            let tri_idx = *self
                .vertex_lookup
                .entry(quant(verticies[i]))
                .or_insert_with(|| {
                    let idx = self.verticies.len();
                    self.verticies.push(verticies[i]);
                    idx
                });
            tri_idxs[i] = tri_idx;
        }

        // Degenerate triangle
        if tri_idxs[0] == tri_idxs[1] 
        || tri_idxs[0] == tri_idxs[2] 
        || tri_idxs[1] == tri_idxs[2] {
            return;
        }
            
        let triangle = Triangle::canonical_triangle(tri_idxs);

        if self.triangle_lookup.insert(triangle) {
            self.triangles.push(triangle);
        }
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.verticies.len()
    }

    pub fn calculate_bounding_box(&self) -> Option<Bbox3<T>> {
        if self.verticies.is_empty() {
            return None;
        }

        let mut min = Pt3::<T>::MAX;
        let mut max = Pt3::<T>::MIN;

        for v in &self.verticies {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);

            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        Some(Bbox3::<T>::new(min, max))
    }

    fn arrow() -> () {

    }

    fn write_ascii_stl(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "solid mesh")?;

        for tri in &self.triangles {
            let a = self.verticies[tri.0];
            let b = self.verticies[tri.1];
            let c = self.verticies[tri.2];
            
            let u = b - a;
            let v = c - a;

            let n = u.cross(v).normalize();

            writeln!(file, "  facet normal {} {} {}", n.x, n.y, n.z)?;
            writeln!(file, "    outer loop")?;
            writeln!(file, "      vertex {} {} {}", a.x, a.y, a.z)?;
            writeln!(file, "      vertex {} {} {}", b.x, b.y, b.z)?;
            writeln!(file, "      vertex {} {} {}", c.x, c.y, c.z)?;
            writeln!(file, "    endloop")?;
            writeln!(file, "  endfacet")?;
        }

        writeln!(file, "endsolid mesh")?;

        Ok(())
    }

    fn from_stl() {
        todo!()
    }
}

fn quant<T: Scalar>(v: Pt3<T>) -> (i64, i64, i64) {
    let scale: f64 = 1_000_000.0;

    (
        (v.x.to_f64() * scale).round() as i64,
        (v.y.to_f64() * scale).round() as i64,
        (v.z.to_f64() * scale).round() as i64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom::scalar_test;

    scalar_test!(test_single_triangle, |T| {
        let v1 = Pt3::<T>::ZERO;
        let v2 = Pt3::<T>::new(10.0, 0.0, 0.0);
        let v3 = Pt3::<T>::new(0.0, 10.0, 0.0);

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v1, v2, v3]);

        assert!(mesh.triangle_count() == 1);
        assert!(mesh.vertex_count() == 3);
    });

    scalar_test!(test_duplicate_triangle_same_order, |T| {
        let v1 = Pt3::<T>::ZERO;
        let v2 = Pt3::<T>::new(10.0, 0.0, 0.0);
        let v3 = Pt3::<T>::new(0.0, 10.0, 0.0);

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v1, v2, v3]);
        mesh.add_triangle([v1, v2, v3]);

        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.vertex_count(), 3);
    });

    scalar_test!(test_duplicate_triangle_different_order, |T| {
        let v1 = Pt3::<T>::ZERO;
        let v2 = Pt3::<T>::new(10.0, 0.0, 0.0);
        let v3 = Pt3::<T>::new(0.0, 10.0, 0.0);

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v1, v2, v3]);
        mesh.add_triangle([v3, v2, v1]); // reversed

        assert_eq!(mesh.triangle_count(), 1); // if you canonicalize
    });

    scalar_test!(test_shared_vertices, |T| {
        let v1 = Pt3::<T>::ZERO;
        let v2 = Pt3::<T>::new(10.0, 0.0, 0.0);
        let v3 = Pt3::<T>::new(0.0, 10.0, 0.0);
        let v4 = Pt3::<T>::new(10.0, 10.0, 0.0);

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v1, v2, v3]);
        mesh.add_triangle([v2, v4, v3]);

        assert_eq!(mesh.triangle_count(), 2);
        assert_eq!(mesh.vertex_count(), 4); // NOT 6
    });
    scalar_test!(test_quantization_boundary, |T| {
        let v1 = Pt3::<T>::new(0.0, 0.0, 0.0);
        let v1b = Pt3::<T>::new(0.01, 0.0, 0.0); // should NOT merge

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v1, v1b, v1]);

        assert!(mesh.vertex_count() >= 2);
    });
    scalar_test!(test_degenerate_triangle, |T| {
        let v = Pt3::<T>::ZERO;

        let mut mesh = Mesh::<T>::new();
        mesh.add_triangle([v, v, v]);

        // depends on your policy:
        // either reject or accept
        assert_eq!(mesh.triangle_count(), 0); // recommended
    });
    scalar_test!(test_many_triangles, |T| {
        let mut mesh = Mesh::<T>::new();

        for i in 0..1000 {
            let x = T::from_f64(i as f64);
            let v1 = Pt3::<T>::new(x, 0.0, 0.0);
            let v2 = Pt3::<T>::new(x, 1.0, 0.0);
            let v3 = Pt3::<T>::new(x, 0.0, 1.0);

            mesh.add_triangle([v1, v2, v3]);
        }

        assert_eq!(mesh.triangle_count(), 1000);
    });
}
