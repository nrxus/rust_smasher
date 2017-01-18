extern crate glm;
extern crate num_traits;
extern crate sdl2;

use std::error::Error;

use sdl2::render::Renderer as SdlRenderer;
use sdl2::EventPump as SdlEventPump;
use sdl2::image::{INIT_PNG, INIT_JPG};

use resource_manager::*;
use input_manager::*;

pub mod input_manager;
pub mod resource_manager;
pub mod window_wrapper;

pub trait MohoEngine {
    type Renderer: Renderer;
    type EventPump: EventPump;
}

pub struct SdlMohoEngine {}

impl MohoEngine for SdlMohoEngine {
    type Renderer = SdlRenderer<'static>;
    type EventPump = SdlEventPump;
}

pub fn init(name: &'static str,
            width: u32,
            height: u32)
            -> Result<(ResourceManager<SdlRenderer>, InputManager<SdlEventPump>), Box<Error>> {
    let sdl_ctx = sdl2::init()?;
    let video_ctx = sdl_ctx.video()?;
    let _image_ctx = sdl2::image::init(INIT_PNG | INIT_JPG)?;

    let window = video_ctx.window(name, width, height)
        .position_centered()
        .opengl()
        .build()?;

    let renderer = window.renderer().present_vsync().build()?;
    let mut resource_manager = ResourceManager::new(renderer);
    resource_manager.clear();
    resource_manager.present();

    let event_pump = sdl_ctx.event_pump()?;
    let input_manager = InputManager::new(event_pump);
    Ok((resource_manager, input_manager))
}
