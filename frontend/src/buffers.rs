use std::ops::Mul;

use awsm_web::webgl::{BufferData, BufferTarget, BufferUsage};
use nalgebra_glm::{self as glm, Vec3, Mat4, Vec2};
use nalgebra::geometry::Point3;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use awsm_web::prelude::*;
use crate::{media::{MediaPiece, PuzzleInfo, Media, MediaView}, renderer::{SceneRenderer, picker::{InteractableLookup, Interactable}, RendererViewMut}, camera::Z_DEPTH};
#[derive(Component, Unique, Default)]
pub struct DataBuffers {
    pub has_set_bg: bool,
    pub piece_active_vertices: Vec<f32>,
    pub piece_bg_vertices: Vec<f32>,
    pub tex_vertices: Vec<f32>,
    pub picker_color_vertices: Vec<f32>,
    pub border_vertices: Vec<f32>,
}

const BG_DEPTH:f32 = -Z_DEPTH + 2.0;

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
        transform_buffer_piece(&mut self.piece_active_vertices, index, piece, m, None); 
        if !self.has_set_bg {
            transform_buffer_piece(&mut self.piece_bg_vertices, index, piece, m, Some(BG_DEPTH)); 
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

        self.piece_active_vertices.extend(piece.vertices);
        self.piece_bg_vertices.extend(piece.vertices);
        self.tex_vertices.extend(get_uvs());
        self.picker_color_vertices.extend(get_picker_colors());
    }

    pub fn flush_pieces(&mut self, renderer:&mut SceneRenderer, media: &Media) {
        if !self.has_set_bg {
            renderer.upload_buffer(renderer.buffers.piece_bg,
                BufferData::new(&self.piece_bg_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
            ).unwrap_ext();

            self.set_border(renderer, media);
        }

        renderer.upload_buffer(renderer.buffers.piece_active,
            BufferData::new(&self.piece_active_vertices, BufferTarget::ArrayBuffer, BufferUsage::DynamicDraw)
        ).unwrap_ext();
    }

    pub fn flush_predefined(&mut self, renderer:&mut SceneRenderer) {
        renderer.upload_buffer(renderer.buffers.texture,
            BufferData::new(&self.tex_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
        ).unwrap_ext();

        renderer.upload_buffer(renderer.buffers.picker_color,
            BufferData::new(&self.picker_color_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
        ).unwrap_ext();
    }

    fn set_border(&mut self, renderer:&mut SceneRenderer, media: &Media) {
        let size = 10.0;
        let dsize = size * 2.0;
        let width = media.puzzle_info.puzzle_width as f32;
        let height = media.puzzle_info.puzzle_height as f32;


        let mut vertices_left = make_vertices(-size, -size, BG_DEPTH, size, height + dsize).to_vec();
        let mut vertices_right = make_vertices(width, -size, BG_DEPTH, size, height + dsize).to_vec();
        let mut vertices_top = make_vertices(-size, -size, BG_DEPTH, width + dsize, size).to_vec();
        let mut vertices_bottom = make_vertices(-size, height, BG_DEPTH, width + dsize, size).to_vec();

        self.border_vertices = Vec::with_capacity(vertices_left.len() * 4);
        self.border_vertices.extend(vertices_left.into_iter());
        self.border_vertices.extend(vertices_right.into_iter());
        self.border_vertices.extend(vertices_top.into_iter());
        self.border_vertices.extend(vertices_bottom.into_iter());

        renderer.upload_buffer(renderer.buffers.border,
            BufferData::new(&self.border_vertices, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)
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
        buffers.flush_pieces(&mut renderer, &media);
        buffers.has_set_bg = true;
    }

}

pub fn make_vertices(x: f32, y: f32, z: f32, w: f32, h: f32) -> [f32;18] {
    [
        //TRIANGLE 1
        //left-bottom
        x,y,z,
        //left-top
        x, y+h,z,
        //right-bottom
        x+w, y,z,
        //TRIANGLE 2
        //right-bottom
        x+w, y,z,
        //left-top
        x, y+h,z,
        //right-top
        x+w, y+h, z 
    ]
}
