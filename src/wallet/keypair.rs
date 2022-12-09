// Solwalrs, A simple and easy to use CLI Solana wallet
// Copyright (C) 2022  Solwalrs contributors <https://github.com/TheAwiteb/solwalrs/graphs/contributors>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/gpl-3.0.html>.

use base58::{FromBase58, ToBase58};
use ed25519_dalek::{PublicKey, SecretKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use super::utils;
use crate::errors::{Error as SolwalrsError, Result as SolwalrsResult};

#[derive(Debug)]
/// A keypair with clean data (decrypted)
pub struct KeyPair {
    /// The name of the keypair
    pub name: String,
    /// The public key of the keypair
    pub public_key: PublicKey,
    /// The secret key of the keypair
    pub secret_key: SecretKey,
    /// The private key of the keypair (public and secret keys), base58 encoded
    pub private_key: String,
    /// Is this keypair the default keypair
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
/// A keypair with encrypted data
pub struct EncryptedKeyPair {
    /// The encrypted name of the keypair, base54 encoded
    name: String,
    /// The encrypted private key of the keypair (public and secret keys) , base54 encoded
    private_key: String,
    /// Is this keypair the default keypair, (unencrypted)
    #[serde(default)] // Default value is false
    is_default: bool,
}

impl Clone for KeyPair {
    fn clone(&self) -> Self {
        // We can't clone the secret key, so we need to create a new one from the bytes
        // This is safe, because we know that the secret key is valid
        // Yahh, is workaround, but it's the only way to clone the secret key (is works anyway) :D I just joking
        Self {
            name: self.name.clone(),
            // Copy is implemented for PublicKey
            public_key: self.public_key,
            secret_key: SecretKey::from_bytes(&self.secret_key.to_bytes()).unwrap(),
            private_key: self.private_key.clone(),
            is_default: self.is_default,
        }
    }
}

impl KeyPair {
    /// Create a new keypair, with given name
    pub fn new(name: impl Into<String>) -> Self {
        let mut rng = OsRng::default();
        let keypair = ed25519_dalek::Keypair::generate(&mut rng);
        let private_key = keypair.to_bytes().to_base58();
        Self {
            name: name.into(),
            public_key: keypair.public,
            secret_key: keypair.secret,
            private_key,
            is_default: false,
        }
    }

    /// Import a keypair from a private key, with given name
    /// Note: the private key must be 64 bytes long, will return `Error::InvalidPrivateKey` if the private key is not 64 bytes long.
    /// Note: the private key must be in base58 format, will return `Error::InvalidPrivateKey` if the private key is not in base58 format.
    pub fn from_private_key(
        name: impl Into<String>,
        private_key: String,
        is_default: bool,
    ) -> SolwalrsResult<Self> {
        let name = name.into();
        let private_key = private_key
            .from_base58()
            .map_err(|_| SolwalrsError::InvalidPrivateKey(name.clone()))?;
        let keypair = ed25519_dalek::Keypair::from_bytes(private_key.as_slice())
            .map_err(|_| SolwalrsError::InvalidPrivateKey(name.clone()))?;
        Ok(Self {
            name,
            public_key: keypair.public,
            secret_key: keypair.secret,
            private_key: private_key.to_base58(),
            is_default,
        })
    }

    /// Encrypt the keypair with the given password, will return the encrypted keypair.
    /// The password must be 32 bytes long. will return `None` if the password is not 32 bytes long.
    #[must_use = "encrypting the keypair will return the encrypted keypair"]
    pub fn encrypt(self, password: &[u8]) -> SolwalrsResult<EncryptedKeyPair> {
        // encrypt it as base58
        let name = utils::encrypt(password, self.name.as_bytes().to_base58().as_bytes())?;
        let private_key = utils::encrypt(password, self.private_key.as_bytes())?;
        Ok(EncryptedKeyPair {
            name,
            private_key,
            is_default: self.is_default,
        })
    }
}

impl EncryptedKeyPair {
    /// Decrypt the keypair with the given password, will return the decrypted keypair.
    /// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
    /// Will return `Error::InvalidPassword` if the password is not correct.
    #[must_use = "decrypting the keypair will return the decrypted keypair"]
    pub fn decrypt(self, password: &[u8]) -> SolwalrsResult<KeyPair> {
        let name = String::from_utf8(
            utils::decrypt(password, &self.name)?
                .from_base58()
                .map_err(|_| {
                    SolwalrsError::Keypair("Failed to decrypt the keypair name".to_string())
                })?,
        )
        .map_err(|_| SolwalrsError::Keypair("Failed to decrypt the keypair name".to_string()))?;
        let private_key = utils::decrypt(password, &self.private_key)?;

        KeyPair::from_private_key(name, private_key, self.is_default)
    }
}
