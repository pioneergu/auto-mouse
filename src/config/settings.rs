use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub interval_seconds: f64,
    pub move_distance: i32,
    pub enable_sound: bool,
    pub start_minimized: bool,

    // 타이머 설정
    pub enable_timer: bool,
    pub timer_minutes: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_seconds: 30.0,
            move_distance: 10,
            enable_sound: false,
            start_minimized: false,
            enable_timer: false,
            timer_minutes: 60,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let settings: Settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // 디렉토리 생성
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("설정 디렉토리를 찾을 수 없습니다"))?;
        path.push("auto-mouse");
        path.push("config.json");
        Ok(path)
    }
}
