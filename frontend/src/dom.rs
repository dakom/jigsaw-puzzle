use shipyard::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement, WebGlRenderingContext};
use awsm_web::window::get_window_size;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
};
use std::rc::Rc;
use std::cell::RefCell;
use futures::channel::oneshot;
use crate::prelude::*;

pub mod ui;
pub mod theme;
pub(super) mod canvas;

use ui::state::Ui;
use canvas::Canvas;

pub type DomViewMut<'a> = NonSendSync<UniqueViewMut<'a, DomState>>;
pub type DomView<'a> = NonSendSync<UniqueView<'a, DomState>>;

#[derive(Component, Unique)]
pub struct DomState {
    pub ui: Rc<Ui>,
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub canvas: HtmlCanvasElement,
}

impl DomState {
    pub async fn new(world: Rc<World>) -> Self {
        let window = web_sys::window().expect_throw("should have a Window");
        let document = window.document().expect_throw("should have a Document");
        let body = document.body().expect_throw("should have a Body");

        theme::init_stylesheet();

        let (mut tx, mut rx) = oneshot::channel();
        dominator::append_dom(&body, Canvas::render(tx)); 

        let ui = Ui::new(world);
        dominator::append_dom(&body, Ui::render(ui.clone())); 

        let canvas = rx.await.unwrap_ext();

        DomState {
            ui,
            window,
            document,
            body,
            canvas
        }
    }

    pub fn window_size(&self) -> (u32, u32) {
        get_window_size(&self.window).unwrap_ext()
    }

    pub fn create_gl_context(&self) -> WebGlRenderingContext {
        //not using any webgl2 features so might as well stick with v1
        get_webgl_context_1(&self.canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_ext()
    }
}
