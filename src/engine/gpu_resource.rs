use std::{sync::Arc, rc::Rc, cell::RefCell};

use super::context::Context;

pub struct GpuResource<T> {
    pub data: Rc<RefCell<T>>,
    pub buffers: Vec<wgpu::Buffer>,
    pub bind_groups: Vec<wgpu::BindGroup>,

    pub context: Arc<Context>
}