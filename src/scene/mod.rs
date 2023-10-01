use std::sync::Arc;

use crate::{objects::entity::Entity, asset::{asset_manager::AssetManager, handle::Handle, texture::Texture, primitives::PrimitiveMesh}, engine::gpu_resource::GpuResource};

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
}