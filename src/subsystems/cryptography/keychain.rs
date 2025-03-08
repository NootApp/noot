use hmac::{Hmac, Mac};
use keyring::Entry;
use pbkdf2::pbkdf2;
use rand::RngCore;
use secp256k1::constants::SECRET_KEY_SIZE;
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};
use sha2::Sha512;
use zeroize::Zeroize;

const HARDENED_OFFSET: u32 = 0x80000000;

// Use a much lower iteration count on dev builds to provide some security whilst being fast
#[cfg(debug_assertions)]
const PBKDF2_ITERATIONS: u32 = 10_000;

#[cfg(not(debug_assertions))]
const PBKDF2_ITERATIONS: u32 = 310_000;
const SALT: &[u8] = include_bytes!("../../../salt-secret.bin");



/// Represents an extended secret key with a chain code.
#[derive(Debug)]
pub struct ExtendedSecretKey {
    pub(crate) secret_key: [u8; SECRET_KEY_SIZE],
    pub(crate) chain_code: [u8; 32],
}

impl ExtendedSecretKey {
    /// Creates a new `ExtendedSecretKey`.
    fn new(secret_key: [u8; SECRET_KEY_SIZE], chain_code: [u8; 32]) -> Self {
        Self {
            secret_key,
            chain_code,
        }
    }
}

impl Zeroize for ExtendedSecretKey {
    fn zeroize(&mut self) {
        self.secret_key.zeroize();
        self.chain_code.zeroize();
    }
}

impl Drop for ExtendedSecretKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}



// !=========== CAUTION ============!
// All code below this point was sourced from a blogpost on medium.com
// Found here: https://medium.com/@evadawnleycoding/building-a-secure-hierarchical-key-derivation-system-in-rust-b5a0ecee18d7
//
// I do not know enough about cryptography to determine whether this code is even remotely secure
// I do not have the time at this moment to figure out if it is following good practices or not
// I will not take responsibility for any data compromised as a result of inadequate protections offered by the code below
// USE AT YOUR OWN PERIL

/// Derives the master extended secret key using HMAC-SHA-512.
///
/// # Arguments
/// - `seed`: A secure random seed (16 to 64 bytes).
pub fn derive_master_extended_secret_key(
    seed: &[u8],
) -> Result<ExtendedSecretKey, &'static str> {
    let mut master_seed_bytes = [0u8; 64];
    let _ = pbkdf2::<Hmac<Sha512>>(
        seed,
        SALT,
        PBKDF2_ITERATIONS,
        &mut master_seed_bytes,
    );

    if master_seed_bytes.len() != 64 {
        return Err("Seed must be exactly 64 bytes");
    }

    let mut mac = Hmac::<Sha512>::new_from_slice(b"Crypto seed")
        .map_err(|_| "HMAC initialization failed")?;
    mac.update(&master_seed_bytes);
    let result = mac.finalize().into_bytes();

    let (secret_key_bytes, chain_code_bytes) = result.split_at(SECRET_KEY_SIZE);

    let mut secret_key = [0u8; SECRET_KEY_SIZE];
    let mut chain_code = [0u8; 32];

    secret_key.copy_from_slice(secret_key_bytes);
    chain_code.copy_from_slice(chain_code_bytes);

    // Validate the derived secret key
    SecretKey::from_slice(&secret_key).map_err(|_| {
        secret_key.zeroize();
        chain_code.zeroize();
        "Invalid secret key derived"
    })?;

    Ok(ExtendedSecretKey::new(secret_key, chain_code))
}

/// Derives a child extended secret key.
///
/// # Arguments
/// - `parent`: The parent extended secret key.
/// - `index`: The index of the child key.
/// - `hardened`: Whether the derivation is hardened.
fn derive_child_extended_secret_key(
    parent: &ExtendedSecretKey,
    index: u32,
    hardened: bool,
) -> Result<ExtendedSecretKey, &'static str> {
    if hardened && index < HARDENED_OFFSET {
        return Err("Hardened derivation requires index >= 0x80000000");
    }

    let mut mac = Hmac::<Sha512>::new_from_slice(&parent.chain_code)
        .map_err(|_| "HMAC initialization failed")?;

    if hardened {
        mac.update(&[0x00]);
        mac.update(&parent.secret_key);
    } else {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&parent.secret_key)
            .map_err(|_| "Invalid parent secret key")?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        mac.update(&public_key.serialize());
    }

    mac.update(&index.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let (secret_key_tweak_bytes, chain_code_bytes) =
        result.split_at(SECRET_KEY_SIZE);

    let mut secret_key_tweak = [0u8; SECRET_KEY_SIZE];
    let mut new_chain_code = [0u8; 32];

    secret_key_tweak.copy_from_slice(secret_key_tweak_bytes);
    new_chain_code.copy_from_slice(chain_code_bytes);

    // Add tweak to parent secret key
    let tweak = Scalar::from_be_bytes(secret_key_tweak)
        .map_err(|_| "Invalid tweak value")?;
    let parent_sk = SecretKey::from_slice(&parent.secret_key)
        .map_err(|_| "Invalid parent secret key")?;

    let child_sk = parent_sk
        .add_tweak(&tweak)
        .map_err(|_| "Invalid resulting secret key")?;

    Ok(ExtendedSecretKey::new(
        child_sk.secret_bytes(),
        new_chain_code,
    ))
}

/// Derives a child public key from an extended public key.
///
/// # Arguments
/// - `parent_pubkey`: The parent public key.
/// - `chain_code`: The chain code associated with the parent.
/// - `index`: The index of the child key.
fn derive_child_public_key(
    parent_pubkey: &PublicKey,
    chain_code: &[u8],
    index: u32,
) -> Result<PublicKey, &'static str> {
    if index >= HARDENED_OFFSET {
        return Err("Cannot derive hardened key from public key");
    }

    let mut mac = Hmac::<Sha512>::new_from_slice(chain_code)
        .map_err(|_| "HMAC initialization failed")?;
    mac.update(&parent_pubkey.serialize());
    mac.update(&index.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let (key_tweak_bytes, _new_chain_code) = result.split_at(SECRET_KEY_SIZE);

    let key_tweak_array: [u8; 32] = key_tweak_bytes
        .try_into()
        .map_err(|_| "Invalid tweak size")?;
    let tweak = Scalar::from_be_bytes(key_tweak_array)
        .map_err(|_| "Invalid tweak value")?;

    let secp = Secp256k1::new();
    let child_pubkey = parent_pubkey
        .add_exp_tweak(&secp, &tweak)
        .map_err(|_| "Invalid resulting public key")?;

    Ok(child_pubkey)
}