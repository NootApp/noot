use crypto::{aead, aead::{AeadCore, AeadInPlace, KeyInit}, ChaCha20Poly1305 as Poly, Key, Nonce};
use crypto::aead::{Aead, OsRng};
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

        if err.to_string() == String::from("No matching entry found in secure storage") {
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

            public_key.set_secret(&master_pubkey.serialize())?;
        } else {
            error!("{:?}", err.to_string());
            return Err(err.into());
        }
    }
    //
    // if symmetric_key.get_secret().is_err() {
    //     debug!("Symmetric key: No matching entry found in secure storage");
    //     let err = symmetric_key.get_secret().unwrap_err();
    //
    //     if err.to_string() == String::from("No matching entry found in secure storage") {
    //         let key = Aes256GcmSiv::generate_key(&mut OsRng);
    //         symmetric_key.set_secret(key.as_slice())?;
    //         let key_str = String::from_utf8_lossy(&key);
    //
    //         debug!("Generated symmetric key: {}", key_str);
    //         info!("Successfully generated symmetric key");
    //     } else {
    //         error!("{}", err);
    //         return Err(err.into());
    //     }
    // }
    //
    // info!("Successfully generated or checked for all expected key values");

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

    dbg!(primary.get_attributes().unwrap());
    
    Ok(())
}


pub fn find_primary_key() -> keyring::Result<Vec<u8>> {
    debug!("Finding primary key");
    let primary = Entry::new("com.nootapp.behring", "emil_von.primary")?;




    primary.get_secret()
}
// 
// pub fn derive_public_key(file: &PathBuf) -> keyring::Result<PublicKey> {
//     
// }

/// Locates (or generates and stores) a symmetric ChaCha20Poly1305 key within the system using the given ID
pub fn find_symmetric_key(id: String, generate: bool) -> keyring::Result<Vec<u8>> {
    debug!("Finding symmetric key");
    let symmetric = Entry::new("com.nootapp.behring", &id)?;

    let secret = symmetric.get_secret();

    if secret.is_err() {
        let err = secret.unwrap_err();

        if err.to_string() == String::from("No matching entry found in secure storage") {
            if generate {
                debug!("Generating new symmetric key for id '{}'", id);
                let key = Poly::generate_key(&mut OsRng);
                symmetric.set_secret(key.as_slice())?;
                return symmetric.get_secret();
            }
            return Err(err.into());
        }
        return Err(err.into());
    }

    secret
}



pub struct CipherError {}

pub struct SymmetricCipher {
    pub key: Key,
    pub cipher: Poly,
    pub nonce: Nonce,
}

impl SymmetricCipher {
    pub fn new(key: &Key) -> Self {
        Self {
            key: key.clone(),
            cipher: Poly::new(&key),
            nonce: Poly::generate_nonce(&mut OsRng),
        }
    }

    pub fn new_from_id(id: &str) -> keyring::Result<Self> {
        let key = find_symmetric_key(id.to_owned(), true)?;

        Ok(Self::new(Key::from_slice(&key)))
    }

    pub fn update_nonce(&mut self) {
        self.nonce = Poly::generate_nonce(&mut OsRng);
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> aead::Result<Vec<u8>> {
        let mut buffer = plaintext.clone().to_vec();
        let ad = [];

        self.cipher.encrypt_in_place(&self.nonce, &ad, &mut buffer)?;
        buffer.splice(0..0, self.nonce.as_slice().iter().cloned());

        Ok(buffer)

    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> aead::Result<Vec<u8>> {
        let nonce = self.nonce.clone();
        self.decrypt_with_nonce(nonce.as_slice(), ciphertext)
    }

    pub fn decrypt_with_nonce(&mut self, nonce: &[u8], ciphertext: &[u8]) -> aead::Result<Vec<u8>> {

        let mut buffer = ciphertext.clone().to_vec();
        let ad = [];
        let nonce = Nonce::from_slice(nonce);
        self.cipher.decrypt_in_place(&nonce, &ad, &mut buffer)?;

        Ok(buffer)

    }

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_codec_round_trip() {
        let mut cipher = SymmetricCipher::new_from_id("test_workspace_cipher").unwrap();
        let test_text = b"Hello, World";

        let encrypted = cipher.encrypt(test_text).unwrap();

        let decrypted = cipher.decrypt(&encrypted[12..]).unwrap();

        let parsed_text = String::from_utf8(decrypted).unwrap();
        assert_eq!(parsed_text, "Hello, World");
    }
}
