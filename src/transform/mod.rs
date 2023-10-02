use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::{engine::{gpu_resource::GpuResource, renderer::Renderer, context::Context}, util::cast_slice};

pub struct Transform {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: f32,

    matrix: glam::Mat4,
}

impl Transform {
    pub fn new(position: glam::Vec3, rotation: glam::Quat, scale: f32) -> Self {
        let matrix = glam::Mat4::from_scale_rotation_translation(glam::Vec3::splat(scale), rotation, position);

        Self {
            position,
            rotation,
            scale,
            matrix,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        let position = glam::vec3(0.0, 0.0, 0.0);
        let rotation = glam::Quat::IDENTITY;
        let scale = 1.0;

        let matrix = glam::Mat4::from_scale_rotation_translation(glam::Vec3::splat(scale), rotation, position);

        Self {
            position,
            rotation,
            scale,
            matrix
        }
    }
}

impl GpuResource<Transform> {
    pub fn new(context: Arc<Context>, transform: Transform) -> Self {
        let buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("matrix_buffer"),
            contents: cast_slice(&[transform.matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Renderer::get_transform_layout(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ],
            label: None,
        });
        
        Self {
            data: transform,
            buffers: vec![buffer],
            bind_groups: vec![bind_group],
            context,
        }
    }
}