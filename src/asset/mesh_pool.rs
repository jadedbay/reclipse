use std::collections::HashMap;
use std::sync::Arc;

use crate::engine::context::Context;

use super::mesh::Mesh;
use super::primitives::quad;

pub struct MeshPool {
    meshes: HashMap<usize, Arc<Mesh>>,
}

impl MeshPool {
    pub fn new(context: &Context) -> Self {
        let meshes = Self::load_primitives(context);

        Self {
            meshes,
        }
    }

    fn load_primitives(context: &Context) -> HashMap<usize, Arc<Mesh>> {
        let quad = Arc::new(Mesh::new(context, quad::VERTICES, quad::INDICES));

        let mut meshes = HashMap::new();
        meshes.insert(0, quad);

        meshes
    } 

    pub fn get_mesh(&self, mesh_id: usize) -> Arc<Mesh> {
        self.meshes.get(&mesh_id).unwrap().clone()
    }
}