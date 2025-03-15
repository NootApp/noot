use crate::events::types::{EventQueue, AppEvent};
use crate::filesystem::workspace::manager::{MANAGER, WorkspaceResult};
use crate::filesystem::workspace::state::WorkspaceState;
use crate::views::landing::LandingView;
use crate::{Noot, ViewPort};
use iced::Task;
use iced::futures::executor::block_on;

pub fn on_load(
    noot: &mut Noot,
    outcome: WorkspaceResult<WorkspaceState>,
) -> Task<AppEvent> {
    debug!("Workspace load event triggered");

    if let Ok(state) = outcome {
        
        noot.viewport = ViewPort::WorkspaceView(state);
    } else {
        error!("Workspace load failed");
        error!("{:?}", outcome.unwrap_err());
    }

    Task::none()
}

pub fn on_ingest(noot: &mut Noot) -> Task<AppEvent> {
    let mut mgr = MANAGER.lock().unwrap();
    let config = noot.config.clone();

    if let Some(cfg) = config {
        mgr.ingest_config(cfg.workspaces.unwrap_or_default());
        drop(mgr);
        // Trigger workspace load event to start loading the
        // previous workspace if one is present
        noot.update(AppEvent::WorkspaceLoadStart)
    } else {
        panic!("Cannot ingest config whilst config is not initialized");
    }
}

pub fn on_load_start(noot: &mut Noot) -> Task<AppEvent> {
    debug!("Workspace load event triggered");
    let mut mgr = MANAGER.lock().unwrap();
    let mut queue = EventQueue::new();
    let cfg = noot.config.clone().unwrap();

    if let Some(previous_workspace) = cfg.last_open {
        debug!("Previous workspace found");
        let load_outcome = mgr.load_workspace(previous_workspace);
        let outcome = block_on(load_outcome);
        queue.add(AppEvent::WorkspaceLoadResult(outcome));
    } else {
        debug!("No previous workspace found - Showing landing view");
        noot.viewport = ViewPort::LandingView(LandingView::new());
    }

    queue.drain(noot)
}
