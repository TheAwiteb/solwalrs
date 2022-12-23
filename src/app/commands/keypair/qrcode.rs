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

use std::path::PathBuf;

use clap::Parser;
use qrcode::render::unicode;

use crate::app::{AppArgs, GetKeypairName};
use crate::errors::{Error as SolwalrsError, Result as SolwalrsResult};
use crate::wallet::{short_public_key, Wallet};

/// Print the QR code of a keypair, or save it as an image
#[derive(Parser, Debug)]
pub struct QrCodeCommand {
    /// The name of the keypair, will use the default keypair if not provided
    pub name: Option<String>,
    /// The path to save the QR code image to, if not provided, the QR code will be printed to the terminal
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

impl QrCodeCommand {
    /// Run the command
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        let name = self.name.get_keypair_name(wallet, args)?;
        let keypair = wallet.get_keypair(&name, args)?;
        let qr_code = keypair.qr_code();
        if let Some(path) = &self.output {
            crate::info!(args, "Saving QR code to {path:?}");
            qr_code
                .render::<image::Luma<u8>>()
                .build()
                .save(path)
                .map_err(|err| {
                    SolwalrsError::IO(format!(
                        "Failed to save QR code to {}: {err}",
                        path.display()
                    ))
                })?;
            crate::info!(args, "Saved QR code to `{path:?}`");
            println!("Saved QR code to `{}`", path.display())
        } else {
            crate::info!(args, "Printing QR code to terminal");
            let str_qr_code = qr_code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!(
                "\n{str_qr_code}\n{:>24}\n",
                short_public_key(&keypair.public_key)
            )
        }
        Ok(())
    }
}
