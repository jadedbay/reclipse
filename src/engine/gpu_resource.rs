use std::sync::Arc;

use super::context::Context;

pub struct GpuResource<T> {
    pub data: T,
    pub buffers: Vec<wgpu::Buffer>,
    pub bind_groups: Vec<wgpu::BindGroup>,

    pub context: Arc<Context>
}