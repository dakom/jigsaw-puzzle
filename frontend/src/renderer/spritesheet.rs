use awsm_web::webgl::Id;
use derive_deref::{Deref, DerefMut};
use shipyard::Component;

#[derive(Component, Deref, DerefMut)]
pub struct PuzzleSheet(pub SpriteSheet);

pub struct SpriteSheet {
    pub texture_id: Id,
    pub width: f32,
    pub height: f32
}

#[derive(Component, Clone)]
pub struct SpriteCell {
    pub x: f32, 
    pub y: f32, 
    pub width: f32, 
    pub height: f32,
    pub uvs: [f32;8],
}


