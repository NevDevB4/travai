use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            x: 100,
            y: 100,
            width: 1400,
            height: 900,
            is_maximized: false,
        }
    }
}

pub struct WindowStateManager;

impl WindowStateManager {
    fn get_state_path() -> PathBuf {
        let app_data = dirs::data_local_dir().expect("Failed to get app data directory");
        app_data.join("Travai").join("window_state.json")
    }

    pub fn load() -> WindowState {
        let path = Self::get_state_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str(&content) {
                    return state;
                }
            }
        }
        WindowState::default()
    }

    pub fn save(state: &WindowState) -> Result<(), String> {
        let path = Self::get_state_path();
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

        let json = serde_json::to_string(state).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
