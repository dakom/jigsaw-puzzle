#![allow(warnings)]
mod config;
mod world;
mod renderer;
mod setup;
mod media;
mod dom;
mod mainloop;
mod controller;
mod pieces;
mod prelude;
mod camera;
mod buffers;
mod evaluate;
mod animation;
mod websocket;
mod reset;

use setup::setup;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    setup().await?;
    Ok(())
}
