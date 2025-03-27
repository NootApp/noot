use sha2::{Digest, Sha256, Sha512};

/// Helper method to hash a string using sha256 and return the output in hex encoded text
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:02x}", result)
}

/// Helper method to has a string using sha512 and return the output in hex encoded text
pub fn sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:02x}", result)
}