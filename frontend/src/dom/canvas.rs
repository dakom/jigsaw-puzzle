use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use dominator::{Dom, html, clone, events};
use web_sys::HtmlCanvasElement;
use crate::controller::queue::{InputQueueViewMut, Input};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use futures::channel::oneshot::{Sender};

use dominator::{stylesheet, class, pseudo};

pub struct Canvas {
}
impl Canvas {
    pub fn render(tx: Sender<HtmlCanvasElement>) -> Dom {
        html!("canvas" => web_sys::HtmlCanvasElement, {
            .class(&*CLASS)
            .after_inserted(|elem| {
                tx.send(elem);
            })
        })
    }
}

const CLASS:Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("padding", "0")
        .style("margin", "0")
        .style("touch-action", "none")
        .style("cursor", "pointer")
    }
});
