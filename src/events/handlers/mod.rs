use crate::events::types::AppEvent;
use crate::events::types::AppEvent::*;
use crate::{Noot, events};
use iced::{exit, task, window, Size, Task};
use iced::window::{icon, Icon, Position};
use crate::app::{App, GlobalEvent};
use crate::consts::{APP_ICON, APP_NAME};
use crate::windows::AppWindow;

pub mod config;
pub mod rpc;
pub mod thread_pool;
pub mod workspace;

pub(crate) fn core(noot: &mut Noot, message: AppEvent) -> Task<AppEvent> {
    match message {
        ConfigLoaded(cfg) => config::on_load(noot, cfg),

        WorkspaceIngestManifests => workspace::on_ingest(noot),
        WorkspaceLoadStart => workspace::on_load_start(noot),
        WorkspaceLoadResult(outcome) => workspace::on_load(noot, outcome),

        RPCConnected => rpc::on_connect(),
        RPCDisconnected => rpc::on_disconnect(),
        RPCModified => rpc::on_change(),
        RPCInit => rpc::on_init(noot),


        WindowOpened(id) => {
            info!("Window opened with ID: {}", id);
            Task::none()
        },
        WindowResized(id, dimensions) => {
            info!("Window resized with id: {}, dimensions: {}x{}", id, dimensions.width, dimensions.height);
            Task::none()
        }


        _ => {
            warn!("Received an unknown message payload");
            dbg!(message);
            Task::none()
        }

    }
}


// pub fn handle_global_event(app: &mut App, message: GlobalEvent) -> Task<GlobalEvent> {
//     match message {
//         GlobalEvent::OpenWindow(name) => {
//             match &*name {
//                 "editor" => {
//                     if app.windows.is_empty() {
//                         let (id, open_task) = window::open(window::Settings {
//                             size: Size {
//                                 width: 480.,
//                                 height: 720.,
//                             },
//                             position: Position::Centered,
//                             resizable: true,
//                             decorations: true,
//                             transparent: false,
//                             #[cfg(target_os = "macos")]
//                             platform_specific: PlatformSpecific {
//                                 title_hidden: true,
//                                 titlebar_transparent: true,
//                                 fullsize_content_view: true,
//                             },
//                             #[cfg(target_os = "linux")]
//                             platform_specific: PlatformSpecific {
//                                 application_id: String::from(APP_NAME),
//                                 override_redirect: true,
//                             },
//                             ..Default::default()
//                         });
//
//                         app.windows.insert(
//                             id,
//                             AppWindow::Editor(
//                                 Box::new(
//
//                                 )
//                             )
//                         )
//                     }
//                     Task::none()
//                 },
//                 _ => Task::none(),
//             }
//         }
//         GlobalEvent::ExitApp => exit(),
//         _ => Task::none(),
//     }
// }