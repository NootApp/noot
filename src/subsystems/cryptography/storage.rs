use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use cocoon::Cocoon;
use crypto::Nonce;
use rand::RngCore;
use crate::subsystems::cryptography::keys::SymmetricCipher;
use super::keys;

/// Magic number, used for determining whether the file is encrypted or not.
/// This is set as the value "223" (0b11011111 / 0xDF)
pub const CONSUMER_MAGIC: u8 = 0xDF;


/// Magic number, used for determining whether the file is encrypted or not.
/// This is set as the value "137" (0b10001001 / 0x89)
pub const ENTERPRISE_MAGIC: u8 = 0x89;


/// This is the maximum number of bytes which can be read into memory in a single cycle.
pub const CHUNK_SIZE: u32 = 1024;

pub const REVISION: u16 = 1;

/// Stores the provided content using the selected encryption method at the provided path
/// Takes three params
/// - path: `&PathBuf` - The path to store the encrypted contents at
/// - data: `&[u8]` - The raw data you'd like to encrypt
/// - enterprise: `bool` - Whether to use enterprise or consumer level encryption
/// both are equally secure, but targeted towards different levels of security requirements
/// and implementation complexities.
pub fn store(path: &PathBuf, data: &[u8], enterprise: bool) -> Result<(), std::io::Error> {


    if enterprise {

        // If we are using the enterprise encryption, we use asymmetric encryption with signed payloads.

        // This implementation is not correct for the enterprise encryption and uses an old version which is being replaced.
        let mut seed: [u8; 32] = [0; 32];
        let pass = keys::find_primary_key().unwrap();
        generate_seed(&mut seed);

        let mut shell = Cocoon::from_seed(&pass, seed);
        let mut buffer: Vec<u8> = data.to_vec();
        let header = shell.encrypt(&mut buffer).unwrap();
        let mut finalised_buffer: Vec<u8> = vec![];

        finalised_buffer.push(ENTERPRISE_MAGIC);
        finalised_buffer.extend(header);
        finalised_buffer.extend(buffer);

        dbg!(&finalised_buffer);

        let mut handle = File::options().write(true).create(true).truncate(true).open(path)?;

        handle.write_all(finalised_buffer.as_slice())?;
        handle.sync_all()?;
        // drop(handle);
    } else {

        // If we are using non-enterprise encryption, we use AES-GCM with a 256-bit key
        // let raw_key = keys::find_symmetric_key().unwrap();
        let mut cipher = SymmetricCipher::new_from_id("symmetric_workspace").unwrap();



        // let key_str = String::from_utf8_lossy(&raw_key);
        //
        // debug!("Found symmetric key (store): {}", key_str);
        //
        // let cipher = Aes256GcmSiv::new_from_slice(&*raw_key).unwrap();
        // let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);
        // debug!("Nonce (store): {}", String::from_utf8_lossy(nonce.as_slice()));
        let mut buffer: Vec<u8> = vec![];

        let encrypted = cipher.encrypt(data).unwrap();

        buffer.push(CONSUMER_MAGIC);
        // buffer.extend(nonce.as_slice());
        buffer.extend(encrypted);

        let mut handle = File::options().write(true).truncate(true).create(true).open(path)?;
        handle.write_all(buffer.as_slice())?;
        handle.sync_all()?;

        // drop(handle);
    }

    Ok(())
}





pub fn retrieve(path: &PathBuf) -> Result<Vec<u8>, String> {
    let handle_result = File::open(path);

    if let Ok(mut handle) = handle_result {
        let mut buffer: Vec<u8> = vec![];

        handle.read_to_end(&mut buffer).unwrap();

        let magic = &buffer[0];
        return match magic {
            &CONSUMER_MAGIC => {
                let nonce = Nonce::from_slice(&buffer[1..13]);

                let mut cipher = SymmetricCipher::new_from_id("symmetric_workspace").unwrap();
                
                let decrypt_outcome = cipher.decrypt_with_nonce(&nonce, &buffer[13..]);

                if decrypt_outcome.is_ok() {
                    Ok(decrypt_outcome.unwrap())
                } else {
                    error!("Decryption failed: {:?}", decrypt_outcome.clone().unwrap_err());
                    Err("Decryption failed".to_string())
                }



            },

            // If the enterprise magic number is present...
            // panic because we don't support it yet.
            &ENTERPRISE_MAGIC => {
                let pass = keys::find_primary_key().unwrap();
                let mut size: u32 = 0;
                let version_bytes: &[u8] = &buffer[1..3];
                
                let version = u16::from_be_bytes(version_bytes.try_into().unwrap());
                
                debug!("Storage version: {}", version);
                
                for byte in &buffer[3..8] {
                    size = size << 8 | *byte as u32;
                }
                
                debug!("Final Size Value: {}", size);
                let header: &[u8] = &buffer[1..2+60]; // Get the cocoon header.
                
                let mut cocoon : Vec<u8> = vec![];
                
                buffer[62..].clone_into(&mut cocoon); // Get the data
                
                let shell = Cocoon::new(&pass);
                shell.decrypt(&mut cocoon, header).unwrap();
                
                Ok(cocoon)

                // panic!("Failed to parse magic file")
                
                // panic!("Enterprise grade encryption is not supported yet.")
            },

            // If there is no magic number, presume plaintext.
            _ => {
                Ok(buffer)
            }
        }

    }


    Err(format!("Could not open file: {:?}", path))
}



fn generate_seed(seed: &mut [u8; 32])  {
    let mut rng = rand::rng();
    rng.fill_bytes(seed);
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    pub fn test_encryption() {
        let raw = "Hello, World!";

        store(&PathBuf::from_str("./sample.txt").unwrap(), raw.as_bytes(), false).unwrap();

        assert_eq!(1,1);


        let output = retrieve(&PathBuf::from_str("./sample.txt").unwrap());
        assert!(output.is_ok());

        let bytes = output.unwrap();

        let raw = String::from_utf8(bytes).unwrap();

        assert_eq!("Hello, World!", raw);
    }
}