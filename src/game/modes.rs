#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GameMode {
    Survival,
    Creative,
}

impl GameMode {
    pub fn toggle(&mut self) {
        *self = match self {
            GameMode::Survival => GameMode::Creative,
            GameMode::Creative => GameMode::Survival,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            GameMode::Survival => "SURVIVAL",
            GameMode::Creative => "CRIATIVO",
        }
    }
}
