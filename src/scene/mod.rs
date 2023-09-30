use std::sync::Arc;

use crate::{objects::entity::Entity, asset::{asset_manager::AssetManager, handle::Handle, texture::Texture, primitives::PrimitiveMesh}};

pub struct Scene {
    asset_manager: Arc<AssetManager>,
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(asset_manager: Arc<AssetManager>) -> Self {
        Self {
            asset_manager,
            entities: Vec::new(),
        }
    }

    pub fn create_sprite(&mut self, texture: Handle<Texture>) {
        let mesh = self.asset_manager.mesh_pool.get_mesh(PrimitiveMesh::Quad as usize);
        self.entities.push(Entity::new(texture, mesh));
    }
}