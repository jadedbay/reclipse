use std::sync::Arc;

use async_trait::async_trait;

use crate::engine::context::Context;

pub mod texture;
pub mod mesh;

pub mod pools;

pub mod asset_manager;
pub mod primitives;

pub mod handle;

pub use texture::Texture;
pub use mesh::Mesh;


#[async_trait]
pub trait Asset {
    async fn load(context: &Context, file_path: &str) -> Arc<Self>;
}