use crate::engine::context::Context;

use super::mesh_pool::MeshPool;

pub struct AssetManager {
    pub mesh_pool: MeshPool
}

impl AssetManager {
    pub fn new(context: &Context) -> Self {
        Self {
            mesh_pool: MeshPool::new(context),
        }
    }
}