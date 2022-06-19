use awsm_web::loaders::{self, fetch::fetch_url};
use std::collections::HashMap;
use crate::config::{get_media_href, get_puzzle_href};
use serde::Deserialize;
use shipyard::*;
use web_sys::HtmlImageElement;
use crate::prelude::*;

pub type MediaViewMut<'a> = NonSendSync<UniqueViewMut<'a, Media>>;
pub type MediaView<'a> = NonSendSync<UniqueView<'a, Media>>;

#[derive(Component, Unique)]
pub struct Media {
    pub picker_vertex_shader: String,
    pub picker_fragment_shader: String,
    pub piece_fragment_shader: String,
    pub piece_vertex_shader: String,
    pub outline_vertex_shader: String,
    pub outline_fragment_shader: String,
    pub quad_fragment_shader: String,
    pub quad_vertex_shader: String,
    pub pieces: Vec<MediaPiece>,
    pub puzzle_info: PuzzleInfo,
    pub puzzle_img: HtmlImageElement
}


impl Media {
    pub async fn load() -> Self {
        let puzzle_img = loaders::image::load(get_puzzle_href("puzzle.png")).await.unwrap_ext();
        let puzzle_info:PuzzleInfo = fetch_url(&get_puzzle_href("puzzle.json")).await.unwrap_ext().json_from_str().await.unwrap_ext();

        let mut pieces = Vec::new();

        for (index, (src_x, src_y, dest_x, dest_y, img_width, img_height)) in puzzle_info.images.iter().enumerate() {
            pieces.push(MediaPiece { 
                id: index as u32,
                src_x: *src_x as f32,
                src_y: *src_y as f32,
                width: *img_width as f32,
                height: *img_height as f32,
                dest_x: *dest_x as f64,
                dest_y: *dest_y as f64,
                vertices: crate::buffers::make_vertices(0.0, 0.0, 0.0, *img_width as f32, *img_height as f32)
            });
        }

        let piece_vertex = include_str!("./shaders/piece-vertex.glsl");
        let piece_fragment = include_str!("./shaders/piece-fragment.glsl");
        let quad_vertex = include_str!("./shaders/quad-vertex.glsl");
        let quad_fragment = include_str!("./shaders/quad-fragment.glsl");

        Self {
            picker_vertex_shader: format!("#define PICKER\n{}", piece_vertex), 
            picker_fragment_shader: format!("#define PICKER\n{}", piece_fragment), 
            outline_vertex_shader: format!("#define OUTLINE\n{}", piece_vertex), 
            outline_fragment_shader: format!("#define OUTLINE\n{}", piece_fragment), 
            piece_vertex_shader: piece_vertex.to_string(), 
            piece_fragment_shader: piece_fragment.to_string(), 
            quad_vertex_shader: quad_vertex.to_string(), 
            quad_fragment_shader: quad_fragment.to_string(), 
            pieces,
            puzzle_img,
            puzzle_info,
        }
    }
}

#[derive(Deserialize)]
pub struct PuzzleInfo {
    pub puzzle_width: u32,
    pub puzzle_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    //src x,y, dest x,y and bitmap w,h
    pub images: Vec<(u32, u32, u32, u32, u32, u32)>
}

impl PuzzleInfo {
    // left bottom right top
    // not the actual bounds really
    // but the coordinates for pieces at these bounds
    pub fn get_bounds(&self) -> (u32, u32, u32, u32) {
        self.images
            .iter()
            .fold((u32::MAX, u32::MAX, 0, 0), |(mut left, mut bottom, mut right, mut top), image| {
                let piece_x = image.2;
                let piece_y = image.3;

                if piece_x < left {
                    left = piece_x;
                }

                if piece_y < bottom {
                    bottom = piece_y;
                }

                if piece_x > right {
                    right = piece_x;
                }

                if piece_y > top {
                    top = piece_y;
                }

                (left, bottom, right, top) 
            })

    }
    // but the coordinates for pieces at these bounds
    pub fn get_max_piece_area(&self) -> (u32, u32) {
        self.images
            .iter()
            .fold((0, 0), |(mut width, mut height), image| {
                let piece_w = image.4;
                let piece_h = image.5;

                if piece_w > width {
                    width = piece_w;
                }
                if piece_h > height {
                    height = piece_h;
                }


                (width, height) 
            })

    }
}


#[derive(Debug)]
pub struct MediaPiece {
    pub id: u32,
    //x,y within the texture
    pub src_x: f32,
    pub src_y: f32,
    //width,height within the texture
    pub width: f32,
    pub height: f32,
    //x,y destination on the canvas
    pub dest_x: f64,
    pub dest_y: f64,
    pub vertices: [f32;18] 
}

