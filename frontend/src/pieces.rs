use crate::{
    renderer::{item::*, RendererViewMut, spritesheet::{SpriteSheet, PuzzleSheet, SpriteCell}},
    media::*, camera::Camera,
};
use crate::prelude::*;
use derive_deref::{Deref, DerefMut};
use shipyard::*;
use nalgebra_glm::Vec3;
use awsm_web::{webgl::Id, dom::StyleExt};
use crate::prelude::*;


#[derive(Component, Default, Clone)]
pub struct PuzzleSize {
    pub width: f64, 
    pub height: f64, 
}


#[derive(Component, Deref, DerefMut)]
pub struct PiecesOrder (pub Vec<EntityId>);


pub fn create(world:&World, stage_width: f32, stage_height: f32) {

    // get the puzzle size based on put-together puzzle
    let puzzle_size = get_puzzle_size(&world.borrow::<MediaView>().unwrap_ext().pieces);

    // create the entities for each piece (cannot simultaneously add other components)
    let entity_ids:Vec<EntityId> = world
        .run(|
            mut media: MediaViewMut,
            mut sg: SceneGraphStoragesMut
        | {
            for piece in media.pieces.iter_mut() {
                //bottom-left origin
                piece.dest_y = puzzle_size.height - (piece.dest_y + piece.height);
            }

            media
                .pieces
                .iter()
                .map(|piece| {
                    let translation = Vec3::new(piece.dest_x as f32, piece.dest_y as f32, 1.0);
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

    // add the top-level uniques
    world.add_unique(puzzle_size);
    world.add_unique(PiecesOrder(entity_ids.clone()));

    let spritesheet = world.run(|media: MediaView, mut renderer: RendererViewMut, | {
        SpriteSheet {
            texture_id: renderer.create_img_texture(&media.spritesheet_img).unwrap_ext(),
            width: media.spritesheet_img.natural_width() as f32,
            height: media.spritesheet_img.natural_height() as f32,
        }
    });
    world.add_unique(PuzzleSheet(spritesheet));
    // add the pieces' components

    world
        .run(move |
            media: MediaView,
            puzzle_size: UniqueView<PuzzleSize>,
            puzzle_sheet: UniqueView<PuzzleSheet>,
            mut camera: UniqueViewMut<Camera>,
            mut entities: EntitiesViewMut,
            mut lookup: InteractableLookupViewMut,
            mut renderer: RendererViewMut, 
            mut sprite_cells: ViewMut<SpriteCell>, 
            mut interactables: ViewMut<Interactable>,
        | {

            camera.x = (puzzle_size.width as f64) / 2.0;
            camera.y = (puzzle_size.height as f64) / 2.0;

            for (index, piece) in media.pieces.iter().enumerate() {
                let entity = entity_ids[index];

                entities.add_component(entity, &mut sprite_cells, SpriteCell {
                    x: piece.src_x as f32,
                    y: piece.src_y as f32,
                    width: piece.width as f32, 
                    height: piece.height as f32,
                    uvs: get_uvs(&puzzle_sheet, &piece) 
                });
                entities.add_component(entity, &mut interactables, Interactable(index as u32));
                lookup.insert(index as u32, entity);
            }
        });
}

fn get_uvs(sheet: &SpriteSheet, piece: &MediaPiece) -> [f32;8] {

    let atlas_width = sheet.width as f64;
    let atlas_height = sheet.height as f64;

    //We need to invert the y axis, i.e. atlas_height - coords
    //the other calculations are just getting the relative displacement in the atlas
    let left = piece.src_x / atlas_width;
    let right = (piece.src_x + piece.width) / atlas_width;
    let top = (atlas_height - piece.src_y) / atlas_height;
    let bottom = (atlas_height - (piece.src_y + piece.height)) / atlas_height;

    //Get the corners, just for the sake of clarity
    //Might as well do the casting here too
    let tl = (left as f32, top as f32);
    let bl = (left as f32, bottom as f32);
    let tr = (right as f32, top as f32);
    let br = (right as f32, bottom as f32);
    [tl.0, tl.1, bl.0, bl.1, tr.0, tr.1, br.0, br.1]

}
fn get_puzzle_size(pieces: &[MediaPiece]) -> PuzzleSize {
    pieces
        .iter()
        .fold(PuzzleSize::default(), |mut full_size, piece| {
            //top-left origin
            let right = piece.dest_x + piece.width;
            let bottom = piece.dest_y + piece.height;

            if right > full_size.width {
                full_size.width = right;
            }

            if bottom > full_size.height {
                full_size.height = bottom;
            }

            full_size

        })
}
