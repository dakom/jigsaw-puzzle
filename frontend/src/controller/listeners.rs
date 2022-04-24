// Pass in callbacks and hold onto it
// When it's dropped, all the event listeners are too 
//
// delta is since last move
// diff is since pointer down
use gloo_events::EventListener;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::convert::TryInto;
use super::state::InputState;
use shipyard::*;
use crate::dom::{Dom, DomView};
use super::queue::*;
use super::helpers::get_canvas_x_y;
use crate::prelude::*;

pub struct InputListeners {
    _listeners: Vec<EventListener>,
}

// These listeners are pure input events.
// they have no knowledge of a "controller" or anything else in the world
// other than the DomView which it borrows
// websocket events are dealt with in websocket.rs
impl InputListeners {
    pub fn new(world:Rc<World>) -> Self 
    {
        fn dom_view(world: &World) -> DomView {
            world.borrow::<DomView>().unwrap_ext()
        }
        let state = Rc::new(InputState::new());
        let window = web_sys::window().unwrap_ext();

        let listeners = vec![
            dom_view(&world).with_btn(|btn| EventListener::new(btn, "click", {
                let state = state.clone();
                let world = world.clone();
                move |event| {
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert_replace(Input::ResetButton);
                    });
                }
            })),
            EventListener::new(&window, "pointerdown", {
                let state = state.clone();
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&dom_view(&world).canvas, event);
                    state.is_pointer_down.store(true, Ordering::SeqCst);
                    state.first_pointer_move_x.store(x, Ordering::SeqCst);
                    state.first_pointer_move_y.store(y, Ordering::SeqCst);
                    state.last_pointer_move_x.store(x, Ordering::SeqCst);
                    state.last_pointer_move_y.store(y, Ordering::SeqCst);

                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert_replace(Input::PointerDown(x, y));
                    });

                }
            }),
            
            EventListener::new(&window, "pointermove", {
                let state = state.clone();
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&dom_view(&world).canvas, event);
                    if state.is_pointer_down.load(Ordering::SeqCst) {
                        
                        let (first_x, first_y) = (
                            state.first_pointer_move_x.load(Ordering::SeqCst),
                            state.first_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (last_x, last_y) = (
                            state.last_pointer_move_x.load(Ordering::SeqCst),
                            state.last_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (diff_x, diff_y) = (
                            x - first_x,
                            y - first_y
                        );

                        let (delta_x, delta_y) = (
                            x - last_x,
                            y - last_y
                        );

                        state.last_pointer_move_x.store(x, Ordering::SeqCst);
                        state.last_pointer_move_y.store(y, Ordering::SeqCst);

                        if diff_x != 0 || diff_y != 0 {
                            world.run(|mut queue:InputQueueViewMut| {
                                queue.insert_always(Input::PointerDrag(
                                    x, y, 
                                    delta_x, delta_y, 
                                    diff_x, diff_y
                                ));
                            });
                        }
                    } else {
                        world.run(|mut queue:InputQueueViewMut| {
                            queue.insert_replace(Input::PointerHover(x, y));
                        });
                    }
                }
            }),

            //On window since pointerup is almost always after pointerdown
            //and we want to catch it anywhere
            EventListener::new(&window, "pointerup", {
                let world = world.clone();
                move |event| {
                    if state.is_pointer_down.load(Ordering::SeqCst) {

                        let (x, y) = get_canvas_x_y(&dom_view(&world).canvas, event);
                        
                        let (first_x, first_y) = (
                            state.first_pointer_move_x.load(Ordering::SeqCst),
                            state.first_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (last_x, last_y) = (
                            state.last_pointer_move_x.load(Ordering::SeqCst),
                            state.last_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (diff_x, diff_y) = (
                            x - first_x,
                            y - first_y
                        );

                        let (delta_x, delta_y) = (
                            x - last_x,
                            y - last_y
                        );

                        state.last_pointer_move_x.store(x, Ordering::SeqCst);
                        state.last_pointer_move_y.store(y, Ordering::SeqCst);

                        if diff_x != 0 || diff_y != 0 {
                            world.run(|mut queue:InputQueueViewMut| {
                                queue.insert_replace(Input::PointerUp(
                                    x, y, 
                                    delta_x, delta_y, 
                                    diff_x, diff_y
                                ));
                            });
                        }
                    }
                    state.is_pointer_down.store(false, Ordering::SeqCst);
                }
            }),

            EventListener::new(&dom_view(&world).canvas, "click", {
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&dom_view(&world).canvas, event);
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert_replace(Input::PointerClick( x, y ));
                    });
                }
            }),

            EventListener::new(&dom_view(&world).canvas, "wheel", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::WheelEvent>().unwrap_ext();
                    if let Ok(mode) = event.delta_mode().try_into() {
                        world.run(|mut queue:InputQueueViewMut| {
                            queue.insert_replace(Input::Wheel(
                                mode, 
                                event.delta_x(), 
                                event.delta_y(), 
                                event.delta_z()
                            ));
                        });
                    }
                }
            }),


            // keys need the element to be focused...
            // probably a nicer way to do this, so it doesn't interfere
            // with regular dom input, but w/e
            EventListener::new(&window, "keydown", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_ext();
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert_replace(Input::KeyDown(event.into()));
                    });
                }
            }),

            EventListener::new(&window, "keyup", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_ext();
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert_replace(Input::KeyUp(event.into()));
                    });
                }
            }),

        ];

        Self {
            _listeners: listeners,
        }
    }
}
