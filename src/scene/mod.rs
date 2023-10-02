use std::sync::Arc;

use crate::{objects::entity::Entity, asset::{asset_manager::AssetManager, handle::Handle, texture::Texture, primitives::PrimitiveMesh, mesh::Mesh}, engine::{gpu_resource::GpuResource, context::Context}, transform::Transform};

pub struct Scene {
    context: Arc<Context>,
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(context: Arc<Context>) -> Self {
        Self {
            context,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self, transform: Transform, texture: Handle<Texture>, mesh: Handle<Mesh>) {
        let transform = GpuResource::new(self.context.clone(), transform);
        
        let entity = Entity::new(transform, texture, mesh);
        self.entities.push(entity);
    }
}