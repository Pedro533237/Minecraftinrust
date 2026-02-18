use crate::{engine::input::InputState, game::state::GameState};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    video::{GLContext, GLProfile, Window},
    Sdl,
};

pub struct WindowContext {
    _sdl: Sdl,
    _video: sdl2::VideoSubsystem,
    pub window: Window,
    _gl_context: GLContext,
    pump: sdl2::EventPump,
    pub width: u32,
    pub height: u32,
}

impl WindowContext {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        {
            let attr = video.gl_attr();
            attr.set_context_profile(GLProfile::Compatibility);
            attr.set_context_version(2, 1);
            attr.set_depth_size(24);
        }

        let window = video
            .window(title, width, height)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
        let pump = sdl.event_pump()?;
        video.gl_set_swap_interval(1)?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            window,
            _gl_context: gl_context,
            pump,
            width,
            height,
        })
    }

    pub fn poll_events(&mut self, input: &mut InputState, game: &mut GameState) -> bool {
        for event in self.pump.poll_iter() {
            game.handle_event(&event, self.width as f32, self.height as f32, input);
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    input.keys_down.insert(key);
                    input.keys_pressed.insert(key);
                    if key == Keycode::Escape {
                        input.quit_requested = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    input.keys_down.remove(&key);
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    input.mouse_delta.0 += xrel as f32;
                    input.mouse_delta.1 += yrel as f32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => input.mouse_left = true,
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => input.mouse_left = false,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
                    ..
                } => {
                    self.width = w as u32;
                    self.height = h as u32;
                }
                _ => {}
            }
        }
        !input.quit_requested
    }

    pub fn swap(&self) {
        self.window.gl_swap_window();
    }
}
