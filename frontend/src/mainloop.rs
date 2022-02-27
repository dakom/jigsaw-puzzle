use shipyard::*;
use crate::world::{RENDER, CONTROLLER, CLEANUP};
use crate::prelude::*;

pub type UpdateTickViewMut<'a> = UniqueViewMut<'a, UpdateTick>;
#[derive(Component, Default)]
pub struct UpdateTick {
    pub delta:f64,
}

//callbacks
pub fn begin(world: &World, _time: f64, _delta: f64) {
    world.run_workload(CONTROLLER).unwrap_ext();
}

pub fn update(world: &World, delta: f64) {
    *world.borrow::<UpdateTickViewMut>().unwrap_ext() = UpdateTick {delta};
    //world.run_workload(PHYSICS).unwrap_ext();
}

pub fn draw(world: &World, _interpolation:f64) {
    world.run_workload(RENDER).unwrap_ext();
}

pub fn end(world: &World, _fps: f64, _abort:bool) {
    world.run_workload(CLEANUP).unwrap_ext();
}
