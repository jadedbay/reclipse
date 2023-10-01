use std::{sync::{Arc, mpsc::{self, Receiver}}, collections::{HashMap, HashSet}, any::TypeId, pin::Pin};

use futures::{Future, FutureExt};

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

    pub fn get_asset_handle<T: Asset + 'static>(&mut self, file_path: &str) -> Handle<T> {
        if let Some(&asset_id) = self.paths.get(file_path) {
            Handle::<T>::new(asset_id)
        } else {
            let file_path = file_path.to_owned();
            let asset_id = self.get_new_id();
            self.paths.insert(file_path.clone(), asset_id);

            let context = Arc::clone(&self.context);
            let (tx, rx) = mpsc::channel();
            self.pending.push(rx);

            println!("loading: {}", &file_path);

            match TypeId::of::<T>() {
                id if id == TypeId::of::<Texture>() => {
                    let future = async move {
                        let texture = Texture::load(&context, &file_path).await;
                        tx.send((asset_id, AssetType::Texture(texture))).unwrap();
                    }.boxed();
                    tokio::spawn(future);
                },
                id if id == TypeId::of::<Mesh>() => {
                    let future = async move {
                        let mesh = Mesh::load(&context, &file_path).await;
                        tx.send((asset_id, AssetType::Mesh(mesh))).unwrap();
                    }.boxed();
                    tokio::spawn(future);
                },
                _ => panic!("Invalid asset type"),
            }

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

    pub fn get_primitive(&self, primitive_mesh: PrimitiveMesh) -> Handle<Mesh> {
        Handle::<Mesh>::new(primitive_mesh as usize)
    }

    fn get_new_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }
}