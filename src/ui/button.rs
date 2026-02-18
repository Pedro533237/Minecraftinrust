#[derive(Clone)]
pub struct Button {
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Button {
    pub fn new(label: impl Into<String>, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            label: label.into(),
            x,
            y,
            w,
            h,
        }
    }

    pub fn contains(&self, mx: f32, my: f32) -> bool {
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }
}
