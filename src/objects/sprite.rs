use wgpu::util::DeviceExt;

use crate::{asset::texture, util::cast_slice, engine::vertex::Vertex};

static VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

static INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3
];

pub struct Sprite {
    texture: texture::Texture,
    vertex_buffer: &'static wgpu::Buffer,
    index_buffer: &'static wgpu::Buffer,
}

impl Sprite {
    pub fn init(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        (vertex_buffer, index_buffer)
    }

    pub fn new(texture: texture::Texture, vertex_buffer: &'static wgpu::Buffer, index_buffer: &'static wgpu::Buffer) -> Self {
        Self {
            texture,
            vertex_buffer,
            index_buffer,
        }
    }
}
