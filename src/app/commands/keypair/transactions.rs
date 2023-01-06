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

use crate::app::{AppArgs, GetKeypairName};
use crate::errors::Result as SolwalrsResult;
use crate::wallet::{transactions_url, Wallet};

/// The transactions of a keypair
#[derive(Debug, Parser)]
pub struct TransactionsCommand {
    /// The name of the keypair, defaults to the default keypair
    pub name: Option<String>,
}

impl TransactionsCommand {
    pub fn run(&self, wallet: &Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        let name = self.name.get_keypair_name(wallet, args)?;
        let keypair = wallet.get_keypair(&name, args)?;
        println!(
            "Checking the transaction of `{}`\n    Here: {}",
            name,
            transactions_url(&keypair.public_key, args)?
        );
        Ok(())
    }
}
