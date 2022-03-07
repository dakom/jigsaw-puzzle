use shipyard::*;
use shipyard_scenegraph::init::init_scenegraph;
use crate::buffers::{update_buffers_sys, DataBuffers};
use crate::camera::Camera;
use crate::evaluate::evaluate_sys;
use crate::mainloop::UpdateTick;
use crate::media::Media;
use crate::pieces::pieces_order_sys;
use crate::renderer::{SceneRenderer, picker::InteractableLookup};
use shipyard_scenegraph::prelude::*;
use crate::controller::{queue::InputQueue, Controller, controller_set_sys, controller_process_sys, controller_clear_sys};
use crate::renderer::render_sys;
use crate::animation::{animation_sys, animation_clear_sys};
use std::collections::HashMap;
use nalgebra_glm::{Vec3, Mat4, Quat};
use crate::prelude::*;
use crate::dom::Dom;

pub fn init_world(dom: Dom, media: Media, renderer:SceneRenderer) -> World {
    let world = World::new();

    world.add_unique(Controller::default());
    world.add_unique(InputQueue::new());
    world.add_unique(UpdateTick::default());
    world.add_unique(InteractableLookup::default());
    world.add_unique(Camera::default());
    world.add_unique(DataBuffers::default());
    world.add_unique_non_send_sync(media);
    world.add_unique_non_send_sync(renderer);
    world.add_unique_non_send_sync(dom);

    register_workloads(&world);

    init_scenegraph::<Vec3, Quat, Mat4, f32>(&world);

    world
}

pub const RENDER:&str = "RENDER";
pub const CONTROLLER:&str = "CONTROLLER";
pub const CLEANUP:&str = "CLEANUP";
pub const GAMEPLAY:&str = "GAMEPLAY";

pub fn register_workloads(world:&World) {

    Workload::builder(RENDER)
        .with_system(local_transform_sys)
        .with_system(world_transform_sys)
        .with_system(update_buffers_sys)
        .with_system(render_sys)
        .add_to_world(world)
        .unwrap_ext();

    Workload::builder(CONTROLLER)
        .with_system(controller_set_sys)
        .with_system(controller_process_sys)
        .with_system(pieces_order_sys)
        .add_to_world(world)
        .unwrap_ext();

    Workload::builder(GAMEPLAY)
        .with_system(evaluate_sys)
        .with_system(animation_sys)
        .add_to_world(world)
        .unwrap_ext();

    Workload::builder(CLEANUP)
        .with_system(controller_clear_sys)
        .with_system(animation_clear_sys)
        .add_to_world(world)
        .unwrap_ext();
}

