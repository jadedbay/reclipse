use crate::{asset::{handle::Handle, texture::Texture, mesh::{Mesh, DrawMesh}}, engine::gpu_resource::GpuResource, transform::Transform};

pub struct Entity {
    pub transform: GpuResource<Transform>,
    pub texture: Handle<Texture>,
    pub mesh: Handle<Mesh>,
}

impl Entity {
    pub fn new(transform: GpuResource<Transform>, texture: Handle<Texture>, mesh: Handle<Mesh>, ) -> Self {
        
        Self {
            transform,
            texture,
            mesh
        }
    }
}


pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, entity: &'a Entity, texture: &'a Texture, mesh: &'a Mesh);
}

impl<'a, 'b> DrawEntity<'b> for wgpu::RenderPass<'a>
where 'b: 'a,
{
    fn draw_entity(&mut self, entity: &'a Entity, texture: &'a Texture, mesh: &'a Mesh) {
        self.set_bind_group(1, &texture.bind_group.as_ref().unwrap(), &[]);
        self.set_bind_group(2, &entity.transform.bind_groups[0], &[]);
        self.draw_mesh(mesh);
    }
}