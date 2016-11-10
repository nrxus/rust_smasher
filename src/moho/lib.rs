extern crate sdl2;
extern crate sdl2_image;

use sdl2::render::Renderer as SdlRenderer;
use sdl2_image::{INIT_PNG, INIT_JPG};
use std::error::Error;

pub mod input_manager;
pub mod resource_manager;
pub mod window_wrapper;

use resource_manager::*;
use input_manager::*;

pub fn init(name: &str,
            width: u32,
            height: u32)
            -> Result<(ResourceManager<SdlRenderer>, InputManager<SdlEventStreamGenerator>), Box<Error>> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let _image_ctx = try!(sdl2_image::init(INIT_PNG | INIT_JPG));

    let window = try!(video_ctx.window(name, width, height)
        .position_centered()
        .opengl()
        .build());

    let renderer = try!(window.renderer().present_vsync().build());
    let mut resource_manager = ResourceManager::new(renderer);
    resource_manager.clear();
    resource_manager.present();

    let event_pump = try!(sdl_ctx.event_pump());
    let sdl_event_generator = SdlEventStreamGenerator { event_pump: event_pump };
    let input_manager: InputManager<SdlEventStreamGenerator> =
        InputManager::new(sdl_event_generator);
    Ok((resource_manager, input_manager))
}
