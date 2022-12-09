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

use crate::errors::Result as SolwalrsResult;
use crate::wallet::{print_table, Wallet};

#[derive(Parser, Debug)]
pub struct DeleteCommand {
    /// The name of the keypair, will use the default keypair if not provided
    pub name: Option<String>,
}

impl DeleteCommand {
    /// Delete a keypair, and return the table of deleted keypair
    /// Note: this function will not delete the keypair from the wallet file, you need to call `Wallet::export` to do that
    #[must_use = "deleting a keypair will return the deleted keypair as a table"]
    pub fn run(&self, wallet: &mut Wallet) -> SolwalrsResult<()> {
        let name = self
            .name
            .clone()
            .map(Ok)
            .unwrap_or_else(|| Ok(wallet.default_keypair()?.name.clone()))?;
        let deleted_keypair = wallet.delete_keypair(&name)?;
        println!("Done deleting successfully!");
        print_table(
            vec!["Name", "Public Key (Address)"],
            vec![vec![
                &name,
                &deleted_keypair.public_key.as_bytes().to_base58(),
            ]],
        );
        Ok(())
    }
}
