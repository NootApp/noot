use std::ffi::OsStr;
use std::path::PathBuf;
use iced::futures::SinkExt;
use iced::window::Id;
use crate::runtime::windows::editor::messaging::{EditorMessage, EditorMessageKind};
use crate::runtime::workers::JobList;
use super::super::{Job, Worker};
use crate::storage::workspace::buffer::Buffer;

pub async fn pre_render(_job: Job, worker: &mut Worker, window: Id, path: PathBuf) -> Option<JobList> {
    let jobs = JobList::new();
    if path.exists() {
        worker.info(format!("Attempting to pre-render file at {}", path.display()));
        let content = std::fs::read_to_string(path.clone()).unwrap();


        let buffer = Buffer::from_md(path.file_name().unwrap_or(OsStr::new("?UNKNOWN_file?")).to_str().unwrap().to_string(), format!("file://{}", path.to_str().unwrap().to_string()), content);
        worker.info("pre-render completed");
        worker.sender.send(EditorMessage::new(EditorMessageKind::BufferRendered(buffer), window).into()).await.unwrap();
        worker.info("Update triggered");
    } else {
        worker.error("Cannot pre-render file which does not exist");
    }

    Some(jobs)
}