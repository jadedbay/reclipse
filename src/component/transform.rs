use std::{sync::Arc, rc::Rc, cell::RefCell};

use wgpu::util::DeviceExt;

use crate::{engine::{gpu_resource::GpuResource, context::Context, renderer::Renderer}, util::cast_slice};

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Transform {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: f32,

    matrix: glam::Mat4,
}

impl Transform {
    pub fn new(position: glam::Vec3, rotation: glam::Vec3, scale: f32) -> Self {
        let rotation = glam::Quat::from_euler(glam::EulerRot::XYZ, rotation.x.to_radians(), rotation.y.to_radians(), rotation.z.to_radians());
        let matrix = glam::Mat4::from_scale_rotation_translation(glam::Vec3::splat(scale), rotation, position);
        
        Self {
            position,
            rotation,
            scale,
            matrix,
        }
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
        self.dirty = true;
    }

    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }
}

impl Default for Transform {
    fn default() -> Self {
        let position = glam::Vec3::ZERO;
        let rotation = glam::Quat::IDENTITY;
        let scale = 1.0;

        let matrix = glam::Mat4::from_scale_rotation_translation(glam::Vec3::splat(scale), rotation, position);

        Self {
            position,
            rotation,
            scale,
            matrix,
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
            data: Rc::new(RefCell::new(transform)),
            buffers: vec![buffer],
            bind_groups: vec![bind_group],
            context,
        }
    }

    pub fn update_buffer(&self) {
        self.context.queue.write_buffer(&self.buffers[0], 0, cast_slice(&[self.data.borrow().matrix]));
        self.data.borrow_mut().dirty = false;
    }
}