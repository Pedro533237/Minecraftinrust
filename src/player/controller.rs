use sdl2::keyboard::Keycode;

use crate::{engine::input::InputState, game::modes::GameMode};

use super::camera::Camera;

pub struct PlayerController {
    pub camera: Camera,
    pub velocity_y: f32,
    pub on_ground: bool,
    pub health: i32,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            velocity_y: 0.0,
            on_ground: false,
            health: 20,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32, mode: GameMode, ground_height: f32) {
        self.camera.yaw += input.mouse_delta.0 * 0.09;
        self.camera.pitch = (self.camera.pitch - input.mouse_delta.1 * 0.09).clamp(-89.0, 89.0);

        let speed = if mode == GameMode::Creative {
            14.0
        } else {
            7.5
        };
        let mut dir = [0.0, 0.0, 0.0];

        if input.key_down(Keycode::W) {
            let f = self.camera.forward();
            dir[0] += f[0];
            dir[2] += f[2];
        }
        if input.key_down(Keycode::S) {
            let f = self.camera.forward();
            dir[0] -= f[0];
            dir[2] -= f[2];
        }
        if input.key_down(Keycode::A) {
            let r = self.camera.right();
            dir[0] -= r[0];
            dir[2] -= r[2];
        }
        if input.key_down(Keycode::D) {
            let r = self.camera.right();
            dir[0] += r[0];
            dir[2] += r[2];
        }

        let len = (dir[0] * dir[0] + dir[2] * dir[2]).sqrt().max(1.0);
        self.camera.pos[0] += dir[0] / len * speed * dt;
        self.camera.pos[2] += dir[2] / len * speed * dt;

        if mode == GameMode::Creative {
            if input.key_down(Keycode::Space) {
                self.camera.pos[1] += speed * dt;
            }
            if input.key_down(Keycode::LShift) {
                self.camera.pos[1] -= speed * dt;
            }
        } else {
            self.velocity_y -= 22.0 * dt;
            if input.key_pressed(Keycode::Space) && self.on_ground {
                self.velocity_y = 9.5;
                self.on_ground = false;
            }
            self.camera.pos[1] += self.velocity_y * dt;
            if self.camera.pos[1] < ground_height + 1.8 {
                self.camera.pos[1] = ground_height + 1.8;
                self.velocity_y = 0.0;
                self.on_ground = true;
            }
        }
    }
}
