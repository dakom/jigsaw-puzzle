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
pub type DomViewMut<'a> = NonSendSync<UniqueViewMut<'a, Dom>>;
pub type DomView<'a> = NonSendSync<UniqueView<'a, Dom>>;

#[derive(Component)]
pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub canvas: HtmlCanvasElement,
    ui_container: HtmlElement,
    info_header: RefCell<Option<HtmlElement>>,
    btn: RefCell<Option<HtmlElement>>,
}

impl Dom {
    pub fn new() -> Self {
        let window = web_sys::window().expect_throw("should have a Window");
        let document = window.document().expect_throw("should have a Document");
        let body = document.body().expect_throw("should have a Body");

        let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap_ext().dyn_into().unwrap_ext();

        let ui_container: HtmlElement = document.create_element("div").unwrap_ext().dyn_into().unwrap_ext();
        ui_container.set_class_name("ui-container");
        body.append_child(&ui_container).unwrap_ext();

        Self {
            window,
            document,
            body,
            canvas,
            ui_container,
            info_header: RefCell::new(None),
            btn: RefCell::new(None)
        }
    }

    pub fn _clear_ui(&self) {
        if let Some(header) = self.info_header.borrow_mut().take() {
            self.ui_container.remove_child(&header.unchecked_into()).unwrap_ext();
        }
        if let Some(btn) = self.btn.borrow_mut().take() {
            self.ui_container.remove_child(&btn.unchecked_into()).unwrap_ext();
        }
    }

    pub fn with_btn<A>(&self, mut f: impl FnMut(&HtmlElement) -> A) -> A {
        f(self.btn.borrow().as_ref().unwrap_ext())
    }

    pub fn set_info_header_text(&self, text: &str) {
        if self.info_header.borrow().is_none() {

            let header: HtmlElement = self.document.create_element("div").unwrap_ext().dyn_into().unwrap_ext();
            header.set_class_name("header");
            self.ui_container.append_child(&header).unwrap_ext();
            *self.info_header.borrow_mut() = Some(header);
        }
        self.info_header.borrow().as_ref().unwrap_ext().set_text_content(Some(text));
    }

    pub fn start_game_ui(&self) {
        self.set_info_header_text("click and drag, space-click to pan, mouse wheel to zoom");


        let btn: HtmlElement = self.document.create_element("div").unwrap_ext().dyn_into().unwrap_ext();
        btn.set_class_name("button");
        self.ui_container.append_child(&btn).unwrap_ext();
        *self.btn.borrow_mut() = Some(btn);
        self.btn.borrow().as_ref().unwrap_ext().set_text_content(Some("start"));
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
