use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crossbeam_queue::ArrayQueue;
use iced::futures::channel::mpsc::Sender;
use iced::futures::{SinkExt, Stream, StreamExt};
use iced::futures::channel::mpsc;
use iced::stream;
use iced_core::window::Id;
use url::Url;
use crate::runtime::{Message, GLOBAL_STATE};
use crate::runtime::workers::jobs::build_tree::build_tree;
use crate::runtime::workers::jobs::cache_assets::cache_assets;
use crate::runtime::workers::jobs::pre_render::pre_render;

pub type JobResult<T> = Result<T, JobError>;

pub struct JobError(String);

pub type JobList = Vec<Job>;

impl JobError {
    pub fn new<R: Into<String>>(reason: R) -> Self {
        Self (reason.into())
    }
}

pub struct Worker {
    pub id: usize,
    pub queue: Arc<ArrayQueue<Job>>,
    pub sender: Sender<Message>,
    pub job_id: String,
}


impl Worker {
    pub fn new(id: usize, queue: Arc<ArrayQueue<Job>>, sender: Sender<Message>) -> Worker {
        Worker {
            id,
            queue,
            sender,
            job_id: "Idle".to_string(),
        }
    }

    pub async fn start(&mut self) {
        self.info("Started");
        loop {
            let maybe_job = self.queue.pop();

            if let Some(job) = maybe_job {
                self.info(format!("Processing Job {}", job.id));
                self.handle_job(job).await;
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn handle_job(&mut self, job: Job) {
        let maybe_jobs = match job.clone().kind {
            JobType::BuildTree(path, source, pre_render) => build_tree(job, self, source, path, pre_render).await,
            JobType::PreRender(path, source) => pre_render(job, self, source, path).await,
            JobType::CacheAsset(path, url, source, buffer) => cache_assets(job, self, source, path, url, buffer).await,
        };

        if let Some(jobs) = maybe_jobs {
            for job in jobs {
                let jid = job.id.clone();
                let enqueued = self.queue.push(job);

                if enqueued.is_ok() {
                    self.info(format!("Queued job with id: {}", jid));
                } else {
                    // let err = enqueued.err().unwrap();
                    self.error(format!("Failed to queue job with id: {}", jid));
                }
            }
        }
    }

    fn info<M: Into<String>>(&self, message: M) {
        info!("Worker {} | {}", self.id, message.into());
    }

    fn warn<M: Into<String>>(&self, message: M) {
        warn!("Worker {} | {}", self.id, message.into());
    }

    fn error<M: Into<String>>(&self, message: M) {
        error!("Worker {} | {}", self.id, message.into());
    }
}


pub mod jobs;

/// A generic enum which contains all the possible job types, (and their required metadata).
#[derive(Debug, Clone)]
pub enum JobType {
    /// Requests a worker to walk a directory and build a tree of all the files it finds
    /// Note: This also triggers pre-rendering of any markdown files it locates within the directory.
    /// **Params**
    /// - PathBuf -> The path to use as the root of the tree.
    /// - Id -> The window ID that the tree should be broadcast to when completed.
    /// - bool -> Whether to allow the worker to queue pre-render jobs when indexing.
    BuildTree(PathBuf, Id, bool),

    /// Requests that a worker attempt to preprocess the markdown file into a buffer,
    /// allowing us to manage asset caching transparently in the background
    /// of the application, before we even try to render a source.
    /// **Params**
    /// - PathBuf -> The path of the file we want to pre-render.
    /// - Id -> The window ID that the buffer should be broadcast to when completed.
    PreRender(PathBuf, Id),

    /// Requests that a worker attempt to cache an asset from a file
    /// **Params**
    /// - PathBuf -> The path of the workspace root.
    /// - Url -> The url to use when locating the asset. This allows for local files, cloud files, and internal assets too
    /// - Id -> The window ID that the asset should be broadcast to when completed
    /// - String -> A formatted string containing the buffer ID this asset should be assigned to.
    /// > Note: The assets assigned to a specific buffer ID are not accessible to plugins or other buffers, this is called "enclaving"
    CacheAsset(PathBuf, Url, Id, String),
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

pub fn spawn() -> impl Stream<Item = Message> {
    stream::channel(1, |mut output| async move {
        #[cfg(debug_assertions)]
        let thread_count = 1;
        #[cfg(not(debug_assertions))]
        let thread_count = num_cpus::get_physical();

        let (tx, mut rx) = mpsc::channel(thread_count);

        for id in 0..thread_count {
            // Spawn a new worker thread
            let q = GLOBAL_STATE.lock().unwrap().queue.clone();
            let sender = tx.clone();
            info!("Attempting to spawn worker {}", id);
            thread::spawn(move || {
                info!("Thread for worker {} spawned, starting runtime", id);
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    let mut worker = Worker::new(id, q, sender);
                    worker.start().await;
                });
            });
        }


        loop {
            let maybe_msg = rx.next().await;

            if let Some(msg) = maybe_msg {
                let try_tx = output.send(msg).await;
                if let Err(e) = try_tx {
                    error!("{}", e);
                }
            }
        }
    })
}