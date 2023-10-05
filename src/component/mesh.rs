use crate::asset::handle::Handle;
use bevy_ecs::prelude::*;
use crate::asset;

#[derive(Component)]
pub struct Mesh {
    handle: Handle<asset::Mesh>
}

impl Mesh {
    pub fn new(handle: Handle<asset::Mesh>) -> Self {
        Self {
            handle
        }
    }
}