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
mod price;
mod spl;
mod utils;

pub use keypair::*;
pub use price::*;
pub use spl::*;
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
        crate::info!(
            args,
            "Trying to loading the wallet from `{}`",
            app_file.display()
        );
        if app_file.exists() {
            let enc_wallet = EncryptedWallet::from_app_file(&app_file, args)?;
            crate::info!(
                args,
                "Loadding wallet successfully from `{}`: {enc_wallet:?}",
                app_file.display()
            );
            Ok(enc_wallet.decrypt(password, args)?)
        } else {
            // The `EncryptedWallet::export` function will create the app file if it doesn't exist, so we don't need to do it here
            crate::warn!(
                args,
                "There is no wallet file, so creating new one. new wallet file is {}",
                app_file.display()
            );
            Ok(Self::new())
        }
    }

    /// Encrypt the wallet with the given password.
    /// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
    #[must_use = "encrypting the wallet will return the encrypted wallet"]
    pub fn encrypt(self, password: &str, args: &AppArgs) -> SolwalrsResult<EncryptedWallet> {
        crate::info!(args, "Trying to encrypt the wallet");
        let password = password.as_bytes();
        let enc_keypairs = self
            .keypairs
            .into_iter()
            .map(|keypair| keypair.encrypt(password, args))
            .collect::<SolwalrsResult<Vec<_>>>()?;
        crate::info!(args, "Wallet encrypted successfully");
        Ok(EncryptedWallet {
            keypairs: enc_keypairs,
        })
    }

    /// Export the wallet to the app data file
    pub fn export(self, password: &str, args: &AppArgs) -> SolwalrsResult<()> {
        let enc_wallet = self.encrypt(password, args)?;
        enc_wallet.export(args)
    }

    /// Add a keypair to the wallet, if the keypair name already exists, it will return `Error::DuplicateKeyPairName`
    /// Note: this function will not add the keypair to the wallet file, you need to call `Wallet::export` to do that
    pub fn add_keypair(
        &mut self,
        new_keypair: keypair::KeyPair,
        args: &AppArgs,
    ) -> SolwalrsResult<()> {
        crate::info!(args, "Trying to add {new_keypair:?} to the wallet");
        if let Some(kp) = self
            .keypairs
            .iter()
            .find(|kp| kp.public_key == new_keypair.public_key)
        {
            return Err(SolwalrsError::Other(format!(
                "The public key `{}` already exists in the wallet with the name `{}`",
                short_public_key(&kp.public_key),
                kp.name
            )));
        }
        if self.get_keypair(&new_keypair.name, args).is_err() {
            if new_keypair.is_default {
                self.keypairs
                    .iter_mut()
                    .for_each(|kp| kp.is_default = kp.name == new_keypair.name);
            }
            crate::info!(args, "{new_keypair:?} added to the wallet successfully");
            self.keypairs.push(new_keypair);
            Ok(())
        } else {
            crate::warn!(
                args,
                "The keypair name `{}` already exists in the wallet",
                new_keypair.name
            );
            Err(SolwalrsError::DuplicateKeyPairName(new_keypair.name))
        }
    }

    /// Get a keypair from the wallet, if the keypair name doesn't exist, it will return `Error::KeyPairNotFound`
    pub fn get_keypair(&self, name: &str, args: &AppArgs) -> SolwalrsResult<&keypair::KeyPair> {
        crate::info!(args, "Trying to get {name} from the wallet");
        let keypair = self
            .keypairs
            .iter()
            .find(|keypair| keypair.name == name)
            .ok_or_else(|| SolwalrsError::KeyPairNotFound(name.to_string()));
        crate::info_or_warn!(args, keypair, "{name} found in the wallet successfully"; "{name} not found in the wallet {self:?}");
        keypair
    }

    /// Delete a keypair from the wallet, if the keypair name doesn't exist, it will return `Error::KeyPairNotFound`
    /// Note: this function will not delete the keypair from the wallet file, you need to call `Wallet::export` to do that
    pub fn delete_keypair(
        &mut self,
        name: &str,
        args: &AppArgs,
    ) -> SolwalrsResult<keypair::KeyPair> {
        crate::info!(args, "Trying to delete {name} from the wallet");
        let index = self
            .keypairs
            .iter()
            .position(|keypair| keypair.name == name)
            .ok_or_else(|| SolwalrsError::KeyPairNotFound(name.to_string()));
        crate::info_or_warn!(args, index, "{name} deleted from the wallet successfully"; "{name} not found in the wallet");
        Ok(self.keypairs.remove(index?))
    }

    /// Returns the default keypair, if there is no default keypair, it will return `Error::NoDefaultKeyPair`
    pub fn default_keypair(&self, args: &AppArgs) -> SolwalrsResult<&keypair::KeyPair> {
        crate::info!(
            args,
            "Trying to get the default keypair from the wallet {self:?}"
        );
        let default_key_pair = self
            .keypairs
            .iter()
            .find(|keypair| keypair.is_default)
            .ok_or(SolwalrsError::NoDefaultKeyPair);
        crate::info_or_warn!(args, default_key_pair, "{:?} is the keypair found in the wallet successfully",
            default_key_pair.as_ref().unwrap(); "No default keypair found in the wallet"
        );
        default_key_pair
    }
}

impl EncryptedWallet {
    /// Load the wallet from app date file
    fn from_app_file(file_path: &Path, args: &AppArgs) -> SolwalrsResult<Self> {
        crate::info!(
            args,
            "Trying to import ecrypted wallet from `{}` is a app file",
            file_path.display()
        );
        let file = File::open(file_path)
            .map_err(|err| SolwalrsError::Wallet(format!("Failed to open wallet file: {}", err)))?;
        let reader = BufReader::new(file);
        let wallet: Self = serde_json::from_reader(reader).map_err(|err| {
            SolwalrsError::Wallet(format!("Failed to deserialize wallet: {}", err))
        })?;
        crate::info!(
            args,
            "Ecrypted wallet imported successfully from {}",
            file_path.display()
        );
        Ok(wallet)
    }

    /// Decrypt the wallet with the given password.
    /// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
    #[must_use = "decrypting the wallet will return the decrypted wallet"]
    pub fn decrypt(self, password: &str, args: &AppArgs) -> SolwalrsResult<Wallet> {
        crate::info!(args, "Trying to decrypt the wallet");
        let password = password.as_bytes();
        let mut keypairs = self
            .keypairs
            .into_iter()
            .map(|keypair| keypair.decrypt(password, args))
            .collect::<SolwalrsResult<Vec<_>>>()?;
        crate::info!(args, "Wallet decrypted successfully");

        // Sort the keypairs by name
        keypairs.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Wallet { keypairs })
    }

    /// Export the wallet to the app data file, if the app data file doesn't exist, it will create it
    pub fn export(self, args: &AppArgs) -> SolwalrsResult<()> {
        let app_file = utils::app_file_path(args)?;
        crate::info!(
            args,
            "Trying to export the wallet to `{}`: {:?}",
            app_file.display(),
            self
        );

        let file = std::fs::File::create(&app_file).map_err(|err| {
            SolwalrsError::AppDataDir(format!("Failed to create wallet file: {}", err))
        })?;
        serde_json::to_writer(file, &self)
            .map_err(|err| SolwalrsError::Wallet(format!("Failed to serialize wallet: {}", err)))?;
        crate::info!(
            args,
            "Wallet exported successfully to {}",
            app_file.display()
        );

        Ok(())
    }
}
