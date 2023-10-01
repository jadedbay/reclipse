pub mod mesh_pool;
pub mod texture_pool;

use std::{collections::HashMap, sync::Arc};

use super::Asset;

pub struct AssetPool<T: Asset> {
    assets: HashMap<usize, Arc<T>>,
    default: Arc<T>,
}

impl<T: Asset> AssetPool<T> {
    pub fn get(&self, id: usize) -> Arc<T> {
        match self.assets.get(&id) {
            Some(asset) => return asset.clone(),
            None => return self.default.clone()
        }
    }

    pub fn insert(&mut self, id: usize, asset: Arc<T>) {
        self.assets.insert(id, asset);
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }
}