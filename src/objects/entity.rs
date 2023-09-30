use std::sync::Arc;

use crate::{asset::{texture::Texture, handle::Handle, mesh::{Mesh, DrawMesh}, primitives::PrimitiveMesh, mesh_pool::MeshPool}, util::cast_slice, engine::{vertex::Vertex, gpu_resource::GpuResource}, transform::Transform};

pub struct Entity {
    pub texture: Handle<Texture>,
    pub mesh: Arc<Mesh>,
    //pub transform: GpuResource<Transform>,
}

impl Entity {
    pub fn new(texture: Handle<Texture>, mesh: Arc<Mesh>) -> Self {
        Self {
            texture,
            mesh
        }
    }
}

pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, sprite: &'a Entity);
}

impl<'a, 'b> DrawEntity<'b> for wgpu::RenderPass<'a>
where 'b: 'a,
{
    fn draw_entity(&mut self, entity: &'a Entity) {
        self.set_bind_group(0, &entity.texture.asset.bind_group.as_ref().unwrap(), &[]);
        self.draw_mesh(&entity.mesh);
    }
}