use std::{collections::HashMap, fs, path::PathBuf};

use super::{chunk::Chunk, generator::WorldGen};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WorldMeta {
    pub name: String,
    pub seed: u32,
    pub mode: String,
    pub generator: WorldGen,
}

pub fn world_dir(name: &str) -> PathBuf {
    PathBuf::from("worlds").join(name)
}

pub fn save_meta(meta: &WorldMeta) -> Result<(), String> {
    let dir = world_dir(&meta.name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let data = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
    fs::write(dir.join("meta.json"), data).map_err(|e| e.to_string())
}

pub fn load_meta(name: &str) -> Result<WorldMeta, String> {
    let data = fs::read_to_string(world_dir(name).join("meta.json")).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

pub fn save_modified_chunks(name: &str, chunks: &HashMap<(i32, i32), Chunk>) -> Result<(), String> {
    let dir = world_dir(name).join("chunks");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for ((cx, cz), chunk) in chunks {
        let path = dir.join(format!("{}_{}.json", cx, cz));
        let data = serde_json::to_vec(chunk).map_err(|e| e.to_string())?;
        fs::write(path, data).map_err(|e| e.to_string())?;
    }
    Ok(())
}
