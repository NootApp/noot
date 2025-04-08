use std::path::PathBuf;
use walkdir::WalkDir;
use iced::window::Id;
use crate::runtime::workers::{Job, JobList, JobType, Worker};

pub async fn build_tree(_job: Job, worker: &mut Worker, window: Id, path: PathBuf, pre_render: bool) -> Option<JobList> {
    let mut jobs = JobList::new();

    // Build a file tree from the given path
    for entry in WalkDir::new(&path) {
        match entry {
            Ok(e) => {
                // let ft = e.file_type();
                let name = e.file_name().to_string_lossy().to_string();

                if name.ends_with("md") {
                    if pre_render {
                        worker.info(format!("Queuing job to render file {}", path.display()));
                        jobs.push(Job::new(JobType::PreRender(e.path().to_path_buf(), window)))
                    }
                } else {
                    // File is not markdown, we probably shouldn't try to pre-render it.
                    // dbg!(&e);
                }
            },
            Err(e) => {
                worker.error(e.to_string());
            }
        }
    }

    Some(jobs)
}