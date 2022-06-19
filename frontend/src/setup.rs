use crate::controller::queue::{InputQueueViewMut, Input};
use crate::dom::DomView;
use crate::dom::ui::state::{HeaderState, ButtonState};
use crate::renderer::RendererViewMut;
use crate::{
    dom::DomState,
    controller::listeners::InputListeners,
    mainloop,
    media::Media,
    renderer::SceneRenderer,
    world::init_world,
    camera::Camera,
};
use std::rc::Rc;
use gloo_events::EventListener;
use shipyard::*;
use awsm_web::tick::{Raf, MainLoop, MainLoopOptions};
use awsm_web::webgl::ResizeStrategy;
use crate::prelude::*;
use crate::websocket;

pub async fn setup() -> Result<Rc<World>, JsValue> {

    init_logger();

    let world = Rc::new(World::new());
    let dom = DomState::new(world.clone()).await;
    let media = Media::load().await;

    dom.ui.header.set_neq(HeaderState::Prepping);

    let scene_renderer = SceneRenderer::new(dom.create_gl_context(), &media)?;

    let (stage_width, stage_height) = dom.window_size();

    init_world(
        &world,
        dom,
        media,
        scene_renderer
    );


    let on_resize = {
        let world = Rc::clone(&world);
        move |_: &web_sys::Event| {
            world.run(|dom: DomView, mut camera: UniqueViewMut<Camera>, mut renderer: RendererViewMut| {
                let (width, height) = dom.window_size();
                renderer.resize(&mut camera, ResizeStrategy::All(width, height));
            });
        }
    };

    on_resize(&web_sys::Event::new("").unwrap_ext());

    EventListener::new(&world.borrow::<DomView>().unwrap_ext().window, "resize", on_resize).forget();

    world.run(|dom: DomView| {
        dom.ui.header.set_neq(HeaderState::Connecting);
    });

    // wait for websocket connection
    websocket::connect(world.clone()).await?;

    //start the game loop!
    world.run(|dom: DomView| {
        dom.ui.header.set_neq(HeaderState::Playing);
        dom.ui.button.set_neq(Some(ButtonState::Start));
    });

    let mut main_loop = MainLoop::new(
        MainLoopOptions::default(),
        {
            let world = Rc::clone(&world);
            move |time, delta| mainloop::begin(&world, time, delta)
        },
        {
            let world = Rc::clone(&world);
            move |delta| mainloop::update(&world, delta)
        },
        {
            let world = Rc::clone(&world);
            move |interpolation| mainloop::draw(&world, interpolation)
        },
        {
            let world = Rc::clone(&world);
            move |fps, abort| mainloop::end(&world, fps, abort)
        },
    );

    let tick = Raf::new({
        move |ts| {
            main_loop.tick(ts);
        }
    });

    crate::pieces::create(&world, stage_width as f32, stage_height as f32);

    // force the reset - temp
    world.run(|mut queue: InputQueueViewMut| {
        queue.insert_replace(Input::ResetButton);
    });
    // these just run forever
    std::mem::forget(Box::new(tick));
    std::mem::forget(Box::new(InputListeners::new(world.clone())));

    Ok(world)
}

// enable logging and panic hook only during debug builds
cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

