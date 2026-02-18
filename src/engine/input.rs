use std::collections::HashSet;

use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct InputState {
    pub keys_down: HashSet<Keycode>,
    pub keys_pressed: HashSet<Keycode>,
    pub mouse_delta: (f32, f32),
    pub mouse_left: bool,
    pub quit_requested: bool,
}

impl InputState {
    pub fn key_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn key_pressed(&self, key: Keycode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.mouse_delta = (0.0, 0.0);
    }
}
