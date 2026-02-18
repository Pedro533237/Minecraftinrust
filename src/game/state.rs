use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};

use crate::{
    engine::{
        input::InputState,
        renderer::{look_at, Renderer},
    },
    player::controller::PlayerController,
    ui::{
        menu::{draw_text, CreateWorldAction, CreateWorldMenu, MainMenu, MainMenuAction},
        screen::Screen,
    },
    world::{
        block::Block,
        chunk::{Chunk, CHUNK_D, CHUNK_H, CHUNK_W},
        generator::{Generator, WorldGen},
        save::{self, WorldMeta},
    },
};

use super::modes::GameMode;

pub struct GameState {
    pub screen: Screen,
    pub main_menu: MainMenu,
    pub create_menu: CreateWorldMenu,
    pub world_name_input: String,
    pub mode: GameMode,
    pub world_gen: WorldGen,
    pub running_world: Option<WorldRuntime>,
}

pub struct WorldRuntime {
    pub name: String,
    pub mode: GameMode,
    pub gen_mode: WorldGen,
    pub generator: Generator,
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub player: PlayerController,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen: Screen::MainMenu,
            main_menu: MainMenu::new(640.0, 220.0),
            create_menu: CreateWorldMenu::new(640.0, 130.0),
            world_name_input: "novo_mundo".into(),
            mode: GameMode::Survival,
            world_gen: WorldGen::Realistic,
            running_world: None,
        }
    }

    pub fn handle_event(&mut self, event: &Event, width: f32, _height: f32, input: &InputState) {
        if let Event::TextInput { text, .. } = event {
            if self.screen == Screen::CreateWorld {
                self.world_name_input.push_str(text);
            }
        }

        if let Event::KeyDown {
            keycode: Some(Keycode::Backspace),
            repeat: false,
            ..
        } = event
        {
            if self.screen == Screen::CreateWorld {
                self.world_name_input.pop();
            }
        }

        if let Event::MouseButtonDown {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = *event
        {
            let mx = x as f32;
            let my = y as f32;
            match self.screen {
                Screen::MainMenu => {
                    if let Some(action) = self.main_menu.click(mx, my) {
                        match action {
                            MainMenuAction::CreateWorld => self.screen = Screen::CreateWorld,
                            MainMenuAction::LoadWorld => {
                                if let Ok(meta) = save::load_meta("novo_mundo") {
                                    self.spawn_world(
                                        meta.name,
                                        meta.seed,
                                        self.mode,
                                        meta.generator,
                                    );
                                }
                            }
                            MainMenuAction::Exit => {
                                std::process::exit(0);
                            }
                        }
                    }
                }
                Screen::CreateWorld => {
                    if let Some(action) = self.create_menu.click(mx, my) {
                        match action {
                            CreateWorldAction::ToggleMode => self.mode.toggle(),
                            CreateWorldAction::ToggleGenerator => {
                                self.world_gen = match self.world_gen {
                                    WorldGen::Flat => WorldGen::Realistic,
                                    WorldGen::Realistic => WorldGen::Flat,
                                }
                            }
                            CreateWorldAction::Create => {
                                let seed = fnv_hash(self.world_name_input.as_bytes());
                                let meta = WorldMeta {
                                    name: self.world_name_input.clone(),
                                    seed,
                                    mode: self.mode.as_str().into(),
                                    generator: self.world_gen,
                                };
                                let _ = save::save_meta(&meta);
                                self.spawn_world(meta.name, seed, self.mode, self.world_gen);
                            }
                            CreateWorldAction::Back => self.screen = Screen::MainMenu,
                        }
                    }
                }
                Screen::InGame => {}
            }
        }

        if self.screen == Screen::InGame && input.key_pressed(Keycode::Escape) {
            self.screen = Screen::MainMenu;
        }

        self.main_menu = MainMenu::new(width * 0.5, 220.0);
        self.create_menu = CreateWorldMenu::new(width * 0.5, 130.0);
    }

    fn spawn_world(&mut self, name: String, seed: u32, mode: GameMode, world_gen: WorldGen) {
        let mut runtime = WorldRuntime {
            name,
            mode,
            gen_mode: world_gen,
            generator: Generator::new(seed),
            chunks: HashMap::new(),
            player: PlayerController::new(),
        };
        runtime.ensure_chunks_around_player(2);
        self.running_world = Some(runtime);
        self.screen = Screen::InGame;
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        if self.screen == Screen::InGame {
            if let Some(world) = self.running_world.as_mut() {
                world.ensure_chunks_around_player(2);
                let ground = world
                    .sample_ground_height(world.player.camera.pos[0], world.player.camera.pos[2]);
                world.player.update(input, dt, world.mode, ground);

                if input.key_pressed(Keycode::F5) {
                    let _ = save::save_modified_chunks(&world.name, &world.chunks);
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer, width: f32, height: f32) {
        match self.screen {
            Screen::MainMenu => {
                renderer.setup_2d(width, height);
                self.main_menu.render(renderer, width);
            }
            Screen::CreateWorld => {
                renderer.setup_2d(width, height);
                self.create_menu.render(
                    renderer,
                    width,
                    &self.world_name_input,
                    self.mode.as_str(),
                    match self.world_gen {
                        WorldGen::Flat => "PLANO",
                        WorldGen::Realistic => "REALISTA",
                    },
                );
            }
            Screen::InGame => {
                if let Some(world) = &self.running_world {
                    let c = &world.player.camera;
                    let f = c.forward();
                    let view = look_at(
                        c.pos,
                        [c.pos[0] + f[0], c.pos[1] + f[1], c.pos[2] + f[2]],
                        [0.0, 1.0, 0.0],
                    );
                    renderer.setup_3d((width / height).max(0.1), 70.0, 0.1, 400.0, view);
                    world.render_world(renderer);
                    renderer.setup_2d(width, height);
                    draw_text(
                        renderer,
                        "WASD + MOUSE | F5 SALVAR | ESC MENU",
                        12.0,
                        14.0,
                        2.4,
                        [1.0, 1.0, 1.0],
                    );
                    draw_text(
                        renderer,
                        &format!(
                            "MODO: {} | VIDA: {}",
                            world.mode.as_str(),
                            world.player.health
                        ),
                        12.0,
                        36.0,
                        2.4,
                        [1.0, 1.0, 1.0],
                    );
                }
            }
        }
    }
}

impl WorldRuntime {
    fn ensure_chunks_around_player(&mut self, radius: i32) {
        let px = (self.player.camera.pos[0].floor() as i32).div_euclid(CHUNK_W as i32);
        let pz = (self.player.camera.pos[2].floor() as i32).div_euclid(CHUNK_D as i32);
        for cz in (pz - radius)..=(pz + radius) {
            for cx in (px - radius)..=(px + radius) {
                self.chunks
                    .entry((cx, cz))
                    .or_insert_with(|| self.generator.generate_chunk(cx, cz, self.gen_mode));
            }
        }
    }

    fn sample_ground_height(&self, world_x: f32, world_z: f32) -> f32 {
        let cx = (world_x.floor() as i32).div_euclid(CHUNK_W as i32);
        let cz = (world_z.floor() as i32).div_euclid(CHUNK_D as i32);
        if let Some(chunk) = self.chunks.get(&(cx, cz)) {
            let lx = (world_x.floor() as i32 - cx * CHUNK_W as i32).clamp(0, CHUNK_W as i32 - 1)
                as usize;
            let lz = (world_z.floor() as i32 - cz * CHUNK_D as i32).clamp(0, CHUNK_D as i32 - 1)
                as usize;
            for y in (0..CHUNK_H).rev() {
                if chunk.get(lx, y, lz).solid() {
                    return y as f32;
                }
            }
        }
        0.0
    }

    fn render_world(&self, renderer: &Renderer) {
        let c = &self.player.camera;
        for ((cx, cz), chunk) in &self.chunks {
            let ox = *cx as f32 * CHUNK_W as f32;
            let oz = *cz as f32 * CHUNK_D as f32;
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    for x in 0..CHUNK_W {
                        let b = chunk.get(x, y, z);
                        if b == Block::Air {
                            continue;
                        }
                        let wx = ox + x as f32;
                        let wz = oz + z as f32;
                        if (wx - c.pos[0]).abs() > 48.0 || (wz - c.pos[2]).abs() > 48.0 {
                            continue;
                        }
                        renderer.cube(wx, y as f32, wz, b.color());
                    }
                }
            }
        }
    }
}

fn fnv_hash(bytes: &[u8]) -> u32 {
    let mut hash = 2166136261u32;
    for b in bytes {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}
