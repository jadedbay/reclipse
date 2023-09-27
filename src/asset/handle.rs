use std::sync::Arc;

use crate::{asset::Asset, engine::renderer::Renderer};

use super::texture::Texture;

pub struct Handle<T: Asset> {
    pub asset: Arc<T>,
}

impl Handle<Texture> {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self {
            asset: texture,
        }
    } 
}