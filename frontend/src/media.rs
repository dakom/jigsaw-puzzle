use awsm_web::loaders::{self, fetch::fetch_url};
use std::collections::HashMap;
use crate::config::{get_media_href, get_puzzle_href};
use serde::Deserialize;
use shipyard::*;
use web_sys::HtmlImageElement;
use crate::prelude::*;
use crate::dom::Dom;

pub type MediaViewMut<'a> = NonSendSync<UniqueViewMut<'a, Media>>;
pub type MediaView<'a> = NonSendSync<UniqueView<'a, Media>>;

#[derive(Component)]
pub struct Media {
    pub picker_vertex_shader: &'static str,
    pub picker_fragment_shader: &'static str,
    pub forward_fragment_shader: &'static str,
    pub forward_vertex_shader: &'static str,
    pub outline_fragment_shader: &'static str,
    pub outline_vertex_shader: &'static str,
    pub pieces: Vec<MediaPiece>,
    pub puzzle_info: PuzzleInfo,
    pub puzzle_img: HtmlImageElement
}


impl Media {
    pub async fn load(dom: &Dom) -> Self {
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
                vertices: make_vertices(*img_width as f32, *img_height as f32)
            });
        }

        Self {
            picker_vertex_shader: include_str!("./shaders/picker-vertex.glsl"),
            picker_fragment_shader: include_str!("./shaders/picker-fragment.glsl"),
            forward_vertex_shader: include_str!("./shaders/forward-vertex.glsl"),
            forward_fragment_shader: include_str!("./shaders/forward-fragment.glsl"),
            outline_vertex_shader: include_str!("./shaders/outline-vertex.glsl"),
            outline_fragment_shader: include_str!("./shaders/outline-fragment.glsl"),
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

fn make_vertices(w: f32, h: f32) -> [f32;18] {
    [
        //TRIANGLE 1
        //left-bottom
        0.0, 0.0,0.0,
        //left-top
        0.0, h,0.0,
        //right-bottom
        w, 0.0,0.0,
        //TRIANGLE 2
        //right-bottom
        w, 0.0,0.0,
        //left-top
        0.0, h,0.0,
        //right-top
        w, h, 0.0
    ]
}
