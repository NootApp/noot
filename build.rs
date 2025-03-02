use std::fs;
use rand::RngCore;
fn main() {

    // Generates secrets used for platform security at compile time
    // These secrets are then stored within the binary and rotated with
// every release to prevent them from being too highly exposed.

    let secret_exists = std::fs::exists("crypto-secret.bin");

    if secret_exists.is_err() {
        eprintln!("{:?}",secret_exists.unwrap_err());
        panic!("secret file check errored");
    }

    if !secret_exists.unwrap() {
        fs::write("crypto-secret.bin", generate_secret()).unwrap();
    }


    let salt_exists = std::fs::exists("salt-secret.bin");
    if salt_exists.is_err() {
        eprintln!("{:?}",salt_exists.unwrap_err());
        panic!("salt file check errored");
    }

    if !salt_exists.unwrap() {
        fs::write("salt-secret.bin", generate_secret()).unwrap();
    }

    println!("Building...");
}


fn generate_secret() -> [u8; 64] {
    let mut rng = rand::rng();
    let mut secret = [0u8; 64];
    rng.fill_bytes(&mut secret);
    secret
}