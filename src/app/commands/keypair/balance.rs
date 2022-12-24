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

use crate::app::GetKeypairName;
use crate::errors::Result as SolwalrsResult;
use crate::wallet::{short_public_key, Tokens};
use crate::{app::AppArgs, wallet::Wallet};

#[derive(Parser, Debug)]
pub struct BalanceCommand {
    /// The name of the keypair to get the balance of
    pub name: Option<String>,
    /// Whether to show the balance in lamports
    #[clap(short, long)]
    pub lamports: bool,
    /// The spl token to get the balance of, if not specified, the SOL balance will be shown
    #[clap(long)]
    pub spl: Option<Tokens>,
}

impl BalanceCommand {
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        let name = self.name.get_keypair_name(wallet, args)?;
        let keypair = wallet.get_keypair(&name, args)?;
        let balance = keypair.balance(args, self.spl.as_ref())?;
        let token_name = self.spl.as_ref().map(|token| token.name()).unwrap_or("SOL");
        let message = format!(
            "The `{}` address has",
            short_public_key(&keypair.public_key)
        );
        if self.lamports {
            println!("{message} `{balance}` {token_name} lamports");
        } else {
            println!(
                "{message} `{}` {token_name}",
                balance as f64
                    / self
                        .spl
                        .as_ref()
                        .map(|s| s.lamports_per_token())
                        .unwrap_or(1e9)
            );
        }
        Ok(())
    }
}
