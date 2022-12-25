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
use crate::wallet::{confirm_signature, rpc_url, transaction_url};
use crate::{app::AppArgs, wallet::Wallet};

/// Request an airdrop to a keypair
#[derive(Debug, Parser)]
pub struct AirdropCommand {
    /// The name of the keypair to airdrop to (defaults to the default wallet)
    pub name: Option<String>,
    /// The amount to airdrop
    #[clap(short, long)]
    pub amount: f64,
    /// Whether the amount is in lamports
    #[clap(short, long)]
    pub lamports: bool,
}

impl AirdropCommand {
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        let name = self.name.get_keypair_name(wallet, args)?;
        let keypair = wallet.get_keypair(&name, args)?;
        let amount = if !self.lamports {
            (self.amount * 1e9) as u64
        } else {
            self.amount as u64
        };
        let signature = keypair.request_airdrop(amount, args)?;
        let rpc_url = rpc_url(args)?;
        println!(
            "Waiting for airdrop to be confirmed, this may take a while...\n{}",
            transaction_url(&signature, &rpc_url)
        );
        confirm_signature(args, &signature)?;
        println!("Transaction confirmed!");
        Ok(())
    }
}
