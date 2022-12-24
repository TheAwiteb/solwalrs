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

mod airdrop;
mod balance;
mod default;
mod delete;
mod qrcode;

pub use self::qrcode::QrCodeCommand;
pub use airdrop::AirdropCommand;
pub use balance::BalanceCommand;
pub use default::DefaultCommand;
pub use delete::DeleteCommand;

use crate::{errors::Result as SolwalrsResult, wallet::Wallet};

use clap::Subcommand;

use crate::app::AppArgs;

/// Commands for managing a keypair
#[derive(Subcommand, Debug)]
pub enum KeypairCommand {
    #[clap(visible_alias = "D")]
    Delete(DeleteCommand),
    SetDefault(DefaultCommand),
    #[clap(visible_alias = "qr")]
    QrCode(QrCodeCommand),
    #[clap(visible_alias = "b")]
    Balance(BalanceCommand),
    #[clap(visible_alias = "a")]
    Airdrop(AirdropCommand),
}

impl KeypairCommand {
    /// Run the command
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        use KeypairCommand::*;

        crate::info!(args, "The keypair command is: {self:?}");
        match self {
            Delete(command) => command.run(wallet, args)?,
            SetDefault(command) => command.run(wallet, args)?,
            QrCode(command) => command.run(wallet, args)?,
            Balance(command) => command.run(wallet, args)?,
            Airdrop(command) => command.run(wallet, args)?,
        };
        Ok(())
    }
}
