use crate::engine::renderer::Renderer;

use super::button::Button;

pub enum MainMenuAction {
    CreateWorld,
    LoadWorld,
    Exit,
}

pub enum CreateWorldAction {
    Create,
    Back,
    ToggleMode,
    ToggleGenerator,
}

pub struct MainMenu {
    pub buttons: Vec<Button>,
}

pub struct CreateWorldMenu {
    pub buttons: Vec<Button>,
}

impl MainMenu {
    pub fn new(center_x: f32, y: f32) -> Self {
        Self {
            buttons: vec![
                Button::new("CRIAR MUNDO", center_x - 120.0, y, 240.0, 48.0),
                Button::new("CARREGAR MUNDO", center_x - 120.0, y + 64.0, 240.0, 48.0),
                Button::new("SAIR", center_x - 120.0, y + 128.0, 240.0, 48.0),
            ],
        }
    }

    pub fn click(&self, mx: f32, my: f32) -> Option<MainMenuAction> {
        self.buttons.iter().enumerate().find_map(|(i, b)| {
            if b.contains(mx, my) {
                Some(match i {
                    0 => MainMenuAction::CreateWorld,
                    1 => MainMenuAction::LoadWorld,
                    _ => MainMenuAction::Exit,
                })
            } else {
                None
            }
        })
    }

    pub fn render(&self, renderer: &Renderer, w: f32) {
        draw_text(renderer, "MINECRAFT IN RUST", w * 0.5 - 220.0, 80.0, 6.0, [0.05, 0.05, 0.05]);
        for b in &self.buttons {
            renderer.rect(b.x, b.y, b.w, b.h, [0.2, 0.25, 0.3]);
            draw_text(renderer, &b.label, b.x + 14.0, b.y + 14.0, 3.5, [0.9, 0.9, 0.9]);
        }
    }
}

impl CreateWorldMenu {
    pub fn new(center_x: f32, y: f32) -> Self {
        Self {
            buttons: vec![
                Button::new("MODO", center_x - 140.0, y + 86.0, 280.0, 42.0),
                Button::new("GERACAO", center_x - 140.0, y + 136.0, 280.0, 42.0),
                Button::new("CRIAR", center_x - 140.0, y + 196.0, 135.0, 48.0),
                Button::new("VOLTAR", center_x + 5.0, y + 196.0, 135.0, 48.0),
            ],
        }
    }

    pub fn click(&self, mx: f32, my: f32) -> Option<CreateWorldAction> {
        self.buttons.iter().enumerate().find_map(|(i, b)| {
            if b.contains(mx, my) {
                Some(match i {
                    0 => CreateWorldAction::ToggleMode,
                    1 => CreateWorldAction::ToggleGenerator,
                    2 => CreateWorldAction::Create,
                    _ => CreateWorldAction::Back,
                })
            } else {
                None
            }
        })
    }

    pub fn render(
        &self,
        renderer: &Renderer,
        w: f32,
        name: &str,
        mode: &str,
        generator: &str,
    ) {
        draw_text(renderer, "CRIAR NOVO MUNDO", w * 0.5 - 190.0, 60.0, 5.0, [0.05, 0.05, 0.05]);
        renderer.rect(w * 0.5 - 140.0, 130.0, 280.0, 42.0, [0.14, 0.14, 0.17]);
        draw_text(renderer, &format!("NOME: {}", name), w * 0.5 - 126.0, 144.0, 3.0, [0.95, 0.95, 0.95]);

        for b in &self.buttons {
            renderer.rect(b.x, b.y, b.w, b.h, [0.2, 0.25, 0.3]);
        }

        draw_text(renderer, &format!("MODO: {}", mode), w * 0.5 - 120.0, 230.0, 3.0, [0.95, 0.95, 0.95]);
        draw_text(
            renderer,
            &format!("GERACAO: {}", generator),
            w * 0.5 - 120.0,
            280.0,
            3.0,
            [0.95, 0.95, 0.95],
        );
        draw_text(renderer, "CRIAR", w * 0.5 - 100.0, 340.0, 3.8, [0.95, 0.95, 0.95]);
        draw_text(renderer, "VOLTAR", w * 0.5 + 40.0, 340.0, 3.8, [0.95, 0.95, 0.95]);
    }
}

pub fn draw_text(renderer: &Renderer, text: &str, x: f32, y: f32, scale: f32, color: [f32; 3]) {
    let mut pen_x = x;
    for c in text.chars() {
        if c == ' ' {
            pen_x += 6.0 * scale;
            continue;
        }
        if let Some(bitmap) = glyph(c.to_ascii_uppercase()) {
            for (row, bits) in bitmap.iter().enumerate() {
                for col in 0..5 {
                    if ((bits >> (4 - col)) & 1) == 1 {
                        renderer.rect(
                            pen_x + col as f32 * scale,
                            y + row as f32 * scale,
                            scale,
                            scale,
                            color,
                        );
                    }
                }
            }
        }
        pen_x += 6.0 * scale;
    }
}

fn glyph(c: char) -> Option<[u8; 7]> {
    Some(match c {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b11110, 0b10001, 0b10001, 0b10001, 0b11110],
        'C' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0b10000],
        'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001, 0b10001],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        ':' => [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        d if d.is_ascii_digit() => match d {
            '0' => [0b01110, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b01110],
            '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
            '2' => [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
            '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
            '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
            '5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
            '6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
            '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
            '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
            _ => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        },
        _ => return None,
    })
}
