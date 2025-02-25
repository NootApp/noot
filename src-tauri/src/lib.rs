//use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use tauri::Manager;

pub mod types;
pub mod utils;
pub mod filesystem;
pub mod rpc;
pub mod workspace;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build());

    builder = builder
        .plugin(tauri_plugin_log::Builder::new().build())
        //.plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init());

    log::info!("Noot is ready to launch...");
    
    rpc::start_rpc_thread();

    builder
        .invoke_handler(tauri::generate_handler![types::config::get_app_config, rpc::start_rich_presence, rpc::set_rich_presence_activity])
        .setup(|app| {
            // NOTE: Tray icons seem to cause a panic on my system
            //let tray = TrayIconBuilder::new()
            //    .icon(app.default_window_icon().unwrap().clone())
            //    .build(app)?;
            
            let webview = app.get_webview_window("main").unwrap();

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                webview.open_devtools();
            }

            webview.eval("console.log('Hey there, you\\'re in dangerous waters, if somebody asked you to put something in here, it may be a scam!')");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



