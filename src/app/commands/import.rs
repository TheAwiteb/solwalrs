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

use clap::Parser;

use crate::app::AppArgs;
use crate::errors::Result as SolwalrsResult;
use crate::utils;
use crate::wallet::{short_public_key, ImportType, KeyPair, Wallet};

/// Import new keypair by private key or secret key (input prompt).
///
/// base58 encoded or bytes array.
#[derive(Parser, Debug)]
pub struct ImportCommand {
    /// The name of the keypair
    name: String,
    /// Whether to make the keypair the default keypair
    #[clap(short, long)]
    default: bool,
}

impl ImportCommand {
    /// Import new keypair by private key or secret key (input prompt).
    /// This function will prompt the user to enter the private key or secret key.
    pub fn run(&self, args: &AppArgs) -> SolwalrsResult<()> {
        crate::info!(args, "Importing keypair `{}`", self.name);
        let password = utils::get_password()?;
        let mut wallet = Wallet::load(&password, args)?;
        let import_type = ImportType::parse(
            rpassword::prompt_password("Enter the private key or secret key: ").map_err(|err| {
                crate::errors::Error::Other(format!("Faild to read from stdin: {err}"))
            })?,
        )?;

        let keypair = KeyPair::import(&self.name, import_type, self.default, args)?;

        crate::info!(args, "Imported keypair `{keypair:?}`");
        println!(
            "New keypair `{}` imported successfully. His public key is `{}`",
            self.name,
            short_public_key(&keypair.public_key)
        );
        wallet.add_keypair(keypair, args)?;
        wallet.export(&password, args)
    }
}
