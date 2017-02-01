use errors::*;

use serde_yaml;

use std::fs::File;

#[derive(Debug,Deserialize)]
pub enum PlanetKind {
    RED { ring: f64, strength: f64 },
    BLUE { ring: f64, strength: f64 },
    WHITE { ring: f64, strength: f64 },
}

#[derive(Debug,Deserialize)]
pub struct ObjectData {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug,Deserialize)]
pub struct PlanetData {
    pub x: i32,
    pub y: i32,
    pub kind: PlanetKind,
}

#[derive(Debug,Deserialize)]
pub struct LevelData {
    pub meteor: ObjectData,
    pub stars: Vec<ObjectData>,
    pub planets: Vec<PlanetData>,
}

impl LevelData {
    pub fn load(path: &'static str) -> Result<LevelData> {
        let f = File::open(path)?;
        Ok(serde_yaml::from_reader(&f)?)
    }
}
