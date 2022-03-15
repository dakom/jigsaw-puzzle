use shipyard::*;
use crate::media::MediaView;
use crate::renderer::picker::InteractableLookupView;
use shipyard_scenegraph::prelude::*;
use nalgebra_glm::Vec3;
use crate::media::MediaPiece;
use crate::pieces::{PieceState, PiecesOrder};
use crate::animation::TweenPos;

#[derive(Component, PartialEq, Copy, Clone)]
pub enum Evaluate {
    Select,
    Release,
    StartAnimFinished,
    EndAnimFinished
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
                        log::info!("release: {} {:?}", index, entity);
                        let piece = &media.pieces[index];

                        let end = Vec3::new(piece.dest_x as f32, piece.dest_y as f32, translation.z);
                        let tween_pos = TweenPos::new(**translation, end, 0.001, Some(Evaluate::EndAnimFinished));
                        entities.add_component(entity, &mut tweens, tween_pos);
                    }
                },
                Evaluate::StartAnimFinished => {
                    *piece_state = PieceState::Free;
                },
                Evaluate::EndAnimFinished => {
                    *piece_state = PieceState::Locked;
                    if let Some(index) = get_index() {
                        pieces_order.remove(index);
                        pieces_order.push(entity);
                    }
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
