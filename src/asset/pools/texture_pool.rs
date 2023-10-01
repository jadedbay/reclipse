use std::{collections::HashMap, sync::Arc};

use crate::{asset::texture::Texture, engine::context::Context};

use super::AssetPool;

impl AssetPool<Texture> {
    pub fn new(context: &Context) -> Self {
        let default = Arc::new(Texture::from_bytes(context, include_bytes!("../../../res/textures/default_texture.png"), false).unwrap());

        Self {
            assets: HashMap::new(),
            default,
        }
    }
}