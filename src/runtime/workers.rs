use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crossbeam_queue::ArrayQueue;
use iced::futures::channel::mpsc::Sender;
use iced::futures::{SinkExt, Stream};
use iced::futures::channel::mpsc;
use iced::stream;
use rusqlite::Connection;
use walkdir::{DirEntry, WalkDir};
use crate::runtime::{Message, GLOBAL_STATE};
use crate::runtime::workers::jobs::build_tree::build_tree;

pub mod jobs;

#[derive(Debug, Clone)]
pub enum JobType {
    BuildTree(PathBuf),
    PreRender(String),
}

#[derive(Debug, Clone)]
pub struct Job {
    pub(crate) kind: JobType,
    pub(crate) id: String,
}

impl Job {
    pub(crate) fn new(kind: JobType) -> Job {
        let id = nanoid!(5);
        info!("Created job with id {} -> {:?}", id, kind);
        Job {
            id,
            kind
        }
    }
}

pub(crate) fn handle_job(job: Job, id: usize, queue: Arc<ArrayQueue<Job>>, sender: &mut Sender<Message>) {
    dbg!(&job);

    match job.clone().kind {
        JobType::BuildTree(path) => build_tree(job, id, queue, sender, path),
        // JobType::PreRender(file) => {
        //
        // }
        _ => {
            error!("Unable to handle job {:?}", job);
        }
    }
}

pub fn spawn() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        let thread_count = num_cpus::get_physical();

        let (tx, mut rx) = mpsc::channel(thread_count);

        for id in 0..thread_count {
            // Spawn a new worker thread
            let q = GLOBAL_STATE.lock().unwrap().queue.clone();
            let mut sender = tx.clone();

            thread::spawn(move || {
                info!("Worker {} spawned", id);

                loop {
                    let maybe_job = q.pop();

                    if let Some(job) = maybe_job {
                        info!("WORKER-{} | Processing Job {}", id, job.id);
                        handle_job(job, id, q.clone(), &mut sender);
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            });
        }


        loop {
            let try_rx = rx.try_next();

            if let Ok(maybe_msg) = try_rx {
                if let Some(msg) = maybe_msg {
                    let try_tx = output.send(msg).await;
                    if let Err(e) = try_tx {
                        error!("{}", e);
                    }
                }
            }
        }
    })
}