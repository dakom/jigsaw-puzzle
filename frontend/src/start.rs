use std::rc::Rc;
use crate::{renderer::{picker::Interactable, RendererView}, animation::get_tween_start};
use nalgebra_glm::Vec3;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use crate::animation::TweenPos;
use std::ops::Deref;

pub fn start(world: Rc<World>) {

    world
        .run(move |
             renderer: RendererView,
            mut entities: EntitiesViewMut,
            interactables: View<Interactable>,
            translations: View<Translation>,
            mut tweens: ViewMut<TweenPos>,
        | {
            let (_, _, area_width, area_height) = renderer.get_viewport();

            for (entity, (translation, _)) in (&translations, &interactables).iter().with_id() {
                let tween_pos = get_tween_start(**translation, area_width as f64, area_height as f64);

                entities.add_component(entity, &mut tweens, tween_pos);
            }
        });
}
