use crate::asset::handle::Handle;
use bevy_ecs::prelude::*;
use crate::asset;

#[derive(Component)]
pub struct Texture {
    handle: Handle<asset::Texture>
}

impl Texture {
    pub fn new(handle: Handle<asset::Texture>) -> Self {
        Self {
            handle
        }
    }
}