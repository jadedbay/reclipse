use std::{collections::HashMap, sync::Arc};

use serde::{Serialize, Deserialize};

use super::{texture::Texture, handle::Handle};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Texture,
}


#[derive(Clone, Serialize, Deserialize)]
struct AssetMetadata {
    id: usize,
    path: String,
    pub asset_type: AssetType,
}

pub struct AssetManager {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    textures: HashMap<usize, Handle<Texture>>,
    metadata: HashMap<usize, AssetMetadata>,
}

impl AssetManager {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let mut metadata = HashMap::new();
        let assets_yaml = include_str!("../../assets.yaml");
        let assets: HashMap<String, AssetMetadata> = serde_yaml::from_str(assets_yaml).unwrap();

        for (path, asset) in assets {
            let id = asset.id;
            metadata.insert(id, asset);
        }

        for entry in std::fs::read_dir("res").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = path.to_str().unwrap();

            if !metadata.contains_key(&path_str) {
                let id = metadata.len();
                let asset = AssetMetadata {
                    id,
                    path: path_str.to_string(),
                    asset_type: AssetType::Texture,
                };
                metadata.insert(id, asset);
            }
        }

        Self {
            device,
            queue,
            textures: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    
}