use super::block::Block;

pub const CHUNK_W: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUNK_H: usize = 64;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
    pub cx: i32,
    pub cz: i32,
    pub blocks: Vec<Block>,
}

impl Chunk {
    pub fn new(cx: i32, cz: i32) -> Self {
        Self {
            cx,
            cz,
            blocks: vec![Block::Air; CHUNK_W * CHUNK_D * CHUNK_H],
        }
    }

    #[inline]
    fn idx(x: usize, y: usize, z: usize) -> usize {
        y * CHUNK_W * CHUNK_D + z * CHUNK_W + x
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let i = Self::idx(x, y, z);
        self.blocks[i] = block;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[Self::idx(x, y, z)]
    }
}
