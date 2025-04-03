use sha2::{Sha512, Digest};

pub fn hash_str<S: Into<String>>(text: S) -> String {
    let mut hasher = Sha512::new();

    hasher.update(text.into());

    format!("{:x}", hasher.finalize())
}
