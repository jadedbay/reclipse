use crate::{engine::gpu_resource::GpuResource, transform::Transform, asset::{Texture, Mesh, mesh::DrawMesh, handle::Handle}};

pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, entity: &'a GpuResource<Transform>, texture: &'a Texture, mesh: &'a Mesh);
}

impl<'a, 'b> DrawEntity<'b> for wgpu::RenderPass<'a>
where 'b: 'a,
{
    fn draw_entity(&mut self, transform: &'a GpuResource<Transform>, texture: &'a Texture, mesh: &'a Mesh) {
        self.set_bind_group(1, &texture.bind_group.as_ref().unwrap(), &[]);
        self.set_bind_group(2, &transform.bind_groups[0], &[]);
        self.draw_mesh(mesh);
    }
}