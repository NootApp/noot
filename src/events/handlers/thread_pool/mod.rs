use crossbeam_queue::SegQueue;
use iced::Task;
use crate::events::types::Message;
use crate::Noot;


pub fn on_spawn(noot: &mut Noot) -> Task<Message> {
    let config = noot.config.clone().unwrap();
    
    let mut max_threads = config.performance.clone().unwrap().max_work_threads.unwrap_or(0);
    
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