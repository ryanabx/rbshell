#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub favorites: Vec<String>,
}

impl<'a> Default for PanelConfig {
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
