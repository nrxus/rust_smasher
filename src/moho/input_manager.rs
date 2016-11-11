extern crate glm;

use sdl2::EventPump as SdlEventPump;
use sdl2::event::EventPollIterator;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub trait EventPump<'a> {
    type I: Iterator<Item = Event>;

    fn poll_iter(&'a mut self) -> Self::I;
}

impl<'a> EventPump<'a> for SdlEventPump {
    type I = EventPollIterator<'a>;

    fn poll_iter(&'a mut self) -> EventPollIterator {
        self.poll_iter()
    }
}

pub struct InputManager<P> {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<Mouse>,
    prev_pressed_keys: HashSet<Keycode>,
    prev_pressed_buttons: HashSet<Mouse>,
    mouse_coords: glm::IVec2,
    event_pump: P,
}

impl<'a, P> InputManager<P>
    where P: EventPump<'a>
{
    pub fn new(events_generator: P) -> InputManager<P> {
        InputManager {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            prev_pressed_keys: HashSet::new(),
            prev_pressed_buttons: HashSet::new(),
            mouse_coords: glm::ivec2(0, 0),
            event_pump: events_generator,
        }
    }

    pub fn update(&'a mut self) -> bool {
        self.prev_pressed_keys = self.pressed_keys.clone();
        self.prev_pressed_buttons = self.pressed_buttons.clone();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return false;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.pressed_keys.insert(keycode);
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.pressed_keys.remove(&keycode);
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_coords = glm::ivec2(x, y);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.pressed_buttons.insert(mouse_btn);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.pressed_buttons.remove(&mouse_btn);
                }
                _ => {}
            }
        }
        true
    }

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn did_press_key(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.prev_pressed_keys.contains(&keycode)
    }

    pub fn did_click_mouse(&self, mouse: Mouse) -> bool {
        self.pressed_buttons.contains(&mouse) && !self.prev_pressed_buttons.contains(&mouse)
    }

    pub fn did_release_mouse(&self, mouse: Mouse) -> bool {
        !self.pressed_buttons.contains(&mouse) && self.prev_pressed_buttons.contains(&mouse)
    }

    pub fn is_mouse_down(&self, mouse: Mouse) -> bool {
        self.pressed_buttons.contains(&mouse)
    }

    pub fn mouse_coords(&self) -> glm::IVec2 {
        self.mouse_coords
    }
}
