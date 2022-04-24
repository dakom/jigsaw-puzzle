use shipyard::*;
use crate::media::MediaView;
use crate::renderer::picker::InteractableLookupView;
use shipyard_scenegraph::prelude::*;
use nalgebra_glm::{Vec3, Vec2};
use crate::media::MediaPiece;
use crate::pieces::{PieceState, PiecesOrder};
use crate::animation::TweenPos;

#[derive(Component, PartialEq, Copy, Clone)]
pub enum Evaluate {
    Select,
    Release,
    FreeAnimFinished,
    LockAnimFinished
}

pub fn evaluate_sys(
    entities: EntitiesViewMut,
    media: MediaView,
    evaluates: View<Evaluate>,
    translations: View<Translation>,
    lookup: InteractableLookupView,
    mut pieces_order: UniqueViewMut<PiecesOrder>,
    mut tweens:ViewMut<TweenPos>,
    mut piece_states:ViewMut<PieceState>,
) {
    (&translations, &evaluates, &mut piece_states)
        .iter()
        .with_id()
        .for_each(|(entity, (translation, evaluate, mut piece_state))| {
            let get_index = || lookup.entity_to_index.get(&entity).map(|i| *i as usize);
            match evaluate {
                Evaluate::Select => {
                    if let Some(index) = get_index() {
                        pieces_order.remove(index);
                        pieces_order.push(entity);
                    }
                },
                Evaluate::Release => {
                    if let Some(index) = get_index() {
                        *piece_state = PieceState::Move;
                        let piece = &media.pieces[index];


                        let sqr_mag = nalgebra_glm::magnitude2(&(Vec2::new(translation.x, translation.y) - Vec2::new(piece.dest_x as f32, piece.dest_y as f32)));
                        if sqr_mag < (piece.width / 4.0).powi(2) && sqr_mag < (piece.height / 4.0).powi(2) {
                            *piece_state = PieceState::Locked;
                            let end = Vec3::new(piece.dest_x as f32, piece.dest_y as f32, translation.z);
                            let tween_pos = TweenPos::new(**translation, end, 0.001, Some(Evaluate::LockAnimFinished));
                            entities.add_component(entity, &mut tweens, tween_pos);
                        } else {
                            *piece_state = PieceState::Free;
                        }
                    }
                },
                Evaluate::FreeAnimFinished => {
                    *piece_state = PieceState::Free;
                },
                Evaluate::LockAnimFinished => {
                    *piece_state = PieceState::Locked;
                },
            }
        });
}

pub fn evaluate_clear_sys(
    mut evaluates: ViewMut<Evaluate>,
) {
    let all_ids:Vec<EntityId> = evaluates.iter().ids().collect();
    for id in all_ids {
        evaluates.delete(id);
    }
}
