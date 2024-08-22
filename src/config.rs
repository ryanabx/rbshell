use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    app_tray_config: AppTrayConfig,
}

impl<'a> Default for PanelConfig {
    fn default() -> Self {
        Self {
            app_tray_config: AppTrayConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppTrayConfig {
    pub favorites: Vec<String>,
}

impl<'a> Default for AppTrayConfig {
    fn default() -> Self {
        Self {
            favorites: vec![
                "com.system76.CosmicTerm".to_string(),
                "org.mozilla.firefox".to_string(),
                "org.kde.discover".to_string(),
            ],
        }
    }
}
