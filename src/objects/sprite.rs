use std::sync::Arc;

use crate::{asset::{texture::Texture, handle::Handle, mesh::Mesh, primitives::{Primitives, PrimitiveMesh}}, util::cast_slice, engine::{vertex::Vertex, gpu_resource::GpuResource}, transform::Transform};

pub struct Sprite {
    pub texture: Handle<Texture>,
    pub mesh: Arc<Mesh>,
    //pub transform: GpuResource<Transform>,
}

impl Sprite {
    pub fn new(texture: Handle<Texture>, primitives: &Primitives) -> Self {
        let mesh = primitives.get_mesh(PrimitiveMesh::Quad);

        Self {
            texture,
            mesh,
        }
    }
}

pub trait DrawSprite<'a> {
    fn draw_sprite(&mut self, sprite: &'a Sprite);
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where 'b: 'a,
{
    fn draw_sprite(&mut self, sprite: &'a Sprite) {
        self.set_bind_group(0, &sprite.texture.asset.bind_group.as_ref().unwrap(), &[]);
        self.set_vertex_buffer(0, sprite.mesh.vertex_buffer.slice(..));
        self.set_index_buffer(sprite.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..sprite.mesh.index_count, 0, 0..1);
    }
}