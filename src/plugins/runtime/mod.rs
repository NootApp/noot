use mlua::{Lua, Variadic};
use crate::plugins::manifest::PluginManifest;

pub fn build_lua_runtime(manifest: PluginManifest) -> Lua {
    let mut lua = Lua::new();
    let mut globals = lua.globals();

    // Create basic runtime functions
    let info = lua.create_function(|_, strings: Variadic<String>| {
        info!("{}", strings.join("\t"));
        Ok(())
    }).unwrap();

    globals.set("info", info).unwrap();

    // if manifest.scopes.contains("theme") {
    //
    // } else {
    //
    // }
    // lua.create_function()

    lua
}




fn register_theme() {

}