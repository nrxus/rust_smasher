extern crate glm;

use sdl2::EventPump as SdlEventPump;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub trait EventPump {
    fn poll_event(&mut self) -> Option<Event>;
}

impl EventPump for SdlEventPump {
    fn poll_event(&mut self) -> Option<Event> {
        self.poll_event()
    }
}

pub struct InputManager<P> {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<MouseButton>,
    prev_pressed_keys: HashSet<Keycode>,
    prev_pressed_buttons: HashSet<MouseButton>,
    mouse_coords: glm::IVec2,
    event_pump: P,
}

impl<P: EventPump> InputManager<P> {
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

    pub fn update(&mut self) -> bool {
        self.prev_pressed_keys = self.pressed_keys.clone();
        self.prev_pressed_buttons = self.pressed_buttons.clone();

        while let Some(event) = self.event_pump.poll_event() {
            if let Event::Quit { .. } = event {
                return false;
            }
            self.process_event(event)
        }
        true
    }

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn did_press_key(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.prev_pressed_keys.contains(&keycode)
    }

    pub fn did_click_mouse(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button) &&
        !self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn did_release_mouse(&self, mouse_button: MouseButton) -> bool {
        !self.pressed_buttons.contains(&mouse_button) &&
        self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button)
    }

    pub fn mouse_coords(&self) -> glm::IVec2 {
        self.mouse_coords
    }

    fn process_event(&mut self, event: Event) {
        match event {
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
}
