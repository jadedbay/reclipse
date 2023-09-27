use std::sync::Arc;

use super::context::Context;

pub struct GpuResource<T> {
    pub data: T,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,

    pub context: Arc<Context>
}