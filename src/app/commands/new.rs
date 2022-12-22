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

use base58::ToBase58;
use clap::Parser;

use crate::app::AppArgs;
use crate::errors::Result as SolwalrsResult;
use crate::utils;
use crate::wallet::app_file_path;
use crate::wallet::print_table;
use crate::wallet::KeyPair;
use crate::wallet::Wallet;

/// Generate a new keypair
#[derive(Parser, Debug)]
pub struct NewCommand {
    /// The name of the keypair
    pub name: String,
    /// To make the keypair the default keypair
    ///
    /// Note: if you have a default keypair, it will be replaced by the new keypair
    #[clap(short, long)]
    pub default: bool,
}

impl NewCommand {
    /// Create a new keypair, and retutn the public key
    #[must_use = "creating a new keypair will return the public key"]
    pub fn run(&self, args: &AppArgs) -> SolwalrsResult<()> {
        let password = utils::get_password()?;
        let mut wallet = Wallet::load(&password, args)?;
        let new_keypair = KeyPair::new(&self.name, self.default);
        let str_public_key = new_keypair.public_key.as_bytes().to_base58();
        let private_key = new_keypair.private_key.clone();
        wallet.add_keypair(new_keypair, args)?;
        let app_file = app_file_path(args)?;
        println!(
            "New keypair created successfully in `{}`",
            app_file.display()
        );
        print_table(
            vec!["Name", "Public Key (Address)", "Private Key", "Is default"],
            vec![vec![
                &self.name,
                &str_public_key,
                &private_key,
                &self.default.to_string(),
            ]],
        );
        wallet.export(&password, args)
    }
}
