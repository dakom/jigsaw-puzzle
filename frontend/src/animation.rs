use nalgebra_glm::Vec3;
use shipyard::*;
use shipyard::Delete;
use crate::{media::{MediaView, MediaPiece}, pieces::PiecesOrder, evaluate::Evaluate};
use shipyard_scenegraph::prelude::*;
use crate::mainloop::UpdateTick;

#[derive(Component)]
pub struct TweenPos {
    pub perc: f64,
    pub start: Vec3,
    pub end: Vec3,
    pub speed: f64,
    pub evaluate_on_end: Option<Evaluate>
}

impl TweenPos {
    pub fn new(start: Vec3, end: Vec3, speed: f64, evaluate_on_end: Option<Evaluate>) -> Self {
        Self {
            perc: 0.0,
            start,
            end,
            speed,
            evaluate_on_end
        }
    }

    pub fn finished(&self) -> bool {
        self.perc >= 1.0
    }
}


pub fn animation_update_sys(
    tick: UniqueView<UpdateTick>,
    mut tweens: ViewMut<TweenPos>,
    mut translations: ViewMut<Translation>
) {
    (&mut translations, &mut tweens)
        .iter()
        .for_each(|(mut translation, mut tween)| {
            tween.perc = (tween.perc + (tween.speed * tick.delta)).min(1.0);
            translation.copy_from(&tween.start.lerp(&tween.end, tween.perc as f32));
        });

}

pub fn animation_end_sys(
    tweens: View<TweenPos>,
    pieces_order: UniqueView<PiecesOrder>,
    entities: EntitiesViewMut,
    mut evaluates: ViewMut<Evaluate>
) {
    (&tweens)
        .iter()
        .with_id()
        .for_each(|(entity, tween)| {
            if tween.finished() {
                if let Some(evaluate) = tween.evaluate_on_end {
                    entities.add_component(entity, &mut evaluates, evaluate);
                }

            }
        });


}

pub fn animation_clear_sys(
    mut tweens: ViewMut<TweenPos>,
) {
    let ids_to_delete:Vec<EntityId> = (&tweens)
        .iter()
        .with_id()
        .filter(|(_, tween)| tween.finished())
        .map(|(entity, _)| entity)
        .collect();

    for id in ids_to_delete {
        (&mut tweens).delete(id);
    }
}
