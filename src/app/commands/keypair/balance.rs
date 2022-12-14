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
use crate::wallet::cache::Cache;
use crate::wallet::{short_public_key, Price, Tokens};
use crate::{app::AppArgs, wallet::Wallet};

/// Get the balance of a keypair, SOL/SPL
#[derive(Parser, Debug)]
pub struct BalanceCommand {
    /// The name of the keypair to get the balance of (defaults to the default wallet)
    pub name: Option<String>,
    /// Whether to show the balance in lamports
    #[clap(short, long)]
    pub lamports: bool,
    /// The spl token to get the balance of, if not specified, the SOL balance will be shown
    #[clap(long)]
    pub spl: Option<Tokens>,
}

impl BalanceCommand {
    pub fn run(
        &self,
        wallet: &mut Wallet,
        args: &AppArgs,
        cache: &mut Cache,
    ) -> SolwalrsResult<()> {
        let name = self.name.get_keypair_name(wallet, args)?;
        let keypair = wallet.get_keypair(&name, args)?;
        let balance = keypair.balance(args, self.spl.as_ref())?;
        let per_one = self
            .spl
            .as_ref()
            .map(Tokens::lamports_per_token)
            .unwrap_or(1e9);
        let price = Price::get_price(self.spl.as_ref(), args, cache)?.data.price
            * (balance as f64 / per_one);
        let token_name = self.spl.as_ref().map(Tokens::name).unwrap_or("SOL");
        let message = format!(
            "The `{}` address has",
            short_public_key(&keypair.public_key)
        );
        if self.lamports {
            println!("{message} `{balance}` {token_name} lamports ~${price:.2}");
        } else {
            println!(
                "{message} `{}` {token_name} ~${price:.2}",
                balance as f64 / per_one
            );
        }
        Ok(())
    }
}
