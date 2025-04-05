use std::collections::BTreeSet;
use mlua::Lua;


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
