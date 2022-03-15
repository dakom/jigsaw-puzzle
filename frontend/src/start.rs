use std::rc::Rc;
use crate::{renderer::{picker::Interactable, RendererView}, media::MediaView, evaluate::Evaluate, pieces::PieceState};
use nalgebra_glm::Vec3;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use crate::animation::TweenPos;
use std::ops::Deref;

pub fn start(world: Rc<World>) {

    world
        .run(move |
             renderer: RendererView,
             media: MediaView,
            mut entities: EntitiesViewMut,
            interactables: View<Interactable>,
            translations: View<Translation>,
            piece_states: View<PieceState>,
            mut tweens: ViewMut<TweenPos>,
        | {
            let bounds = media.puzzle_info.get_bounds();
            let max_piece_area = media.puzzle_info.get_max_piece_area();

            for (entity, (translation, _, _)) in (&translations, &interactables, &piece_states).iter().with_id() {
                let tween_pos = get_tween_start(**translation, bounds, max_piece_area);

                entities.add_component(entity, &mut tweens, tween_pos);
            }
        });
}

fn get_tween_start(start: Vec3, bounds: (u32, u32, u32, u32), max_piece_area: (u32, u32)) -> TweenPos {
    let (left, bottom, right, top) = (bounds.0 as f64, bounds.1 as f64, bounds.2 as f64, bounds.3 as f64);
    let (piece_width, piece_height) = (max_piece_area.0 as f64, max_piece_area.1 as f64);

    let (x, y) = match rand_side() {
        Side::Left => {
            (
                rand_between(left - (piece_width * 2.0), left - (piece_width * 1.5)),
                rand_between(bottom - (piece_height * 2.0), top + (piece_height * 2.0))
            )
        },
        Side::Right=> {
            (
                rand_between(right + (piece_width * 1.5), right + (piece_width * 2.0)),
                rand_between(bottom - (piece_height * 2.0), top + (piece_height * 2.0))
            )
        },
        Side::Top => {
            (
                rand_between(left - (piece_width * 2.0), right + (piece_width * 2.0)),
                rand_between(top + (piece_height * 1.5), top + (piece_height * 2.0))
            )
        },
        Side::Bottom => {
            (
                rand_between(left - (piece_width * 2.0), right + (piece_width * 2.0)),
                rand_between(bottom - (piece_height * 2.0), bottom - (piece_height * 1.5))
            )
        },
    };

    let end = Vec3::new(x as f32, y as f32, start.z);
    TweenPos::new(start, end, 0.001, Some(Evaluate::StartAnimFinished))
}

#[repr(u8)]
enum Side {
    Left,
    Top,
    Bottom,
    Right
}

fn rand_side() -> Side {
    unsafe {std::mem::transmute(rand_between(0.0,4.0).floor() as u8) }
}

fn rand_bool() -> bool {
    js_sys::Math::random() > 0.5
}

// min is inclusive, max exclusive
fn rand_between(min:f64, max:f64) -> f64 {
  js_sys::Math::random() * (max - min) + min
}

