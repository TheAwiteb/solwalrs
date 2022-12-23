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
use crate::wallet::clean_wallet;

/// Clean the wallet. This will remove all the keypairs from the wallet.
#[derive(Parser, Debug)]
pub struct CleanCommand;

impl CleanCommand {
    /// Run the command
    pub fn run(&self, args: &AppArgs) -> SolwalrsResult<()> {
        clean_wallet(args)?;
        println!("Wallet cleaned successfully");
        Ok(())
    }
}
