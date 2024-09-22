use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::freedesktop::icons::IconTheme;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO: {0}")]
    IO(#[from] io::Error),
    #[error("Serializing: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub app_tray: AppTrayConfig,
    pub icon_theme: IconTheme,
    pub use_winit: bool,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            app_tray: Default::default(),
            icon_theme: Default::default(),
            use_winit: Default::default(),
        }
    }
}

impl PanelConfig {
    pub fn from_file_or_default(path: &Path) -> Self {
        File::open(path)
            .map_err(ConfigError::IO)
            .and_then(|mut res| {
                let mut data = String::new();
                res.read_to_string(&mut data)
                    .map(|_| data)
                    .map_err(ConfigError::IO)
            })
            .and_then(|val| -> Result<Self, ConfigError> {
                serde_json::from_str(&val).map_err(ConfigError::Serde)
            })
            .unwrap_or_default()
    }

    pub fn _save_to_file(&self, path: &Path) -> Result<(), ConfigError> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppTrayConfig {
    pub favorites: Vec<String>,
}

impl<'a> Default for AppTrayConfig {
    fn default() -> Self {
        Self {
            favorites: vec!["org.mozilla.firefox".to_string()],
        }
    }
}
