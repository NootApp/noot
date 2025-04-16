use std::collections::BTreeSet;
use mlua::Lua;
use crate::plugins::manifest::PluginManifest;
use crate::plugins::scopes::PluginScopes;

pub mod manifest;
pub mod scopes;
pub mod runtime;

pub enum PluginState {
    NotLoaded,
    Loaded,
    Errored(String)
}

pub struct Plugin {
    pub state: PluginState,
    pub rt: Lua,
    pub manifest: PluginManifest,
    pub scope: PluginScopes
}

pub struct PluginManager {
    pub plugins: BTreeSet<Plugin>
}



impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Default::default()
        }
    }

    pub fn load_plugins(&mut self, path: &str) {
        let entries = std::fs::read_dir(path).unwrap();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    continue;
                } else {

                }
            }
        }
    }
}


impl Plugin {
    pub fn new(manifest: PluginManifest) -> Self {
        let rt = Lua::new();
        Self {
            state: PluginState::NotLoaded,
            rt,
            manifest,
            scope: PluginScopes::empty(),
        }
    }
}