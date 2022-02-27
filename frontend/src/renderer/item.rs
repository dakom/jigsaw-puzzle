use std::collections::HashMap;
use shipyard::*;
use derive_deref::{Deref, DerefMut};

pub type InteractableLookupView<'a> = UniqueView<'a, InteractableLookup>;
pub type InteractableLookupViewMut<'a> = UniqueViewMut<'a, InteractableLookup>;
#[derive(Component, Deref, DerefMut)]
pub struct InteractableLookup(pub HashMap<u32, EntityId>);

#[derive(Component)]
pub struct Interactable(pub u32); // the entity id lookup

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
