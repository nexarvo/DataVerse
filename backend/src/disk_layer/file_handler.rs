use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM implementation
use base64::{decode, encode};
use lazy_static::lazy_static;
use log::info;
use std::env;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

// AES-GCM expects a 32-byte key and a 12-byte nonce
fn load_key_and_nonce() -> ([u8; 32], [u8; 12]) {
    let key_base64 = env::var("FILE_ENCRYPTION_KEY").expect("FILE_ENCRYPTION_KEY must be set");
    let nonce_base64 =
        env::var("FILE_ENCRYPTION_NONCE").expect("FILE_ENCRYPTION_NONCE must be set");

    // Decode from base64
    let key_bytes = decode(key_base64).expect("Failed to decode FILE_ENCRYPTION_KEY");
    let nonce_bytes = decode(nonce_base64).expect("Failed to decode FILE_ENCRYPTION_NONCE");

    // Ensure nonce is exactly 12 bytes, truncating if necessary
    assert!(
        nonce_bytes.len() >= 12,
        "Nonce must be at least 12 bytes long"
    );
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&nonce_bytes[..12]); // Take the first 12 bytes of the decoded nonce

    // Convert to arrays
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);

    (key, nonce)
}

lazy_static! {
    static ref ENCRYPTION_KEY: [u8; 32] = load_key_and_nonce().0;
    static ref ENCRYPTION_NONCE: [u8; 12] = load_key_and_nonce().1;
}

/// Encrypt data before saving to disk.
fn encrypt(data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(&ENCRYPTION_KEY[..]);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&ENCRYPTION_NONCE[..]); // 96-bits; unique per message
    cipher.encrypt(nonce, data).expect("Encryption failed")
}

/// Decrypt data after reading from disk.
fn decrypt(data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(&ENCRYPTION_KEY[..]);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&ENCRYPTION_NONCE[..]); // 96-bits; must match the encryption nonce
    cipher.decrypt(nonce, data).expect("Decryption failed")
}

/// Write dataset securely to disk asynchronously.
pub async fn write_dataset(filename: &str, data: &[u8]) -> io::Result<()> {
    info!("Writing dataset to disk: {}", filename);

    // Get the disk path, fallback to "./data" if not set
    let disk_path = env::var("DATA_DIRECTORY_PATH").unwrap_or_else(|_| "./data".to_string());
    let file_path = format!("{}/{}", disk_path, filename);

    // Ensure that the directory exists, create if not
    let dir_path = Path::new(&disk_path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?; // This will create all necessary parent directories
    }

    // Encrypt the data
    let encrypted_data = encrypt(data);

    // Open or create the file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .await?;

    // Write the encrypted data to the file
    file.write_all(&encrypted_data).await?;

    info!("Successfully wrote dataset to disk: {}", filename);
    Ok(())
}

/// Read dataset securely from disk asynchronously.
pub async fn read_dataset(filename: &str) -> io::Result<Vec<u8>> {
    info!("Reading dataset from disk: {}", filename);
    let disk_path = env::var("DATA_DIRECTORY_PATH").unwrap_or_else(|_| "./data".to_string());
    let file_path = format!("{}/{}", disk_path, filename);

    let mut file = File::open(file_path).await?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data).await?;
    info!("Successfully read dataset from disk: {}", filename);
    Ok(decrypt(&encrypted_data))
}
