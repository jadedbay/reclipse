use std::sync::Arc;

use crate::engine::context::Context;

pub mod quad;

pub enum PrimitiveMesh {
    Quad,
}

pub struct Primitives {
    context: Arc<Context>,
}

impl Primitives {
    pub fn new(context: Arc<Context>) -> Self {
        Self {
            context,
        }
    }

    pub fn create
}