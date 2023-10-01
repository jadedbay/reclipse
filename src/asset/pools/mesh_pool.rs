use std::collections::HashMap;
use std::sync::Arc;

use crate::asset::mesh::Mesh;
use crate::asset::primitives::quad;
use crate::engine::context::Context;

use super::AssetPool;


impl AssetPool<Mesh> {
    pub fn new(context: &Context) -> Self {
        let meshes = Self::load_primitives(context);
        let default = meshes.get(&0).unwrap().clone();

        Self {
            assets: meshes,
            default,
        }
    }

    fn load_primitives(context: &Context) -> HashMap<usize, Arc<Mesh>> {
        let quad = Arc::new(Mesh::new(context, quad::VERTICES, quad::INDICES));

        let mut meshes = HashMap::new();
        meshes.insert(0, quad);

        meshes
    }
}