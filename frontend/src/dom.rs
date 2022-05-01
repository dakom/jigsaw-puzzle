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
use crate::prelude::*;

pub mod ui;
pub mod theme;
pub(super) mod canvas;

use ui::state::Ui;
use canvas::Canvas;

pub type DomViewMut<'a> = NonSendSync<UniqueViewMut<'a, DomState>>;
pub type DomView<'a> = NonSendSync<UniqueView<'a, DomState>>;

#[derive(Component)]
pub struct DomState {
    pub ui: Rc<Ui>,
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub canvas_ref: Rc<RefCell<Option<HtmlCanvasElement>>>,
}

impl DomState {
    pub fn new(world: Rc<World>) -> Self {
        let window = web_sys::window().expect_throw("should have a Window");
        let document = window.document().expect_throw("should have a Document");
        let body = document.body().expect_throw("should have a Body");

        let canvas_ref = Rc::new(RefCell::new(None));

        let state = DomState {
            ui: Ui::new(world),
            window,
            document,
            body,
            canvas_ref: canvas_ref.clone()
        };

        theme::init_stylesheet();

        dominator::append_dom(&state.body, Canvas::render(canvas_ref)); 
        dominator::append_dom(&state.body, Ui::render(state.ui.clone())); 

        state
    }

    pub fn canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas_ref.borrow().as_ref().unwrap_ext().clone()
    }

    pub fn window_size(&self) -> (u32, u32) {
        get_window_size(&self.window).unwrap_ext()
    }

    pub fn create_gl_context(&self) -> WebGlRenderingContext {
        //not using any webgl2 features so might as well stick with v1
        get_webgl_context_1(&self.canvas(), Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_ext()
    }
}
