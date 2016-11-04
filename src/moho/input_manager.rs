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
    prev_pressed_buttons: HashSet<Mouse>,
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
            prev_pressed_buttons: HashSet::new(),
            mouse_coords: glm::ivec2(0, 0),
            event_stream_generator: events_generator,
        }
    }

    pub fn update(&'a mut self) {
        let event_stream = self.event_stream_generator.next();
        self.prev_pressed_buttons = self.pressed_buttons.clone();
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

#[cfg(test)]
mod tests {
    extern crate glm;

    use super::*;
    use sdl2::keyboard::{Keycode, NOMOD};
    use sdl2::event::Event;
    use sdl2::mouse::{MouseState, Mouse};

    struct MockEventIterator {
        events: Vec<Event>,
    }

    impl Iterator for MockEventIterator {
        type Item = Event;

        fn next(&mut self) -> Option<Event> {
            self.events.pop()
        }
    }

    struct MockEventStreamGenerator {
        streams: Vec<MockEventIterator>,
    }

    impl<'a> EventStreamGenerator<'a> for MockEventStreamGenerator {
        type I = MockEventIterator;

        fn next(&'a mut self) -> EventStream<MockEventIterator> {
            let stream = self.streams.pop().unwrap();
            EventStream { event_iterator: stream }
        }
    }

    macro_rules! key_event {
        ($t:ident, $e:expr) => {
            {
                let event = Event::$t {
                    keycode: Some($e),
                    timestamp: 0,
                    window_id: 0,
                    scancode: None,
                    repeat: false,
                    keymod: NOMOD,
                };
                event
            }
        };
    }

    macro_rules! mouse_event {
        ($t:ident, $e:expr) => {
            {
                let event = Event::$t {
                    mouse_btn: $e,
                    timestamp: 0,
                    window_id: 0,
                    which: 0,
                    x: 0,
                    y: 0,
                };
                event
            }
        };
    }

    #[test]
    fn press_keys() {
        let streams = vec![MockEventIterator {
                               events: vec![key_event!(KeyDown, Keycode::Down),
                                            key_event!(KeyDown, Keycode::Up)],
                           }];

        let mut subject = InputManager::new(MockEventStreamGenerator { streams: streams });

        // Nothing is set before
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        assert_eq!(subject.is_key_down(Keycode::Up), false);

        subject.update();

        // Both keys are set after
        assert_eq!(subject.is_key_down(Keycode::Down), true);
        assert_eq!(subject.is_key_down(Keycode::Up), true);
    }

    #[test]
    fn release_keys() {
        let streams = vec![MockEventIterator { events: vec![key_event!(KeyUp, Keycode::Down)] },
                           MockEventIterator { events: vec![key_event!(KeyDown, Keycode::Down),
                                                            key_event!(KeyDown, Keycode::Up)] },];

        let mut subject = InputManager::new(MockEventStreamGenerator { streams: streams });
        subject.update();

        // Both keys set after
        assert_eq!(subject.is_key_down(Keycode::Down), true);
        assert_eq!(subject.is_key_down(Keycode::Up), true);
        subject.update();

        // Only the one released unset after
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        assert_eq!(subject.is_key_down(Keycode::Up), true);
    }

    #[test]
    fn mouse_coords() {
        let streams = vec![MockEventIterator {
                               events: vec![Event::MouseMotion {
                                                timestamp: 0,
                                                window_id: 0,
                                                which: 0,
                                                mousestate: MouseState::from_flags(0),
                                                x: 50,
                                                y: 30,
                                                xrel: 0,
                                                yrel: 0,
                                            }],
                           }];

        let mut subject = InputManager::new(MockEventStreamGenerator { streams: streams });
        subject.update();
        assert_eq!(subject.mouse_coords(), glm::ivec2(50 as i32, 30 as i32));
    }

    #[test]
    fn mouse_clicks() {
        let streams =
            vec![MockEventIterator { events: vec![mouse_event!(MouseButtonDown, Mouse::Right)] },
                 MockEventIterator { events: vec![mouse_event![MouseButtonDown, Mouse::Left]] }];

        let mut subject = InputManager::new(MockEventStreamGenerator { streams: streams });

        // Nothing has been clicked
        assert_eq!(subject.did_click_mouse(Mouse::Right), false);
        assert_eq!(subject.did_click_mouse(Mouse::Left), false);

        // Left button is click
        subject.update();
        assert_eq!(subject.did_click_mouse(Mouse::Right), false);
        assert_eq!(subject.did_click_mouse(Mouse::Left), true);

        // Right button is clicked - left button is still pressed but not a recent click
        subject.update();
        assert_eq!(subject.did_click_mouse(Mouse::Right), true);
        assert_eq!(subject.did_click_mouse(Mouse::Left), false);
    }

    #[test]
    fn mouse_releases() {
        let streams =
            vec![MockEventIterator { events: vec![mouse_event!(MouseButtonDown, Mouse::Right)] },
                 MockEventIterator { events: vec![mouse_event!(MouseButtonUp, Mouse::Left)] },
                 MockEventIterator { events: vec![mouse_event![MouseButtonDown, Mouse::Left]] }];

        let mut subject = InputManager::new(MockEventStreamGenerator { streams: streams });

        // Nothing has been clicked
        assert_eq!(subject.did_release_mouse(Mouse::Right), false);
        assert_eq!(subject.did_release_mouse(Mouse::Left), false);

        // Left button is click
        subject.update();
        assert_eq!(subject.did_release_mouse(Mouse::Right), false);
        assert_eq!(subject.did_release_mouse(Mouse::Left), false);

        // Left button is released
        subject.update();
        assert_eq!(subject.did_release_mouse(Mouse::Right), false);
        assert_eq!(subject.did_release_mouse(Mouse::Left), true);

        // Right button is clicked; left button is not clicked and not released recently
        subject.update();
        assert_eq!(subject.did_release_mouse(Mouse::Right), false);
        assert_eq!(subject.did_release_mouse(Mouse::Right), false);
    }
}
