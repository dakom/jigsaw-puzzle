use awsm_web::webgl::Id;
use derive_deref::{Deref, DerefMut};
use shipyard::{Component, Unique};

#[derive(Component, Unique, Deref, DerefMut)]
pub struct SpriteSheetTextureId(pub Id);

