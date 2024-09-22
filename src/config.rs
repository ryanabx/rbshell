use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
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

#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub inner: InnerConfig,
    file_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InnerConfig {
    pub app_tray: AppTrayConfig,
    pub icon_theme: Option<IconTheme>,
    pub use_winit: Option<bool>,
}

impl PanelConfig {
    pub fn from_file_or_default(path: &Path) -> Self {
        let inner_res = File::open(path)
            .map_err(ConfigError::IO)
            .and_then(|mut res| {
                let mut data = String::new();
                res.read_to_string(&mut data)
                    .map(|_| data)
                    .map_err(ConfigError::IO)
            })
            .and_then(|val| -> Result<InnerConfig, ConfigError> {
                serde_json::from_str(&val).map_err(ConfigError::Serde)
            });
        match inner_res {
            Ok(_) => {
                log::info!("Successfully loaded config from {}", path.display());
            }
            Err(ref e) => {
                log::warn!("Could not load config from {}: {}", path.display(), e);
            }
        }
        Self {
            inner: inner_res.unwrap_or_default(),
            file_path: path.to_path_buf(),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let data = serde_json::to_string_pretty(&self.inner)?;
        std::fs::write(&self.file_path, data)?;
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
