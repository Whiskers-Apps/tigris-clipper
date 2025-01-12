use core::fmt;
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Database {
    pub clips: Vec<Clip>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Clip {
    pub id: usize,
    pub keyword: String,
    pub name: String,
    pub content: String,
    pub clip_type: ClipType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClipType {
    Text,
    TextArea,
    Image,
}

impl Clip {
    pub fn new(keyword: &str, name: &str, content: &str, clip_type: ClipType) -> Self {
        let db = Database::get_db();

        let id: usize = if db.clips.is_empty() {
            0
        } else {
            db.clips.iter().map(|clip| clip.id).max().unwrap() + 1
        };

        Self {
            id,
            keyword: keyword.to_owned(),
            name: name.to_owned(),
            content: content.to_owned(),
            clip_type,
        }
    }
}

impl fmt::Display for ClipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Database {
    pub fn get_db() -> Self {
        let config_dir = get_config_dir();
        let db_path = get_db_path();

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect("Error creating config directory");
        }

        if !db_path.exists() {
            let db = Database { clips: vec![] };

            db.save();

            return db;
        }

        let db_json = fs::read_to_string(&db_path).expect("Error reading db");
        let db = serde_json::from_str::<Database>(&db_json).expect("Error deserializing db");

        db
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
    }

    pub fn save(&self) {
        let json = serde_json::to_string(self).expect("Error creating json");
        fs::write(get_db_path(), &json).expect("Error writing db");
    }
}

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("tigris-clipper")
}

pub fn get_db_path() -> PathBuf {
    get_config_dir().join("db.json")
}
