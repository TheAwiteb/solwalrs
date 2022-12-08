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
use crate::wallet::{print_table, KeyPair, Wallet};

#[derive(Parser, Debug)]
pub struct ListCommand {
    /// The number of keypairs to list (default: all keypairs)
    ///
    /// Note: if you set `name`, this option will be ignored
    #[clap(short, long)]
    pub limit: Option<usize>,
    /// Print the private key of the keypair, (default: false)
    #[clap(short, long, default_value = "false")]
    pub private: bool,
    /// Print the secret key of the keypair, (default: false)
    #[clap(short, long, default_value = "false")]
    pub secret: bool,
    /// The name of the keypair, (default: list all keypairs)
    #[clap(short, long)]
    pub name: Option<String>,
}

/// Create a row for the table
fn create_row(keypair: &KeyPair, list_command: &ListCommand) -> Vec<String> {
    let mut row = vec![
        keypair.name.clone(),
        keypair.public_key.as_bytes().to_base58(),
    ];
    if list_command.secret {
        row.push(keypair.secret_key.as_bytes().to_base58());
    }
    if list_command.private {
        row.push(keypair.private_key.clone());
    }
    row
}

/// List all keypairs
fn list_all_keypairs(list_command: &ListCommand, wallet: &Wallet, header: Vec<&str>) {
    let keypairs_len = wallet.keypairs.len();
    let limit = list_command.limit.unwrap_or(keypairs_len);
    let rows: Vec<_> = wallet
        .keypairs
        .iter()
        .take(limit)
        .map(|kp| create_row(kp, list_command))
        .collect();
    print_table(
        header,
        rows.iter()
            .map(|r| r.iter().map(|s| s.as_str()).collect())
            .collect(),
    );
}

/// List keypair by name
fn list_keypair_by_name(
    list_command: &ListCommand,
    wallet: &Wallet,
    name: &str,
    header: Vec<&str>,
) -> SolwalrsResult<()> {
    let keypair = wallet.get_keypair(name)?;
    print_table(
        header,
        vec![create_row(keypair, list_command)
            .iter()
            .map(|s| s.as_str())
            .collect()],
    );
    Ok(())
}

impl ListCommand {
    /// Run the list command, will return list of keypairs
    #[must_use = "listing keypairs will return list of keypairs"]
    pub fn run(&self, wallet: Wallet) -> SolwalrsResult<()> {
        let keypairs_len = wallet.keypairs.len();
        if keypairs_len != 0 {
            let mut header = vec!["Name", "Public Key (Address)"];
            if self.secret {
                header.push("Secret Key");
            }
            if self.private {
                header.push("Private Key");
            }
            if let Some(name) = &self.name {
                // If the name is set, we will only list the keypair with the name
                list_keypair_by_name(self, &wallet, name, header)?;
            } else {
                // If the name is not set, we will list all keypairs
                list_all_keypairs(self, &wallet, header)
            };
        } else {
            println!("No keypairs found")
        }
        Ok(())
    }
}
