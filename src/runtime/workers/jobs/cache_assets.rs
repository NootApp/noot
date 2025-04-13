use std::path::PathBuf;
use iced::futures::SinkExt;
use iced_core::window::Id;
use rusqlite::fallible_iterator::FallibleIterator;
use url::Url;
use crate::runtime::windows::editor::messaging::{EditorMessage, EditorMessageKind};
use crate::runtime::workers::{Job, JobList, Worker};
use crate::utils::cryptography::hashing::hash_str;

pub async fn cache_assets(_job: Job, worker: &mut Worker, window: Id, workspace: String, path: PathBuf, url: Url, buffer: String) -> Option<JobList> {
    let hash = hash_str(format!("{}", url));
    let mut root = path.clone();
    root.pop();

    let asset_dir = root.join(".assets");

    if !asset_dir.exists() {
        std::fs::create_dir_all(&asset_dir).unwrap();
    }

    let asset_path = asset_dir.join(&hash);

    match url.scheme() {
        "http" | "https" => {
            let res = reqwest::get(&url.to_string()).await;

            if let Ok(res) = res {
                let write = tokio::fs::write(asset_path, res.bytes().await.unwrap().as_ref()).await;
                if let Err(err) = write {
                    error!("Failed to cache asset");
                    error!("{}", err);
                }
                worker.sender.send(EditorMessage::new(EditorMessageKind::Tick, window).into()).await.unwrap();
            } else {
                error!("Failed to get asset url: {}", url);
                error!("{}", res.unwrap_err())
            }
        },
        s => {
            error!("Cannot cache asset. Reason: Unsupported URL scheme: {}", s);
        }
    }

    None
}