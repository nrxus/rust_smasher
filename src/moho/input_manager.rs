extern crate glm;

use sdl2::EventPump;
use sdl2::event::EventPollIterator;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub trait EventStreamGenerator<'a> {
    type I: Iterator<Item = Event>;
    fn next(&'a mut self) -> EventStream<Self::I>;
}

pub struct SdlEventStreamGenerator {
    pub event_pump: EventPump,
}

pub struct EventStream<I> {
    event_iterator: I,
}

impl<I: Iterator<Item = Event>> Iterator for EventStream<I> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        self.event_iterator.next()
    }
}

impl<'a> EventStreamGenerator<'a> for SdlEventStreamGenerator {
    type I = EventPollIterator<'a>;

    fn next(&'a mut self) -> EventStream<EventPollIterator> {
        let event_iterator = self.event_pump.poll_iter();
        EventStream { event_iterator: event_iterator }
    }
}

pub struct InputManager<E> {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<Mouse>,
    mouse_coords: glm::IVec2,
    event_stream_generator: E,
}

impl<'a, E> InputManager<E>
    where E: EventStreamGenerator<'a>
{
    pub fn new(events_generator: E) -> InputManager<E>
        where E: EventStreamGenerator<'a>
    {
        InputManager {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            mouse_coords: glm::ivec2(0, 0),
            event_stream_generator: events_generator,
        }
    }

    pub fn update(&'a mut self) {
        let event_stream = self.event_stream_generator.next();
        for event in event_stream {
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

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn is_mouse_down(&self, mouse: Mouse) -> bool {
        self.pressed_buttons.contains(&mouse)
    }

    pub fn mouse_coords(&self) -> glm::IVec2 {
        self.mouse_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::keyboard::Keycode;

    #[test]
    fn it_adds_pressed_keys() {
        let mut subject = InputManager::new();
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        subject.press_key(Keycode::Down);
        assert_eq!(subject.is_key_down(Keycode::Down), true);
    }

    #[test]
    fn it_releases_keys() {
        let mut subject = InputManager::new();
        subject.press_key(Keycode::Down);
        subject.press_key(Keycode::Up);
        subject.release_key(Keycode::Down);
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        assert_eq!(subject.is_key_down(Keycode::Up), true);
    }
}
