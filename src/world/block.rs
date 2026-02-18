#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Block {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl Block {
    pub fn color(&self) -> [f32; 3] {
        match self {
            Block::Air => [0.0, 0.0, 0.0],
            Block::Dirt => [0.45, 0.3, 0.15],
            Block::Grass => [0.2, 0.7, 0.2],
            Block::Stone => [0.5, 0.5, 0.55],
        }
    }

    pub fn solid(&self) -> bool {
        *self != Block::Air
    }
}
