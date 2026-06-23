use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::error::ProviderError;

const NONCE_LEN: usize = 12;

pub struct GitHubCredentialCipher {
    cipher: Aes256Gcm,
}

impl GitHubCredentialCipher {
    pub fn from_env() -> Result<Self, ProviderError> {
        let key_material = std::env::var("SDKWORK_GITHUB_CREDENTIAL_ENCRYPTION_KEY")
            .map_err(|_| {
                ProviderError::Configuration(
                    "SDKWORK_GITHUB_CREDENTIAL_ENCRYPTION_KEY is not configured".to_string(),
                )
            })?;
        if key_material.trim().is_empty() {
            return Err(ProviderError::Configuration(
                "SDKWORK_GITHUB_CREDENTIAL_ENCRYPTION_KEY is empty".to_string(),
            ));
        }
        Self::from_passphrase(&key_material)
    }

    pub fn from_passphrase(passphrase: &str) -> Result<Self, ProviderError> {
        let digest = Sha256::digest(passphrase.as_bytes());
        let cipher = Aes256Gcm::new_from_slice(&digest).map_err(|error| {
            ProviderError::Configuration(format!("invalid credential cipher key: {error}"))
        })?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, ProviderError> {
        let mut nonce_bytes = [0_u8; NONCE_LEN];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|error| ProviderError::Configuration(format!("encrypt secret failed: {error}")))?;

        let mut payload = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);
        Ok(STANDARD.encode(payload))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, ProviderError> {
        let payload = STANDARD
            .decode(encoded)
            .map_err(|error| ProviderError::Configuration(format!("invalid secret payload: {error}")))?;
        if payload.len() <= NONCE_LEN {
            return Err(ProviderError::Configuration(
                "encrypted secret payload is too short".to_string(),
            ));
        }
        let (nonce_bytes, ciphertext) = payload.split_at(NONCE_LEN);
        let plaintext = self
            .cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|error| ProviderError::Configuration(format!("decrypt secret failed: {error}")))?;
        String::from_utf8(plaintext)
            .map_err(|error| ProviderError::Configuration(format!("secret utf8 invalid: {error}")))
    }
}

#[cfg(test)]
mod tests {
    use super::GitHubCredentialCipher;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let cipher = GitHubCredentialCipher::from_passphrase("test-key-material").unwrap();
        let encrypted = cipher.encrypt("ghp_test_token").unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, "ghp_test_token");
    }
}
