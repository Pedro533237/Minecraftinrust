#[derive(Clone)]
pub struct Camera {
    pub pos: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: [0.0, 28.0, 0.0],
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn forward(&self) -> [f32; 3] {
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();
        [
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ]
    }

    pub fn right(&self) -> [f32; 3] {
        let f = self.forward();
        [f[2], 0.0, -f[0]]
    }
}
