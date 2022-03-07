use crate::{
    renderer::{picker::*, RendererViewMut, spritesheet::SpriteSheetTextureId},
    media::*, camera::{Camera, DEPTH_OFFSET}, buffers::DataBuffers,
};
use crate::prelude::*;
use derive_deref::{Deref, DerefMut};
use shipyard::*;
use nalgebra_glm::Vec3;
use awsm_web::{webgl::Id, dom::StyleExt};
use web_sys::HtmlImageElement;
use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct PiecesOrder (pub Vec<EntityId>);

fn get_z(index: usize) -> f32 {
    (index as f32 * DEPTH_OFFSET)
}
pub fn create(world:&World, stage_width: f32, stage_height: f32) {


    // create the entities for each piece (cannot simultaneously add other components)
    let entity_ids:Vec<EntityId> = world
        .run(|
            mut media: MediaViewMut,
            mut sg: SceneGraphStoragesMut
        | {

            media
                .pieces
                .iter()
                .enumerate()
                .map(|(index, piece)| {
                    let translation = Vec3::new(piece.dest_x as f32, piece.dest_y as f32, get_z(index));
                    sg.spawn_child_trs_origin(
                        None,
                        Some(translation),
                        None,
                        None,
                        None,
                    )
                })
                .collect()
        });


    let spritesheet = world.run(|media: MediaView, mut renderer: RendererViewMut, | {
        SpriteSheetTextureId(renderer.create_img_texture(&media.puzzle_img).unwrap_ext())
    });
    world.add_unique(spritesheet);
    world.add_unique(PiecesOrder(entity_ids.clone()));
    // add the pieces' components

    world
        .run(move |
            media: MediaView,
            mut camera: UniqueViewMut<Camera>,
            mut data_buffers: UniqueViewMut<DataBuffers>,
            mut entities: EntitiesViewMut,
            mut lookup: InteractableLookupViewMut,
            mut renderer: RendererViewMut, 
            mut interactables: ViewMut<Interactable>,
        | {

            let puzzle_info = &media.puzzle_info;

            camera.x = (puzzle_info.puzzle_width as f64) / 2.0;
            camera.y = (puzzle_info.puzzle_height as f64) / 2.0;

            for (index, piece) in media.pieces.iter().enumerate() {
                let entity = entity_ids[index];

                entities.add_component(entity, &mut interactables, Interactable(index as u32));
                lookup.index_to_entity.insert(index as u32, entity);
                lookup.entity_to_index.insert(entity, index as u32);

                data_buffers.add_piece(puzzle_info, piece, index);
            }


            data_buffers.flush_model(&mut renderer);
            data_buffers.flush_static(&mut renderer);
        });
}

pub fn pieces_order_sys(
    pieces_order: UniqueView<PiecesOrder>,
    mut translations: ViewMut<Translation>
) {
    if pieces_order.is_modified() {
        for (index, entity) in pieces_order.iter().enumerate() {
            (&mut translations).get(*entity).unwrap().z = get_z(index);
        }
    }
}
