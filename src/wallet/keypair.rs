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

use super::{short_public_key, utils};
use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
};

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

#[derive(Serialize, Deserialize)]
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

impl std::fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPair")
            .field("name", &self.name)
            .field("public_key", &short_public_key(&self.public_key))
            .field("private_key", &"***")
            .field("is_default", &self.is_default)
            .finish()
    }
}

impl std::fmt::Debug for EncryptedKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptedKeyPair")
            .field("name", &self.name)
            .field("private_key", &"***")
            .field("is_default", &self.is_default)
            .finish()
    }
}

/// The type of the keypair to import, private key or secret key.
pub enum ImportType {
    /// The private key, which is the public and secret key combined. (64 bytes)
    PrivateKey { bytes: Vec<u8> },
    /// The secret key, which is the private key without the public key. (32 bytes)
    SecretKey { bytes: Vec<u8> },
}

impl ImportType {
    /// From bytes array
    pub fn from_bytes(bytes: Vec<u8>) -> SolwalrsResult<Self> {
        let length = bytes.len();
        // private key is 64 bytes, secret key is 32 bytes.
        if length == 64 {
            Ok(ImportType::PrivateKey { bytes })
        } else if length == 32 {
            Ok(ImportType::SecretKey { bytes })
        } else {
            Err(SolwalrsError::InvalidBytesLength(length))
        }
    }
    /// Parse the import type from the string.
    pub fn parse(input: String) -> SolwalrsResult<Self> {
        if input.starts_with('[') && input.ends_with(']') {
            // parse a string vector to vec<u8>
            let bytes = input
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|s| s.trim().parse::<u8>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| SolwalrsError::Other(format!("Failed to parse bytes: {err}")))?;
            Self::from_bytes(bytes)
        } else {
            // base58 encoded, so need to decode it.
            let bytes = input
                .from_base58()
                .map_err(|err| SolwalrsError::Other(format!("Failed to decode base58: {err:?}")))?;
            Self::from_bytes(bytes)
        }
    }
}

impl KeyPair {
    /// Create a new keypair, with given name
    pub fn new(name: impl Into<String>, default: bool) -> Self {
        let mut rng = OsRng::default();
        let keypair = ed25519_dalek::Keypair::generate(&mut rng);
        let private_key = keypair.to_bytes().to_base58();
        Self {
            name: name.into(),
            public_key: keypair.public,
            secret_key: keypair.secret,
            private_key,
            is_default: default,
        }
    }

    /// Import a keypair from a private key, with given name
    /// Note: the private key must be 64 bytes long, will return `Error::InvalidPrivateKey` if the private key is not 64 bytes long.
    /// Note: the private key must be in base58 format, will return `Error::InvalidPrivateKey` if the private key is not in base58 format.
    pub fn from_private_key(
        name: impl Into<String>,
        private_key: String,
        is_default: bool,
        args: &AppArgs,
    ) -> SolwalrsResult<Self> {
        let name = name.into();
        crate::info!(args, "Trying to import the keypair from encoded keypair");

        // FIXME: Should expect the private key as bytes, not base58 encoded string.
        let private_key = private_key
            .from_base58()
            .map_err(|_| SolwalrsError::InvalidPrivateKey(name.clone()))?;
        let keypair = ed25519_dalek::Keypair::from_bytes(private_key.as_slice())
            .map_err(|_| SolwalrsError::InvalidPrivateKey(name.clone()))?;
        crate::info!(
            args,
            "Keypair `{name}` imported successfully from encoded keypair, public key is {}",
            short_public_key(&keypair.public)
        );
        Ok(Self {
            name,
            public_key: keypair.public,
            secret_key: keypair.secret,
            private_key: private_key.to_base58(),
            is_default,
        })
    }

    /// Import a keypair from a secret key, secret key is 32 bytes long.
    pub fn from_secret_key(
        name: impl Into<String>,
        bytes: Vec<u8>,
        is_default: bool,
        args: &AppArgs,
    ) -> SolwalrsResult<Self> {
        crate::info!(args, "Trying to import the keypair from secret key");
        let name = name.into();
        let secret_key = SecretKey::from_bytes(bytes.as_slice())
            .map_err(|err| SolwalrsError::Other(format!("Invaild secret key: {err}")))?;
        let public_key = PublicKey::from(&secret_key);
        crate::info!(
            args,
            "Keypair `{name}` imported successfully from secret key, public key is {}",
            short_public_key(&public_key)
        );
        Ok(Self {
            name,
            public_key,
            secret_key,
            private_key: bytes.to_base58(),
            is_default,
        })
    }

    /// Import a keypair
    pub fn import(
        name: impl Into<String>,
        import_type: ImportType,
        is_default: bool,
        args: &AppArgs,
    ) -> SolwalrsResult<Self> {
        crate::info!(args, "Trying to import the keypair");
        match import_type {
            ImportType::PrivateKey { bytes } => {
                Self::from_private_key(name, bytes.to_base58(), is_default, args)
            }
            ImportType::SecretKey { bytes } => Self::from_secret_key(name, bytes, is_default, args),
        }
    }

    /// Encrypt the keypair with the given password, will return the encrypted keypair.
    /// The password must be 32 bytes long. will return `None` if the password is not 32 bytes long.
    #[must_use = "encrypting the keypair will return the encrypted keypair"]
    pub fn encrypt(self, password: &[u8], args: &AppArgs) -> SolwalrsResult<EncryptedKeyPair> {
        // encrypt it as base58`
        crate::info!(
            args,
            "Trying to encrypt the keypair `{}` is public key is {}",
            self.name,
            short_public_key(&self.public_key)
        );

        let name = utils::encrypt(password, self.name.as_bytes().to_base58().as_bytes())?;
        let private_key = utils::encrypt(password, self.private_key.as_bytes())?;
        crate::info!(args, "Keypair `{}` encrypted successfully", self.name);
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
    pub fn decrypt(self, password: &[u8], args: &AppArgs) -> SolwalrsResult<KeyPair> {
        crate::info!(args, "Trying to decrypt a keypair");
        let name = String::from_utf8(
            utils::decrypt(password, &self.name)?
                .from_base58()
                .map_err(|_| {
                    SolwalrsError::Keypair("Failed to decrypt the keypair name".to_string())
                })?,
        )
        .map_err(|_| SolwalrsError::Keypair("Failed to decrypt the keypair name".to_string()))?;
        let private_key = utils::decrypt(password, &self.private_key)?;
        crate::info!(args, "Keypair `{}` decrypted successfully", name);

        KeyPair::from_private_key(name, private_key, self.is_default, args)
    }
}
