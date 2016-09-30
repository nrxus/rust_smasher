extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::error::Error;

pub fn init() -> Result<(), Box<Error>> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let window = try!(video_ctx.window("rust-sdl2 demo:: Video", 800, 600)
        .position_centered()
        .opengl()
        .build());
    let mut renderer = try!(window.renderer().build());
    renderer.set_draw_color(Color::RGB(255, 0, 0));

    renderer.clear();
    renderer.present();
    let mut event_pump = try!(sdl_ctx.event_pump());
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    Ok(())
}
