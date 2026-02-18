use std::time::Instant;

pub struct Time {
    last: Instant,
    delta: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            delta: 1.0 / 60.0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta = (now - self.last).as_secs_f32().clamp(0.0001, 0.1);
        self.last = now;
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta
    }
}
