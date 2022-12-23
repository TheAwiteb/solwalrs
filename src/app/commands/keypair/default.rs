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
use crate::wallet::Wallet;
// use crate::app::commands::utils::

/// set a keypair as the default
#[derive(Debug, Parser)]
pub struct DefaultCommand {
    /// The name of the keypair to set as default
    pub name: String,
}

impl DefaultCommand {
    /// set a keypair as the default
    /// Note: You need to export the wallet after running this command, using `Wallet::export`
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        crate::info!(args, "Setting `{}` as a default in {:?}", self.name, wallet);
        let mut keypair = wallet.get_keypair(&self.name, args)?.clone();
        crate::info!(
            args,
            "The public key of the keypair is {:?}",
            keypair.public_key
        );
        keypair.is_default = true;
        wallet.delete_keypair(&self.name, args)?;
        wallet.add_keypair(keypair, args)?;
        println!("Done setting `{}` as a default!", self.name);
        Ok(())
    }
}
