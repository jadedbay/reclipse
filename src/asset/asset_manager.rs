use std::{sync::{Arc, mpsc::{self, Receiver}}, collections::HashMap, any::TypeId};

use futures::FutureExt;

use crate::engine::context::Context;

use super::{pools::AssetPool, mesh::Mesh, texture::Texture, handle::Handle, Asset, primitives::PrimitiveMesh};

enum AssetType {
    Texture(Arc<Texture>),
    Mesh(Arc<Mesh>),
}

pub struct AssetManager {
    context: Arc<Context>,

    meshes: AssetPool<Mesh>,
    textures: AssetPool<Texture>,
    paths: HashMap<String, usize>,

    pending: Vec<Receiver<(usize, AssetType)>>,

    next_id: usize,
}

impl AssetManager {
    pub fn new(context: Arc<Context>) -> Self {
        let meshes = AssetPool::<Mesh>::new(&context);
        let textures = AssetPool::<Texture>::new(&context);

        let next_id = meshes.len() + textures.len();
        
        Self {
            context,

            meshes,
            textures,
            paths: HashMap::new(),
            
            pending: Vec::new(),

            next_id,
        }
    }

    pub fn get_handle<T: Asset + 'static>(&mut self, file_path: &str) -> Handle<T> {
        let file_path = file_path.to_owned();

        if let Some(&asset_id) = self.paths.get(&file_path) {
            Handle::<T>::new(asset_id)
        } else {
            let asset_id = self.get_new_id();
            self.paths.insert(file_path.clone(), asset_id);

            let context = Arc::clone(&self.context);
            let (tx, rx) = mpsc::channel();
            self.pending.push(rx);

            let future = async move {
                match TypeId::of::<T>() {
                    id if id == TypeId::of::<Texture>() => {
                        let texture = Texture::load(&context, &file_path).await;
                        tx.send((asset_id, AssetType::Texture(texture))).unwrap();
                    },
                    id if id == TypeId::of::<Mesh>() => {
                        let mesh = Mesh::load(&context, &file_path).await;
                        tx.send((asset_id, AssetType::Mesh(mesh))).unwrap();
                    },
                    _ => panic!("Invalid asset type"),
                }
            }.boxed();
            tokio::spawn(future);

            Handle::<T>::new(asset_id)
        }
    }

    pub fn process_pending(&mut self) {
        let mut i = 0;
        while i != self.pending.len() {
            if let Ok((asset_id, asset)) = self.pending[i].try_recv() {
                match asset {
                    AssetType::Texture(texture) => {
                        self.textures.insert(asset_id, texture);
                    },
                    AssetType::Mesh(mesh) => {
                        self.meshes.insert(asset_id, mesh);
                    },
                }
                self.pending.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn get_texture(&self, handle: &Handle<Texture>) -> Arc<Texture> {
        self.textures.get(handle.asset_id)
    }

    pub fn get_mesh(&self, handle: &Handle<Mesh>) -> Arc<Mesh> {
        self.meshes.get(handle.asset_id)
    }

    pub fn get_primitive_handle(&self, primitive_mesh: PrimitiveMesh) -> Handle<Mesh> {
        Handle::<Mesh>::new(primitive_mesh as usize)
    }

    pub fn get_primitive_mesh(&self, primitive_mesh: PrimitiveMesh) -> Arc<Mesh> {
        self.meshes.get(primitive_mesh as usize)
    }

    fn get_new_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }
}