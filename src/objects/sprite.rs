use std::sync::Arc;

use once_cell::sync::OnceCell;
use wgpu::util::DeviceExt;

use crate::{asset::{texture::Texture, handle::Handle}, util::cast_slice, engine::{vertex::Vertex, gpu_resource::GpuResource}, transform::Transform};

pub struct Sprite {
    pub texture: Handle<Texture>,
    pub mesh: Arc<SpriteMesh>,
    //pub transform: GpuResource<Transform>,
}

impl Sprite {
    pub fn new(texture: Handle<Texture>) -> Self {
        let mesh = SPRITE_MESH.get().unwrap();

        Self {
            texture,
            mesh: mesh.clone(),
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

static SPRITE_MESH: OnceCell<Arc<SpriteMesh>> = OnceCell::new();

pub struct SpriteMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl SpriteMesh {
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


        let mesh = Self {
            vertex_buffer,
            index_buffer,
            index_count: INDEX_COUNT,
        };

        let _ = SPRITE_MESH.set(Arc::new(mesh));
    }
}