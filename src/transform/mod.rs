use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::{engine::{gpu_resource::GpuResource, renderer::Renderer, context::Context}, util::cast_slice};

pub struct Transform {
    position: cg::Vector3<f32>,
    rotation: cg::Quaternion<f32>,
    scale: f32,

    matrix: cg::Matrix4<f32>,
}

impl Transform {
    pub fn new(position: cg::Vector3<f32>, rotation: cg::Quaternion<f32>, scale: f32) -> Self {
        let matrix = calculate_transform_matrix(position, rotation, scale);
        
        Self {
            position,
            rotation,
            scale,
            matrix,
        }
    }
}

fn calculate_transform_matrix(position: cg::Vector3<f32>, rotation: cg::Quaternion<f32>, scale: f32) -> cg::Matrix4<f32> {
    cg::Matrix4::from_translation(position) *
    cg::Matrix4::from_scale(scale) *
    cg::Matrix4::from(rotation)
}

impl Default for Transform {
    fn default() -> Self {
        let position = cg::vec3(0.0, 0.0, 0.0);
        let rotation = cg::Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let scale = 1.0;

        let matrix = calculate_transform_matrix(position, rotation, scale);

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
            buffer,
            bind_group,
            context,
        }
    }
}