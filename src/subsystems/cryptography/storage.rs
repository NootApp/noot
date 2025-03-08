use std::io::{Read, Write};
use std::path::PathBuf;
use cocoon::Cocoon;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::RngCore;
use sha2::Sha512;
use super::{keychain, keys};

/// Magic number, used for determining whether the file is encrypted or not.
/// This is set as the value "223" (0b11011111 / 0xDF)
pub const CONSUMER_MAGIC: u8 = 0xDF;


/// Magic number, used for determining whether the file is encrypted or not.
/// This is set as the value "137" (0b10001001 / 0x89)
pub const ENTERPRISE_MAGIC: u8 = 0x89;


/// This is the maximum number of bytes which can be read into memory in a single cycle.
pub const CHUNK_SIZE: u32 = 1024;

pub const REVISION: u16 = 1;

pub fn store(path: &PathBuf, data: &[u8], enterprise: bool) -> Result<(), String> {
    let mut seed: [u8; 32] = [0; 32];
    let pass = keys::find_primary_key().unwrap();
    generate_seed(&mut seed);

    let mut shell = Cocoon::from_seed(&pass, seed);
    let mut buffer = [];
    buffer.copy_from_slice(data);
    shell.encrypt(&mut buffer).unwrap();

    // get the length (in bytes) of the buffer as it has been encrypted.
    // if it is more than this length, then someone has done something very impressive.
    // This should provide a maximum supported file size of approximately
    //  4.294 Petabytes....
    let len = buffer.len() as u32;
    let len_bytes = len.to_be_bytes();

    let mut finalised_buffer: Vec<u8> = vec![];

    
    
    if enterprise {
        finalised_buffer.push(ENTERPRISE_MAGIC);   
    } else {
        finalised_buffer.push(CONSUMER_MAGIC);
    }
    finalised_buffer.extend(REVISION.to_be_bytes());
    finalised_buffer.extend(len_bytes);
    finalised_buffer.extend(buffer);

    dbg!(&finalised_buffer);
    
    let handle_error = std::fs::File::options().write(true).create(true).truncate(true).open(path);
    
    if let Ok(mut handle) = handle_error {
        handle.write_all(finalised_buffer.as_slice()).unwrap();
        handle.sync_all().unwrap();
    } else {
        return Err(format!("Could not open file: {:?}", path));
    }


    Ok(())
}





pub fn retrieve(path: &PathBuf) {
    let handle_result = std::fs::File::open(path);

    if let Ok(mut handle) = handle_result {
        let mut buffer: Vec<u8> = vec![];
        
        
        handle.read_to_end(&mut buffer).unwrap();

    }

}



fn generate_seed(seed: &mut [u8; 32])  {
    let mut rng = rand::rng();
    rng.fill_bytes(seed);
}