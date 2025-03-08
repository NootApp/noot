use std::path::PathBuf;
use keyring::Entry;
use rand::RngCore;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use crate::subsystems::cryptography::keychain::derive_master_extended_secret_key;

/// check for the presence of the primary encryption key within the OS secure storage
/// on macOS this may be the M series "Secure Enclave"
/// on Windows this could be the TPM
/// on Linux this is provided by the OS itself
pub fn perform_startup_checks() -> keyring::Result<()> {
    debug!("Performing cryptography startup checks");

    let primary = Entry::new("com.nootapp.behring", "emil_von.primary")?;
    let primary_chain = Entry::new("com.nootapp.behring", "emil_von.chain")?;
    let public_key = Entry::new("com.nootapp.behring", "emil_von.public")?;
    
    debug!("Primary entry: {:?}", primary);

    if primary.get_secret().is_err() {
        let err = primary.get_secret().unwrap_err();

        if err.to_string()
            == String::from("No matching entry found in secure storage")
        {
            debug!("No matching entry found in secure storage");
            debug!("Generating new master cryptographic key");

            let mut seed: [u8; 32] = [0; 32];
            let mut rng = rand::rng();
            rng.fill_bytes(&mut seed);

            let master_secret_key_outcome =
                derive_master_extended_secret_key(&seed);

            if !master_secret_key_outcome.is_ok() {
                error!("{:?}", master_secret_key_outcome.unwrap_err());
                panic!("Failed to generate master key");
            }

            debug!("Generated new master key");

            let master_key = master_secret_key_outcome.unwrap();

            debug!("Generating new master public key");
            let secp = Secp256k1::new();
            let master_pubkey = PublicKey::from_secret_key(
                &secp,
                &SecretKey::from_byte_array(&master_key.secret_key).unwrap(),
            );

            primary.set_secret(&master_key.secret_key)?;
            primary_chain.set_secret(&master_key.chain_code)?;

            debug!("Credentials successfully stored");

            debug!("Public key: {:?}", master_pubkey);
        } else {
            error!("{:?}", err.to_string());
            return Err(err.into());
        }
    }

    // let secret = Entry::new("com.nootapp.roentgen", "wilhelm")?;
    // 
    // if secret.get_secret().is_err() {
    //     let err = secret.get_secret().unwrap_err();
    //     
    //     if err.to_string() == String::from("No matching entry found in secure storage") {
    //         debug!("No matching entry found in secure storage");
    //         debug!("Generating new farnsworth value");
    //         let mut seed: [u8; 32] = [0; 32];
    //         let mut rng = rand::rng();
    //         rng.fill_bytes(&mut seed);
    //         
    //         let primary = 
    //         
    //         let p2 = 
    //     }
    // }
    
    Ok(())
}


pub fn find_primary_key() -> keyring::Result<Vec<u8>> {
    debug!("Finding primary key");
    let primary = Entry::new("com.nootapp.behring", "emil-von.primary")?;
    primary.get_secret()
}
// 
// pub fn derive_public_key(file: &PathBuf) -> keyring::Result<PublicKey> {
//     
// }