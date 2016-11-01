extern crate glm;

use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use self::glm::*;

pub struct InputManagerImp {
    pressed_keys: HashSet<Keycode>,
    previously_pressed_keys: HashSet<Keycode>,
    mouse_coords: glm::Vector2<f64>,
}

impl InputManagerImp {
    pub fn new() -> Self {
        InputManagerImp {
            pressed_keys: HashSet::new(),
            previously_pressed_keys: HashSet::new(),
            mouse_coords: dvec2(0., 0.),
        }
    }

    pub fn update(&mut self) {
        self.previously_pressed_keys = self.pressed_keys.clone();
    }

    pub fn set_mouse_coords(&mut self, x: f64, y: f64) {
        self.mouse_coords = dvec2(x, y);
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

    pub fn was_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.previously_pressed_keys.contains(&keycode)
    }

    pub fn was_key_released(&self, keycode: Keycode) -> bool {
        !self.pressed_keys.contains(&keycode) && self.previously_pressed_keys.contains(&keycode)
    }

    pub fn get_mouse_coords(&self) -> glm::Vector2<f64> {
        self.mouse_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::keyboard::Keycode;
    
    #[test]
    fn it_adds_pressed_keys() {
        let mut subject = InputManagerImp::new();
        subject.press_key(Keycode::Down);
        assert_eq!(subject.is_key_down(Keycode::Down), true);
    }
}
