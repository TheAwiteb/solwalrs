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

use std::{fs::File, io::BufReader, path::Path};

use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
};
use serde::{Deserialize, Serialize};

mod keypair;
mod utils;

pub use keypair::*;
pub use utils::*;

/// The clean wallet (decrypted)
#[derive(Debug)]
pub struct Wallet {
    /// Wallet keypairs
    pub keypairs: Vec<keypair::KeyPair>,
}

/// The encrypted wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedWallet {
    /// encrypted wallet keypairs
    keypairs: Vec<keypair::EncryptedKeyPair>,
}

impl Wallet {
    /// Create a new wallet instance
    pub fn new() -> Self {
        Self {
            keypairs: Vec::new(),
        }
    }

    /// Load the wallet from app date file, or create a new one if it doesn't exist
    pub fn load(password: &str, args: &AppArgs) -> SolwalrsResult<Wallet> {
        let app_file = utils::app_file_path(args)?;
        if app_file.exists() {
            let enc_wallet = EncryptedWallet::from_app_file(&app_file)?;
            Ok(enc_wallet.decrypt(password)?)
        } else {
            // The `EncryptedWallet::export` function will create the app file if it doesn't exist, so we don't need to do it here
            Ok(Self::new())
        }
    }

    /// Encrypt the wallet with the given password.
    /// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
    #[must_use = "encrypting the wallet will return the encrypted wallet"]
    pub fn encrypt(self, password: &str) -> SolwalrsResult<EncryptedWallet> {
        let password = password.as_bytes();
        let enc_keypairs = self
            .keypairs
            .into_iter()
            .map(|keypair| keypair.encrypt(password))
            .collect::<SolwalrsResult<Vec<_>>>()?;

        Ok(EncryptedWallet {
            keypairs: enc_keypairs,
        })
    }

    /// Export the wallet to the app data file
    pub fn export(self, password: &str, args: &AppArgs) -> SolwalrsResult<()> {
        let enc_wallet = self.encrypt(password)?;
        enc_wallet.export(args)
    }

    /// Add a keypair to the wallet, if the keypair name already exists, it will return `Error::DuplicateKeyPairName`
    /// Note: this function will not add the keypair to the wallet file, you need to call `Wallet::export` to do that
    pub fn add_keypair(
        &mut self,
        mut new_keypair: keypair::KeyPair,
        is_default: bool,
    ) -> SolwalrsResult<()> {
        if self.get_keypair(&new_keypair.name).is_err() {
            if is_default {
                self.keypairs
                    .iter_mut()
                    .for_each(|kp| kp.is_default = false);
                new_keypair.is_default = true;
            }
            self.keypairs.push(new_keypair);
        } else {
            return Err(SolwalrsError::DuplicateKeyPairName(new_keypair.name));
        }

        Ok(())
    }

    /// Get a keypair from the wallet, if the keypair name doesn't exist, it will return `Error::KeyPairNotFound`
    pub fn get_keypair(&self, name: &str) -> SolwalrsResult<&keypair::KeyPair> {
        self.keypairs
            .iter()
            .find(|keypair| keypair.name == name)
            .ok_or_else(|| SolwalrsError::KeyPairNotFound(name.to_string()))
    }

    /// Delete a keypair from the wallet, if the keypair name doesn't exist, it will return `Error::KeyPairNotFound`
    /// Note: this function will not delete the keypair from the wallet file, you need to call `Wallet::export` to do that
    pub fn delete_keypair(&mut self, name: &str) -> SolwalrsResult<keypair::KeyPair> {
        let index = self
            .keypairs
            .iter()
            .position(|keypair| keypair.name == name)
            .ok_or_else(|| SolwalrsError::KeyPairNotFound(name.to_string()))?;
        Ok(self.keypairs.remove(index))
    }

    /// Returns the default keypair, if there is no default keypair, it will return `Error::NoDefaultKeyPair`
    pub fn default_keypair(&self) -> SolwalrsResult<&keypair::KeyPair> {
        self.keypairs
            .iter()
            .find(|keypair| keypair.is_default)
            .ok_or(SolwalrsError::NoDefaultKeyPair)
    }
}

impl EncryptedWallet {
    /// Load the wallet from app date file
    fn from_app_file(file_path: &Path) -> SolwalrsResult<Self> {
        let file = File::open(file_path)
            .map_err(|err| SolwalrsError::Wallet(format!("Failed to open wallet file: {}", err)))?;
        let reader = BufReader::new(file);
        let wallet: Self = serde_json::from_reader(reader).map_err(|err| {
            SolwalrsError::Wallet(format!("Failed to deserialize wallet: {}", err))
        })?;
        Ok(wallet)
    }

    /// Decrypt the wallet with the given password.
    /// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
    #[must_use = "decrypting the wallet will return the decrypted wallet"]
    pub fn decrypt(self, password: &str) -> SolwalrsResult<Wallet> {
        let password = password.as_bytes();
        let keypairs = self
            .keypairs
            .into_iter()
            .map(|keypair| keypair.decrypt(password))
            .collect::<SolwalrsResult<Vec<_>>>()?;

        Ok(Wallet { keypairs })
    }

    /// Export the wallet to the app data file, if the app data file doesn't exist, it will create it
    pub fn export(self, args: &AppArgs) -> SolwalrsResult<()> {
        let app_file = utils::app_file_path(args)?;
        let file = std::fs::File::create(app_file).map_err(|err| {
            SolwalrsError::AppDataDir(format!("Failed to create wallet file: {}", err))
        })?;
        serde_json::to_writer(file, &self)
            .map_err(|err| SolwalrsError::Wallet(format!("Failed to serialize wallet: {}", err)))?;

        Ok(())
    }
}
