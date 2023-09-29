use std::sync::Arc;

use crate::engine::context::Context;

use super::mesh::Mesh;

pub mod quad;

pub enum PrimitiveMesh {
    Quad = 0,
}

pub struct Primitives {
    meshes: [Arc<Mesh>; 1],
}

impl Primitives {
    pub fn new(context: &Context) -> Self {
        let quad = Arc::new(Mesh::new(context, quad::VERTICES, quad::INDICES));
        let meshes = [quad];

        Self {
            meshes,
        }
    }

    pub fn get_mesh(&self, mesh: PrimitiveMesh) -> Arc<Mesh> {
        self.meshes[mesh as usize].clone()
    }
}