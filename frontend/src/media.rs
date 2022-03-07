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
    pub picker_vertex_shader: String,
    pub picker_fragment_shader: String,
    pub forward_fragment_shader: String,
    pub forward_vertex_shader: String,
    pub pieces: Vec<MediaPiece>,
    pub puzzle_info: PuzzleInfo,
    pub puzzle_img: HtmlImageElement
}


impl Media {
    pub async fn load(dom: &Dom) -> Self {

        let forward_vertex_shader = fetch_url(&get_media_href("forward-vertex.glsl")).await.unwrap_ext().text().await.unwrap_ext();
        let forward_fragment_shader = fetch_url(&get_media_href("forward-fragment.glsl")).await.unwrap_ext().text().await.unwrap_ext();
        let picker_vertex_shader = fetch_url(&get_media_href("picker-vertex.glsl")).await.unwrap_ext().text().await.unwrap_ext();
        let picker_fragment_shader = fetch_url(&get_media_href("picker-fragment.glsl")).await.unwrap_ext().text().await.unwrap_ext();

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
            picker_vertex_shader,
            picker_fragment_shader,
            forward_fragment_shader,
            forward_vertex_shader,
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
