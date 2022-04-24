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
mod start;
mod websocket;

use setup::setup;
use start::start;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let world = setup().await?;
    start(world);
    Ok(())

}
