use crate::engine::vertex::Vertex;

static VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

static INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3
];

static INDEX_COUNT: u32 = INDICES.len() as u32;

pub struct Quad {
    
}

impl Quad {
    pub fn load(device: &wgpu::Device) {
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
    }
}