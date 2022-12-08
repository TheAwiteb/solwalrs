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

mod delete;
mod list;
mod new;

pub use delete::*;
pub use list::*;
pub use new::*;

use crate::{errors::Result as SolwalrsResult, utils, wallet::Wallet};

use clap::Subcommand;

use super::AppArgs;

/// Commands for managing keypairs
#[derive(Subcommand, Debug)]
pub enum KaypairCommand {
    /// Generate a new keypair
    #[clap(visible_alias = "n")]
    New(NewCommand),
    /// List all keypairs
    #[clap(visible_alias = "ls")]
    List(ListCommand),
    /// Delete a keypair
    #[clap(visible_alias = "D")]
    Delete(DeleteCommand),
}

impl KaypairCommand {
    /// Run the command
    pub fn run(self, args: &AppArgs) -> SolwalrsResult<()> {
        use KaypairCommand::*;

        let password = utils::get_password()?;
        let mut wallet = Wallet::load(&password, args)?;
        match self {
            New(command) => {
                command.run(&mut wallet, args)?;
                wallet.export(&password, args)?;
            }
            List(command) => {
                command.run(wallet)?;
            }
            Delete(command) => {
                command.run(&mut wallet)?;
                wallet.export(&password, args)?;
            }
        };
        Ok(())
    }
}
