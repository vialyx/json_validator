use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub name: String,
    pub age: u8,
    pub email: String,
}

// Sanitize input: remove control chars, trim spaces
pub fn sanitize_input(raw: &str) -> String {
    let control_chars = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
    let sanitized = control_chars.replace_all(raw, "");
    sanitized.trim().to_string()
}

// Parse JSON string into UserData
pub fn parse_json(data: &str) -> Result<UserData, String> {
    let sanitized = sanitize_input(data);
    serde_json::from_str::<UserData>(&sanitized).map_err(|e| e.to_string())
}

// Load JSON from file
pub fn load_json_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}