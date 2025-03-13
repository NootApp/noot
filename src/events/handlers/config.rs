use iced::futures::executor::block_on;
use iced::Task;
use crate::events::types::{EventQueue, Message};
use crate::filesystem::config::Config;
use crate::filesystem::utils::traits::{list_validation_results, Configuration};
use crate::{subsystems, views, Noot, ViewPort};

pub fn on_load(noot: &mut Noot, cfg: Config) -> Task<Message> {
    let outcomes = cfg.validate("");
    let (outcome , exiting)= list_validation_results(outcomes);
    let mut queue = EventQueue::new();

    let mut tasks : Vec<Task<Message>> = vec![outcome];

    if exiting {
        return Task::batch(tasks);
    }

    let save_outcome = block_on(cfg.save_to_disk());

    if save_outcome.is_err() {
        error!("Error saving config file to disk");
    }

    info!("Config loaded");
    noot.config = Some(cfg.clone());

    let _outcome =
        subsystems::cryptography::keys::perform_startup_checks().unwrap();




    debug!("Config load finished...");

    queue.add(Message::WorkspaceIngestManifests);
    queue.add(Message::RPCInit);
    queue.add(Message::TPSpawn);
    tasks.push(queue.drain(noot));

    Task::batch(tasks)
}