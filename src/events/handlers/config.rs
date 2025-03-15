use crate::events::types::{EventQueue, AppEvent};
use crate::filesystem::config::Config;
use crate::filesystem::utils::traits::{
    Configuration, list_validation_results,
};
use crate::{Noot, ViewPort, subsystems, views};
use iced::{window, Task};
use iced::futures::executor::block_on;

pub fn on_load(noot: &mut Noot, cfg: Config) -> Task<AppEvent> {
    let outcomes = cfg.validate("");
    let (outcome, exiting) = list_validation_results(outcomes);
    let mut queue = EventQueue::new();

    let mut tasks: Vec<Task<AppEvent>> = vec![outcome];

    if exiting {
        return Task::batch(tasks);
    }

    let save_outcome = cfg.save_to_disk();

    if save_outcome.is_err() {
        error!("Error saving config file to disk");
    }

    info!("Config loaded");
    noot.config = Some(cfg.clone());

    let _outcome =
        subsystems::cryptography::keys::perform_startup_checks().unwrap();

    debug!("Config load finished...");

    
    queue.add(AppEvent::WorkspaceIngestManifests);
    queue.add(AppEvent::RPCInit);
    queue.add(AppEvent::TPSpawn);
    tasks.push(queue.drain(noot));
    
    Task::batch(tasks)
}
