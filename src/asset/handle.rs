use std::sync::Arc;

use crate::{asset::Asset, engine::renderer::Renderer};

use super::texture::Texture;

pub struct Handle<T: Asset> {
    asset: Arc<T>,
    pub bind_group: Arc<wgpu::BindGroup>
}

impl Handle<Texture> {
    pub fn new(device: &wgpu::Device, renderer: &Renderer, texture: Texture) -> Self {
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &renderer.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("texture_bind_group"),
            }
        );

        Self {
            asset: Arc::new(texture),
            bind_group: Arc::new(bind_group),
        }
    } 
}