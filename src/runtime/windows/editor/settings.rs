use crate::storage::workspace::{AssetCachingStrategy, RemoteDataStrategy, WorkspaceManager};


#[derive(Debug)]
pub struct EditorSettings {
    plugins: EditorPluginSettings,
    assets: EditorAssetSettings,
}

#[derive(Debug, Copy, Clone)]
pub struct EditorPluginSettings {
    pub enable: bool,
    pub allow_unpacked: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct EditorAssetSettings {
    pub cache_strategy: AssetCachingStrategy,
    pub fetch_remote: RemoteDataStrategy,
}



impl EditorSettings {
    pub fn new() -> Self {
        Self {
            plugins: Default::default(),
            assets: Default::default(),
        }
    }

    pub fn save(&self, mgr: &mut WorkspaceManager) {
        self.plugins.save(mgr);
        self.assets.save(mgr);
    }
}

impl EditorPluginSettings {
    pub fn save(&self, mgr: &mut WorkspaceManager) {
        mgr.set_setting("plugins.enable", self.enable)
            .set_setting("plugins.allow-unpacked", self.allow_unpacked);
    }
}

impl EditorAssetSettings {
    pub fn save(&self, mgr: &mut WorkspaceManager) {
        mgr.set_setting("assets.cache-strategy", self.cache_strategy)
            .set_setting("assets.fetch-remote", self.fetch_remote);
    }
}


impl Default for EditorPluginSettings {
    fn default() -> Self {
        Self {
            enable: false,
            allow_unpacked: false,
        }
    }
}

impl Default for EditorAssetSettings {
    fn default() -> Self {
        Self {
            cache_strategy: AssetCachingStrategy::Blob,
            #[cfg(not(feature = "enterprise"))]
            fetch_remote: RemoteDataStrategy::All,
            #[cfg(feature = "enterprise")]
            fetch_remote: RemoteDataStrategy::None,
        }
    }
}
