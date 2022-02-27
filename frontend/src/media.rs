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
    pub vertex_shader: String,
    pub picker_fragment_shader: String,
    pub texture_fragment_shader: String,
    pub pieces: Vec<MediaPiece>,
    pub spritesheet_img: HtmlImageElement
}


impl Media {
    pub async fn load(dom: &Dom) -> Self {

        let vertex_shader = fetch_url(&get_media_href("vertex.glsl")).await.unwrap_ext().text().await.unwrap_ext();
        let picker_fragment_shader = fetch_url(&get_media_href("picker-fragment.glsl")).await.unwrap_ext().text().await.unwrap_ext();
        let texture_fragment_shader = fetch_url(&get_media_href("texture-fragment.glsl")).await.unwrap_ext().text().await.unwrap_ext();


        let spritesheet_img = loaders::image::load(get_puzzle_href("spritesheet.png")).await.unwrap_ext();
        let spritesheet_json:PiecesJson = fetch_url(&get_puzzle_href("spritesheet.json")).await.unwrap_ext().json_from_str().await.unwrap_ext();
        let dest_pieces:PiecesJson = fetch_url(&get_puzzle_href("pieces.json")).await.unwrap_ext().json_from_str().await.unwrap_ext();

        let mut pieces = Vec::new();

        for (id, (src_x, src_y, width, height)) in spritesheet_json.into_iter() {
            let (dest_x, dest_y, _, _) = *dest_pieces.get(&id).unwrap_ext();

            pieces.push(MediaPiece { 
                id,
                src_x,
                src_y,
                dest_x,
                dest_y,
                width,
                height
            });
        }

        Self {
            vertex_shader,
            picker_fragment_shader,
            texture_fragment_shader,
            pieces,
            spritesheet_img
        }
    }
}


type PiecesJson = HashMap<String, (f64, f64, f64, f64)>;


#[derive(Debug)]
pub struct MediaPiece {
    pub id: String,
    pub src_x: f64,
    pub src_y: f64,
    pub dest_x: f64,
    pub dest_y: f64,
    pub width: f64,
    pub height: f64,
}
