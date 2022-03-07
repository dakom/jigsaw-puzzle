use awsm_web::webgl::Id;
use derive_deref::{Deref, DerefMut};
use shipyard::Component;

#[derive(Component, Deref, DerefMut)]
pub struct SpriteSheetTextureId(pub Id);

