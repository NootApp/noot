use crate::Noot;
use crate::events::types::AppEvent;
use crossbeam_queue::SegQueue;
use iced::Task;

pub fn on_spawn(noot: &mut Noot) -> Task<AppEvent> {
    let config = noot.config.clone().unwrap();

    let mut max_threads = config
        .performance
        .clone()
        .unwrap()
        .max_work_threads
        .unwrap_or(0);

    let num_cpus = num_cpus::get();

    if max_threads > num_cpus {
        max_threads = num_cpus;
    }

    if max_threads == 0 {
        max_threads = 1
    }

    // let queue: SegQueue<> = crossbeam_queue::SegQueue::new();

    Task::none()
}
