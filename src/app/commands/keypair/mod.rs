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

mod default;
mod delete;

pub use default::DefaultCommand;
pub use delete::DeleteCommand;

use crate::{errors::Result as SolwalrsResult, utils, wallet::Wallet};

use clap::Subcommand;

use crate::app::AppArgs;

/// Commands for managing a keypair
#[derive(Subcommand, Debug)]
pub enum KeypairCommand {
    #[clap(visible_alias = "D")]
    Delete(DeleteCommand),
    SetDefault(DefaultCommand),
}

impl KeypairCommand {
    /// Run the command
    pub fn run(&self, args: &AppArgs) -> SolwalrsResult<()> {
        use KeypairCommand::*;

        let password = utils::get_password()?;
        let mut wallet = Wallet::load(&password, args)?;
        crate::info!(args, "The keypair command is: {self:?}");
        match self {
            Delete(command) => {
                command.run(&mut wallet, args)?;
                wallet.export(&password, args)?;
            }
            SetDefault(command) => {
                command.run(&mut wallet, args)?;
                wallet.export(&password, args)?;
            }
        };
        Ok(())
    }
}
