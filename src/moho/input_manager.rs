extern crate glm;

use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct InputManagerImp {
    pressed_keys: HashSet<Keycode>,
}

impl InputManagerImp {
    pub fn new() -> Self {
        InputManagerImp {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn press_key(&mut self, keycode: Keycode) {
        self.pressed_keys.insert(keycode);
    }

    pub fn release_key(&mut self, keycode: Keycode) {
        self.pressed_keys.remove(&keycode);
    }

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::keyboard::Keycode;
    
    #[test]
    fn it_adds_pressed_keys() {
        let mut subject = InputManagerImp::new();
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        subject.press_key(Keycode::Down);
        assert_eq!(subject.is_key_down(Keycode::Down), true);
    }

    #[test]
    fn it_releases_keys() {
        let mut subject = InputManagerImp::new();
        subject.press_key(Keycode::Down);
        subject.press_key(Keycode::Up);
        subject.release_key(Keycode::Down);
        assert_eq!(subject.is_key_down(Keycode::Down), false);
        assert_eq!(subject.is_key_down(Keycode::Up), true);
    }
}
