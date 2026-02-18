use super::{
    block::Block,
    chunk::{Chunk, CHUNK_D, CHUNK_H, CHUNK_W},
};

#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WorldGen {
    Flat,
    Realistic,
}

pub struct Generator {
    pub seed: u32,
}

impl Generator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn generate_chunk(&self, cx: i32, cz: i32, mode: WorldGen) -> Chunk {
        let mut chunk = Chunk::new(cx, cz);
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = cx * CHUNK_W as i32 + x as i32;
                let wz = cz * CHUNK_D as i32 + z as i32;
                let height = match mode {
                    WorldGen::Flat => 20,
                    WorldGen::Realistic => self.height(wx, wz),
                };
                for y in 0..CHUNK_H {
                    let block = if y as i32 > height {
                        Block::Air
                    } else if y as i32 == height {
                        Block::Grass
                    } else if y as i32 > height - 4 {
                        Block::Dirt
                    } else {
                        Block::Stone
                    };
                    chunk.set(x, y, z, block);
                }
            }
        }
        chunk
    }

    fn height(&self, x: i32, z: i32) -> i32 {
        let base = 18.0;
        let n1 = value_noise_2d(x as f32 * 0.04, z as f32 * 0.04, self.seed);
        let n2 = value_noise_2d(x as f32 * 0.01, z as f32 * 0.01, self.seed.wrapping_add(999));
        (base + n1 * 16.0 + n2 * 12.0) as i32
    }
}

fn hash(mut x: i32, mut y: i32, seed: u32) -> f32 {
    x = x.wrapping_mul(374761393).wrapping_add(seed as i32);
    y = y.wrapping_mul(668265263).wrapping_add((seed >> 16) as i32);
    let mut n = x ^ y;
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    ((n ^ (n >> 16)) & 0x7fffffff) as f32 / 0x7fffffff as f32
}

fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn value_noise_2d(x: f32, z: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let z0 = z.floor() as i32;
    let x1 = x0 + 1;
    let z1 = z0 + 1;
    let tx = smoothstep(x - x0 as f32);
    let tz = smoothstep(z - z0 as f32);

    let v00 = hash(x0, z0, seed);
    let v10 = hash(x1, z0, seed);
    let v01 = hash(x0, z1, seed);
    let v11 = hash(x1, z1, seed);

    lerp(lerp(v00, v10, tx), lerp(v01, v11, tx), tz)
}
