mod engine;
mod game;
mod player;
mod ui;
mod world;

use engine::{input::InputState, renderer::Renderer, time::Time, window::WindowContext};
use game::state::GameState;

fn main() -> Result<(), String> {
    let mut window = WindowContext::new("Minecraft in Rust (GL 2.0)", 1280, 720)?;
    let mut renderer = Renderer::new(&window)?;
    let mut input = InputState::default();
    let mut timer = Time::new();
    let mut game = GameState::new();

    'running: loop {
        timer.tick();
        if !window.poll_events(&mut input, &mut game) {
            break 'running;
        }

        game.update(&input, timer.delta_seconds());
        renderer.begin_frame(window.width as i32, window.height as i32);
        game.render(&mut renderer, window.width as f32, window.height as f32);
        renderer.end_frame();
        window.swap();

        input.end_frame();
    }

    Ok(())
}
