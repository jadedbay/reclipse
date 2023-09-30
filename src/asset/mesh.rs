use wgpu::util::DeviceExt;

use crate::{engine::{vertex::Vertex, context::Context}, util::cast_slice};

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(context: &Context, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh);
}

impl<'a, 'b> DrawMesh<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..mesh.index_count, 0, 0..1);
    }
}