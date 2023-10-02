use wgpu::util::DeviceExt;

use std::f32::consts::FRAC_PI_2;

use instant::Duration;

use crate::util::cast_slice;

use crate::engine::input::{InputState, Key};

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;


pub struct Camera {
    pub position: glam::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub projection: Projection,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device, 
        camera_bind_group_layout: &wgpu::BindGroupLayout, 
        position: glam::Vec3, 
        yaw: f32, 
        pitch: f32, 
        projection: Projection
    ) -> Self {
        let camera_uniform = CameraUniform::new();

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ],
            label: Some("camera_bind_group")
        });

        Self {
            position,
            yaw: yaw.to_radians(),
            pitch: pitch.to_radians(),
            projection,
            uniform: camera_uniform,
            buffer: camera_buffer,
            bind_group: camera_bind_group,
        }
    }

    pub fn update_uniform(&mut self) {
        let view_proj = (self.projection.calc_matrix() * self.calc_matrix()).into();
        self.uniform.update(&self.position, view_proj);
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        glam::Mat4::look_to_rh(
            self.position,
            glam::Vec3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            glam::Vec3::Y,
        )
    }
}

pub struct Projection {
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.to_radians(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraUniform {
    pub view_position: glam::Vec4,
    pub view_proj: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: glam::Vec4::new(0.0, 0.0, 0.0, 0.0),
            view_proj: glam::Mat4::IDENTITY,
        }
    }

    pub fn update(&mut self, eye: &glam::Vec3, view_proj: glam::Mat4) {
        self.view_position = eye.extend(1.0);
        self.view_proj = view_proj;
    }
}

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn movement(&mut self, input: &InputState) {
        if input.key_down(Key::W) { self.amount_forward = 1.0 } else { self.amount_forward = 0.0 }
        if input.key_down(Key::S) { self.amount_backward = 1.0 } else { self.amount_backward = 0.0 }
        if input.key_down(Key::A) { self.amount_left = 1.0 } else { self.amount_left = 0.0 }
        if input.key_down(Key::D) { self.amount_right = 1.0 } else { self.amount_right = 0.0 }
        if input.key_down(Key::Space) { self.amount_up = 1.0 } else { self.amount_up = 0.0 }
        if input.key_down(Key::LShift) { self.amount_down = 1.0 } else { self.amount_down = 0.0 }
    }

    pub fn process_mouse(&mut self, input: &InputState) {
        self.rotate_horizontal = input.cursor_delta.x as f32;
        self.rotate_vertical = input.cursor_delta.y as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration, input: &InputState) {
        self.movement(input);
        if input.right_button_down() {
            self.process_mouse(input);
        }
        
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = glam::Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        camera.yaw += self.rotate_horizontal * self.sensitivity * dt;
        camera.pitch += -self.rotate_vertical * self.sensitivity * dt;

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        if camera.pitch < -SAFE_FRAC_PI_2 {
            camera.pitch = -SAFE_FRAC_PI_2;
        } else if camera.pitch > SAFE_FRAC_PI_2 {
            camera.pitch = SAFE_FRAC_PI_2;
        }
    }
}
