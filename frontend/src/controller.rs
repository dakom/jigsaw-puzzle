pub mod listeners;
pub mod state;
pub mod helpers;
pub mod queue;

use queue::*;
use crate::buffers::DataBuffers;
use crate::media::MediaView;
use crate::reset::Reset;
use crate::{prelude::*, camera::Camera};
use std::convert::TryInto;
use crate::renderer::RendererViewMut;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use crate::renderer::picker::*;
use crate::config::{ZOOM_AMOUNT, ZOOM_MIN, ZOOM_MAX};
use crate::pieces::{PiecesOrder, PieceState};
use crate::animation::{TweenPos};
use crate::evaluate::Evaluate;

pub type ControllerViewMut<'a> = UniqueViewMut<'a, Controller>;

#[derive(Component, Unique, PartialEq, Default)]
pub struct Controller {
    pub drag: Option<DragController>,
    pub camera: CameraController,
    pub first_selected: Option<EntityId>,
    pub let_go: Option<EntityId>,
    pub reset: bool,
}

#[derive(Component, PartialEq)]
pub enum DragController {
    Selected(EntityId),
    Move(EntityId, i32, i32),
}

#[derive(Component, PartialEq, Default)]
pub struct CameraController {
    pub pan: Option<(f64, f64)>,
    pub zoom: Option<(f64)>,
}

impl DragController {
    pub fn get_selected(&self) -> Option<EntityId> {
        match self {
            Self::Selected(entity) => Some(*entity),
            Self::Move(entity, _, _) => Some(*entity),
            _ => None
        }
    }

}

// the first step in mapping input to world updates: input -> controller
// at this point it isn't actually updating anything else in the world though
// just pure controller mapping
// it needs the renderer for entity selection detection though
pub fn controller_set_sys(
    mut renderer: RendererViewMut, 
    mut input_queue: InputQueueViewMut,
    mut controller: ControllerViewMut,
    lookup: InteractableLookupView,
) {
     for input in input_queue.0.drain(..) {
        match input {
            Input::PointerDown(x, y) => {
                if controller.camera.pan.is_none() {
                    if let Some(index) = renderer.get_picker_index(x.try_into().unwrap_ext(), y.try_into().unwrap_ext()).unwrap_ext() {
                        let entity = lookup.index_to_entity.get(&index).unwrap_ext();
                        controller.drag = Some(DragController::Selected(*entity));
                        controller.first_selected = Some(*entity);
                    }
                }
            },
            Input::PointerDrag(_x, _y, delta_x, delta_y, _diff_x, _diff_y) => {
                if let Some(drag) = controller.drag.as_ref() {
                    if let Some(entity) = drag.get_selected() {
                        match drag {
                            DragController::Move(entity, old_delta_x, old_delta_y) => {
                                controller.drag = Some(DragController::Move(*entity, old_delta_x + delta_x, old_delta_y + delta_y));
                            },
                            _ => {
                                controller.drag = Some(DragController::Move(entity, delta_x, delta_y));
                            }
                        }
                    }
                }

                if let Some((old_delta_x, old_delta_y)) = controller.camera.pan {
                    controller.camera.pan = Some((old_delta_x + (delta_x as f64), old_delta_y + (delta_y as f64)));
                }
            },
            Input::PointerUp(_, _, _, _, _, _) => {
                if let Some(entity) = controller.drag.as_ref().and_then(|drag| drag.get_selected()) {
                    controller.let_go = Some(entity);
                }
                controller.drag = None;
                controller.camera.pan = None;
                controller.first_selected = None; 
            },
            Input::KeyDown(key) => {
                if key.space && controller.drag.is_none() {
                    if controller.camera.pan.is_none() {
                        controller.camera.pan = Some((0.0, 0.0))
                    }
                }
            },
            Input::KeyUp(key) => {
                if key.space {
                    log::info!("resetting!");
                    controller.camera.pan = None;
                }
            },
            Input::Wheel(mode, x, y, z) => {
                match mode {
                    WheelDeltaMode::Pixel => {
                        if y != 0.0 {
                            controller.camera.zoom = Some(y);
                        } 
                    },
                    WheelDeltaMode::Line => {
                        log::warn!("unsupported scroll wheel per-line");
                    },
                    WheelDeltaMode::Page => {
                        log::warn!("unsupported scroll wheel per-page");
                    }
                }
            }
            Input::ResetButton => {
                controller.reset = true;
            },
            _ => {}
        }
     }
}

// now we actually process the controller, and no longer care about "input"
pub fn controller_process_sys(
    media: MediaView,
    lookup: InteractableLookupView,
    piece_states: View<PieceState>,
    mut controller: ControllerViewMut,
    mut entities: EntitiesViewMut,
    mut pieces_order: UniqueViewMut<PiecesOrder>,
    mut translations:ViewMut<Translation>,
    mut evaluates:ViewMut<Evaluate>,
    mut camera:UniqueViewMut<Camera>,
) {

    if let Some(entity) = controller.first_selected {
        if let Ok(piece_state) = (&piece_states).get(entity) {
            if *piece_state == PieceState::Free {
                entities.add_component(entity, &mut evaluates, Evaluate::Select);
            } else {
                controller.drag = None;
            }
        }
    } else if let Some(entity) = controller.let_go {
        entities.add_component(entity, &mut evaluates, Evaluate::Release);
    }

    // the dividing by zoom is a happy accident.. haven't really thought it through :P
    // but it works great!
    if let Some(drag) = controller.drag.as_ref() {
        match drag {
            DragController::Move(entity, x, y) => { 
                if let Ok(mut p) = (&mut translations).get(*entity) {
                    p.x += ((*x as f32) / (camera.zoom as f32)); 
                    p.y += ((*y as f32) / (camera.zoom as f32)); 
                }
            },
            _ => {}
        }
    }
    if let Some((x, y)) = controller.camera.pan.as_ref() {
        camera.x -= *x / camera.zoom;
        camera.y -= *y / camera.zoom;
    }
    if let Some(zoom) = controller.camera.zoom {
        if zoom > 0.0 {
            camera.zoom = ZOOM_MIN.max(camera.zoom - ZOOM_AMOUNT);
        } else if zoom < 0.0 {
            camera.zoom = ZOOM_MAX.min(camera.zoom + ZOOM_AMOUNT);
        }
    }

}

// clear the controller after each tick
// but in a specific way which maintains some state
pub fn controller_clear_sys(
    mut controller: ControllerViewMut,
    mut reset: UniqueViewMut<Reset>
) {
    if let Some(drag) = controller.drag.as_ref() {
        controller.drag = match drag.get_selected() {
            Some(entity) => Some(DragController::Selected(entity)),
            _ => None 
        }
    }

    if controller.camera.zoom.is_some() {
        controller.camera.zoom = None;
    }
    if controller.camera.pan.is_some() {
        controller.camera.pan = Some((0.0, 0.0));
    }
    if controller.reset {
        reset.0 = true;
    }

    controller.first_selected = None;
    controller.let_go = None;
    controller.reset = false;
}

