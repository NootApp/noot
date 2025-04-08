use std::collections::BTreeMap;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct AssetManager {
    assets: BTreeMap<String, CacheEntry>,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager { assets: BTreeMap::new() }
    }
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub expires: Instant,
    internal: Asset,
}

impl CacheEntry {
    pub fn new(expires: u64, asset: Asset) -> Self {
        let now = Instant::now();
        let future = now.checked_add(Duration::from_secs(expires)).unwrap();

        CacheEntry {
            expires: future,
            internal: asset
        }
    }

    pub fn valid(&self) -> bool {
        self.expires <= Instant::now()
    }

    pub fn extract(&self) -> &Asset {
        &self.internal
    }
}



#[derive(Clone, Debug)]
pub struct Asset {
    pub kind: String,
    pub name: String,
    pub data: AssetData
}


#[derive(Clone, Debug)]
pub enum AssetData {
    Path(String),
    Raw(Vec<u8>)
}