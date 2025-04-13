use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;
use iced::futures::SinkExt;
use iced::window::Id;
use url::Url;
use crate::runtime::windows::editor::messaging::{EditorMessage, EditorMessageKind};
use crate::runtime::workers::{JobList, JobType};
use super::super::{Job, Worker};
use crate::storage::workspace::buffer::{Buffer, ElWrapper};

pub async fn pre_render(_job: Job, worker: &mut Worker, workspace: String, window: Id, path: PathBuf) -> Option<JobList> {
    if path.exists() {
        worker.info(format!("Attempting to pre-render file at {}", path.display()));
        let content = std::fs::read_to_string(path.clone()).unwrap();


        let buffer = Buffer::from_md(path.file_name().unwrap_or(OsStr::new("?UNKNOWN_file?")).to_str().unwrap().to_string(), workspace.clone(), format!("file://{}", path.to_str().unwrap().to_string()), content);

        let images = find_image(buffer.id.clone(), &buffer.doc, worker, workspace, window, path);



        worker.info("pre-render completed");
        worker.sender.send(EditorMessage::new(EditorMessageKind::BufferRendered(buffer), window).into()).await.unwrap();
        worker.info("Update triggered");
        Some(images)
    } else {
        worker.error("Cannot pre-render file which does not exist");
        None
    }


}


fn find_image(bid: String, els: &[ElWrapper], worker: &mut Worker, workspace: String, window: Id, path: PathBuf) -> JobList {
    let mut image_jobs = JobList::new();

    for el in els {
        if el.name == "img" {
            image_jobs.push(
                Job::new(
                    JobType::CacheAsset(workspace.clone(), path.clone(), Url::from_str(el.attributes.get("src").unwrap().clone().unwrap().as_str()).unwrap(), window, bid.clone())
                )
            )
        } else if el.children.len() > 0 {
            image_jobs.append(&mut find_image(bid.clone(), &el.children, worker, workspace.clone(), window, path.clone()));
        }
    }

    image_jobs
}