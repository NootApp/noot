use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
//use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::thread;


pub mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    



    //thread::spawn(|| {
    //    log::debug!("Spawning RPC thread");
    //    let mut client = DiscordIpcClient::new("1343225099834101810").unwrap();
    //
    //    client.connect().unwrap();
    //    client.set_activity(activity::Activity::new()
    //        .state("Idle")
    //        .details("bar")
    //        .timestamps(activity::Timestamps::new().start(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()))
    //    ).unwrap();
    //    //client.close()?;
    //});

    // This should be called as early in the execution of the app as possible
    //#[cfg(debug_assertions)] // only enable instrumentation in development builds
    //let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default().plugin(tauri_plugin_store::Builder::new().build());
    //log::info!("Noot, starting...");
    //#[cfg(debug_assertions)]
    //{
    //    log::debug!("Looks like this is a dev build, enabling debugger!");
    //    builder = builder.plugin(devtools);
    //}

    builder = builder
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        //.plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        //.plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_store::Builder::default().build())
        //.plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_positioner::init())
        //.plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_os::init())
        //.plugin(tauri_plugin_http::init())
        //.plugin(tauri_plugin_autostart::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        //.plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init())
        //.plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init());

    log::info!("Noot is ready to launch...");

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // NOTE: Tray icons seem to cause a panic on my system
            //let tray = TrayIconBuilder::new()
            //    .icon(app.default_window_icon().unwrap().clone())
            //    .build(app)?;
            //    #[cfg(debug_assertions)] // only include this code on debug builds
            //    {
            //        app.g
            //        let window = app.get_webview_window("main").unwrap();
            //        window.open_devtools();
            //    }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
