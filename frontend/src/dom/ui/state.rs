use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use dominator::{Dom, html};
use shipyard::*;
use std::rc::Rc;

pub struct Ui {
    pub button: Mutable<Option<ButtonState>>,
    pub header: Mutable<HeaderState>,
    pub world: Rc<World>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonState {
    Reset,
    Start
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeaderState {
    Loading,
    Prepping,
    Playing,
    Connecting,
}

impl Ui {
    pub fn new(world: Rc<World>) -> Rc<Self> {
        Rc::new(Self {
            button: Mutable::new(None),
            header: Mutable::new(HeaderState::Loading),
            world,
        })
    }
}

