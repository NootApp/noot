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
    pub manifest: manifest::PluginManifest,
    pub scope: scopes::PluginScopes
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

    //pub fn load_plugins() {}
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