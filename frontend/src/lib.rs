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

use setup::setup;
use start::start;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let world = setup().await?;
    start(world);
    Ok(())

}
