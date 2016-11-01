extern crate sdl2;

use sdl2::EventPump;
use sdl2::render::Renderer;
use std::error::Error;

pub mod input_manager;

pub fn init(name: &str, width: u32, height: u32) -> Result<(Renderer, EventPump), Box<Error>> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let window = try!(video_ctx.window(name, width, height)
        .position_centered()
        .opengl()
        .build());

    let mut renderer = try!(window.renderer().present_vsync().build());

    renderer.clear();
    renderer.present();
    let event_pump = try!(sdl_ctx.event_pump());
    Ok((renderer, event_pump))
}
