use geom::{scalar::Scalar, vec3::Vec3};

pub struct Mesh<T: Scalar> {
    verticies: Vec<Vec3<T>>,
    triangles: Vec<Triangle>,
}

impl<T: Scalar> Mesh<T> {
    fn new() -> Self {
        Self {
            verticies: Vec::new(), 
            triangles: Vec::new(),
        }
    }
}

pub struct Triangle(usize, usize, usize);



