use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub duplicates: Vec<Vec<String>>,
}

impl Report {
    pub fn new(dups: Vec<Vec<String>>) -> Self {
        Report { duplicates: dups }
    }

    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Serialize error");
        fs::write(path, json).expect("Write error");
    }

    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Read error");
        serde_json::from_str(&data).expect("Deserialize error")
    }
}
