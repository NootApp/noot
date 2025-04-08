use std::path::PathBuf;
use std::sync::Arc;
use crossbeam_queue::ArrayQueue;
use iced::futures::channel::mpsc::Sender;
use walkdir::WalkDir;
use crate::runtime::Message;
use crate::runtime::workers::Job;

pub fn build_tree(job: Job, id: usize, queue: Arc<ArrayQueue<Job>>, _sender: &mut Sender<Message>, path: PathBuf) {
    // Build a file tree from the given path
    for entry in WalkDir::new(&path) {
        match entry {
            Ok(e) => {
                dbg!(e);
                let ft = e.file_type();
                let name = e.file_name().to_string_lossy().to_string();

            },
            Err(e) => {
                error!("WORKER-{} | {} |{}", id, job.id, e);
            }
        }
    }
}