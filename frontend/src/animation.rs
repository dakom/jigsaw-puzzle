use nalgebra_glm::Vec3;
use shipyard::*;
use shipyard::Delete;
use crate::media::{MediaView, MediaPiece};
use shipyard_scenegraph::prelude::*;
use crate::mainloop::UpdateTick;

#[derive(Component)]
pub struct TweenPos {
    pub perc: f64,
    pub start: Vec3,
    pub end: Vec3,
    pub speed: f64,
}

impl TweenPos {
    pub fn new(start: Vec3, end: Vec3, speed: f64) -> Self {
        Self {
            perc: 0.0,
            start,
            end,
            speed,
        }
    }
}


pub fn animation_sys(
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

pub fn animation_clear_sys(
    mut tweens: ViewMut<TweenPos>,
) {
    let ids_to_delete:Vec<EntityId> = (&tweens)
        .iter()
        .with_id()
        .filter(|(_, tween)| if tween.perc >= 1.0 { true } else { false } )
        .map(|(entity, _)| entity)
        .collect();

    for id in ids_to_delete {
        (&mut tweens).delete(id);
    }
}

pub fn get_tween_start(start: Vec3, area_width: f64, area_height: f64) -> TweenPos {
    let x = js_sys::Math::random() * area_width;
    let y = js_sys::Math::random() * area_height;
    let end = Vec3::new(x as f32, y as f32, start.z);
    TweenPos::new(start, end, 0.001)
}


pub fn get_tween_end(start: Vec3, piece: &MediaPiece) -> TweenPos {
    let end = Vec3::new(piece.dest_x as f32, piece.dest_y as f32, start.z);
    TweenPos::new(start, end, 0.001)
}
