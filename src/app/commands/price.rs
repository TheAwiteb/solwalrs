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
use crate::wallet::{Price, Tokens};

/// Get the price of a token/SOL in USDT
#[derive(Debug, Parser)]
pub struct PriceCommand {
    /// The name of the keypair to get the balance of (defaults to the default keypair)
    pub name: Option<String>,
    /// SPL token to get the price of
    #[clap(long)]
    pub spl: Option<Tokens>,
}

impl PriceCommand {
    pub fn run(&self, args: &AppArgs) -> SolwalrsResult<()> {
        let price = Price::new(self.spl.as_ref(), args)?;
        println!(
            "{}: ${}, Price change in the last 24h: {}",
            self.spl.as_ref().map(|t| t.name()).unwrap_or("SOL"),
            price.data.price,
            price.data.price_change_24h
        );
        Ok(())
    }
}
