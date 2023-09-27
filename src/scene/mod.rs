use crate::objects::sprite::Sprite;

pub struct Scene {
    //player,
    pub sprites: Vec<Sprite>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }
}