//use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub mod types;
pub mod filesystem;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load the config file
    let config = types::config::Config::load_from_disk();

    if config.workspace == "NONE" {
        log::info!("Workspace unset - Maybe Fresh Install");
    } else {
        log::info!("Workspace set - {}", config.workspace);
    }

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

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build());

    builder = builder
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
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

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
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
