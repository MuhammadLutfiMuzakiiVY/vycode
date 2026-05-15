// VyCode - Config Encryption Module
// Simple XOR-based encryption for local API key storage

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};

const ENCRYPTION_KEY: &[u8] = b"VyCode-2024-Muhammad-Lutfi-Muzaki-Secret";

/// Encrypt a plaintext string for local storage
pub fn encrypt(plaintext: &str) -> String {
    let encrypted: Vec<u8> = plaintext
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();
    STANDARD.encode(&encrypted)
}

/// Decrypt an encrypted string from local storage
pub fn decrypt(ciphertext: &str) -> Result<String> {
    let decoded = STANDARD.decode(ciphertext)?;
    let decrypted: Vec<u8> = decoded
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();
    Ok(String::from_utf8(decrypted)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = "sk-test-api-key-12345";
        let encrypted = encrypt(original);
        assert_ne!(encrypted, original);
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }
}
