use crate::asset::{handle::Handle, texture::Texture, mesh::{Mesh, DrawMesh}};

pub struct Entity {
    pub texture: Handle<Texture>,
    pub mesh: Handle<Mesh>,
    //pub transform: GpuResource<Transform>,
}

impl Entity {
    pub fn new(texture: Handle<Texture>, mesh: Handle<Mesh>) -> Self {
        Self {
            texture,
            mesh
        }
    }
}

pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, _entity: &'a Entity, texture: &'a Texture, mesh: &'a Mesh);
}

impl<'a, 'b> DrawEntity<'b> for wgpu::RenderPass<'a>
where 'b: 'a,
{
    fn draw_entity(&mut self, _entity: &'a Entity, texture: &'a Texture, mesh: &'a Mesh) {
        self.set_bind_group(0, &texture.bind_group.as_ref().unwrap(), &[]);
        self.draw_mesh(mesh);
    }
}