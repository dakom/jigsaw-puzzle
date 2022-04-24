use std::ops::Mul;

use awsm_web::webgl::{BufferData, BufferTarget, BufferUsage};
use nalgebra_glm::{self as glm, Vec3, Mat4, Vec2};
use nalgebra::geometry::Point3;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use awsm_web::prelude::*;
use crate::{media::{MediaPiece, PuzzleInfo, Media, MediaView}, renderer::{SceneRenderer, picker::{InteractableLookup, Interactable}, RendererViewMut}};
#[derive(Component, Default)]
pub struct DataBuffers {
    pub has_set_outline: bool,
    pub geom_vertices: Vec<f32>,
    pub outline_vertices: Vec<f32>,
    pub tex_vertices: Vec<f32>,
    pub picker_color_vertices: Vec<f32>,
}

pub fn transform_buffer_piece(buffer: &mut[f32], index: usize, piece:&MediaPiece, m:&Mat4, z_override: Option<f32>) {

    let orig = &piece.vertices;

    for i in 0..6 {
        let pi = i * 3;
        let pt = Point3::new(orig[pi], orig[pi +1], orig[pi+2]);
        let v = m.transform_point(&pt);
        let buffer_start_index = (index * 18) + pi;
        buffer[buffer_start_index] = v.x;
        buffer[buffer_start_index + 1] = v.y;
        match z_override {
            Some(z) => {
                buffer[buffer_start_index + 2] = z;
            }
            None => {
                buffer[buffer_start_index + 2] = v.z;
            }
        }
    }
}
impl DataBuffers {
    pub fn move_piece(&mut self, index: usize, piece:&MediaPiece, m:&Mat4) {
        transform_buffer_piece(&mut self.geom_vertices, index, piece, m, None); 
        if !self.has_set_outline {
            transform_buffer_piece(&mut self.outline_vertices, index, piece, m, Some(0.0)); 
        }
    }


    pub fn add_piece(&mut self, puzzle_info:&PuzzleInfo, piece:&MediaPiece, index: usize) {

        let get_uvs = || -> [f32;12] {
            //We need to invert the y axis, i.e. atlas_height - coords
            //the other calculations are just getting the relative displacement in the atlas
            let atlas_width = puzzle_info.atlas_width as f32;
            let atlas_height = puzzle_info.atlas_height as f32;

            let left = piece.src_x / atlas_width;
            let right = (piece.src_x + piece.width) / atlas_width;
            let top = (atlas_height - piece.src_y) / atlas_height;
            let bottom = (atlas_height - (piece.src_y + piece.height)) / atlas_height;

            [
                left,bottom,
                left,top,
                right,bottom,
                right,bottom,
                left,top,
                right,top
            ]
        };

        let get_picker_colors = || -> [f32;24] {
            let index = index + 1;
            let divisor = 0xFF as f32;
            let r = (0xFF & (index >> 16)) as f32 / divisor;
            let g = (0xFF & (index >> 8)) as f32 / divisor;
            let b = (0xFF & index) as f32 / divisor; 
            let a = 1.0;

            [
                r,g,b,a,
                r,g,b,a,
                r,g,b,a,
                r,g,b,a,
                r,g,b,a,
                r,g,b,a,
            ]

        };

        self.geom_vertices.extend(piece.vertices);
        self.outline_vertices.extend(piece.vertices);
        self.tex_vertices.extend(get_uvs());
        self.picker_color_vertices.extend(get_picker_colors());
    }

    pub fn flush_model(&mut self, renderer:&mut SceneRenderer) {
        if !self.has_set_outline {
            renderer.upload_buffer(renderer.outline_buffer_id,
                BufferData::new(&self.outline_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
            ).unwrap_ext();
        }

        renderer.upload_buffer(renderer.geom_buffer_id,
            BufferData::new(&self.geom_vertices, BufferTarget::ArrayBuffer, BufferUsage::DynamicDraw)
        ).unwrap_ext();
    }

    pub fn flush_static(&mut self, renderer:&mut SceneRenderer) {



        renderer.upload_buffer(renderer.tex_buffer_id,
            BufferData::new(&self.tex_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
        ).unwrap_ext();

        renderer.upload_buffer(renderer.color_buffer_id,
            BufferData::new(&self.picker_color_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
        ).unwrap_ext();
    }
}

pub fn update_buffers_sys(
    mut buffers: UniqueViewMut<DataBuffers>,
    mut renderer: RendererViewMut,
    media: MediaView,
    lookup: UniqueView<InteractableLookup>,
    translations: View<Translation>,
    world_transform: View<WorldTransform>,
    interactables: View<Interactable>,
) {

    let mut changed = false;

    world_transform
        .modified()
        .iter()
        .with_id()
        .for_each(|(entity, m)| {
            if let Some(index) = lookup.entity_to_index.get(&entity) {
                buffers.move_piece(*index as usize, &media.pieces[*index as usize], &m);
                changed = true;
            }
        });

    if changed {
        buffers.flush_model(&mut renderer);
        buffers.has_set_outline = true;
    }

}
