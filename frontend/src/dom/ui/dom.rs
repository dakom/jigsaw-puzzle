use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use dominator::{Dom, html, clone, events};
use super::state::*;
use crate::controller::queue::{InputQueueViewMut, Input};

use dominator::{stylesheet, class, pseudo};
use super::styles;

impl Ui {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(&*styles::UI_CONTAINER)
            .child(html!("div", {
                    .class(&*styles::HEADER)
                    .text_signal(state.header.signal().map(|header| header.as_str()))
            }))
            .child_signal(
                state.button.signal().map(clone!(state => move  |button| {
                    button.map(|button| {
                        html!("div", {
                            .class(&*styles::BUTTON)
                            .text(button.as_str())
                            .event(clone!(state => move |evt:events::Click| {
                                state.world.run(|mut queue:InputQueueViewMut| {
                                    queue.insert_replace(Input::ResetButton);
                                });
                            }))
                        })
                    })
                }))
            )
        })
    }
}

