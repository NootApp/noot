use std::collections::BTreeMap;
use std::io::BufRead;
use std::str::FromStr;
use lazy_static::lazy_static;
use url::Url;
use crate::subsystems::cryptography::hash::sha256;

#[derive(Debug, Clone)]
pub struct MediaAsset {
    pub mime: String,
    pub size: usize,
    pub url: String,
    pub locked: bool,

    // Metadata hashing is not foolproof and can be bypassed easily.
    // This is only used to check sanity before accessing a file.
    pub meta_hash: String,
}

impl MediaAsset {
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        match self.url.as_str() {
            "noot://internal/landing" => Ok(LANDING_VIEW_BYTES.to_vec()),
            "noot://internal/test" => Ok(TEST_VIEW_BYTES.to_vec()),
            _ => {
                let local_url= Url::from_str(self.url.as_str()).unwrap();
                let local_path = local_url.to_file_path().unwrap();
                std::fs::read(&local_path)
            }
        }

    }
}

// impl MediaAsset<'_> {
//     pub fn new(url: String) -> Self {
//         let handle
//     }
// }

pub type MediaResult = Result<MediaAsset, MediaError>;

#[derive(Debug)]
pub struct MediaResolver {
    cache: BTreeMap<Url, (MediaResult, bool, std::time::Instant)>
}

const LANDING_VIEW_BYTES: &'static [u8] = include_bytes!("../../../static/experiences/landing.md");
const TEST_VIEW_BYTES: &'static [u8] = include_bytes!("../../../static/experiences/test.md");

lazy_static! {
    static ref LANDING_VIEW_HASH: String = sha256(&String::from_utf8_lossy(LANDING_VIEW_BYTES).to_string());
    static ref LANDING_VIEW: MediaAsset = MediaAsset {
        mime: "text/markdown".to_string(),
        size: LANDING_VIEW_BYTES.len(),
        url: "noot://internal/landing".to_string(),
        meta_hash: LANDING_VIEW_HASH.clone(),
        locked: true,
    };

    static ref TEST_VIEW_HASH: String = sha256(&String::from_utf8_lossy(TEST_VIEW_BYTES).to_string());
    static ref TEST_VIEW: MediaAsset = MediaAsset {
        mime: "text/markdown".to_string(),
        size: TEST_VIEW_BYTES.len(),
        url: "noot://internal/test".to_string(),
        meta_hash: TEST_VIEW_HASH.clone(),
        locked: true,
    };
}


impl MediaResolver {
    pub fn new() -> Self {
        MediaResolver { cache: BTreeMap::new() }
    }

    pub fn purge_caches(&mut self) {
        self.cache.clear();
    }

    pub fn resolve(&mut self, url: impl Into<String>) -> MediaResult {
        let url = Url::from_str(&url.into()).unwrap();

        warn!("MediaResolver failed to locate url {}", url);
        warn!("Scheme: {}", url.scheme());
        warn!("Path: {}", url.path());

        match url.scheme() {
            "http" | "https" => {
                Err(MediaError::MediaNotSupported)
            },
            "noot" => {
                match url.path() {
                    "/landing" => Ok(LANDING_VIEW.clone()),
                    "/test" => Ok(TEST_VIEW.clone()),
                    _ => Err(MediaError::MediaNotFound)
                }
            },
            "file" => {
                Err(MediaError::MediaNotSupported)
            },
            _ => {
                Err(MediaError::MediaNotSupported)
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum MediaError {
    MediaNotFound,
    MediaNotAvailable,
    MediaNotSupported,
}