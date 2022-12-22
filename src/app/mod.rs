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

mod commands;

use clap::builder::{ValueParser, ValueParserFactory};
use clap::Parser;
pub use commands::*;

use crate::errors::Result as SolwalrsResult;
use crate::wallet::Wallet;

const COPYRIGHT: &str = "Solwalrs  Copyright (C) 2022  Solwalrs contributors <https://github.com/TheAwiteb/solwalrs/graphs/contributors>
This program comes with ABSOLUTELY NO WARRANTY; for details see <https://www.gnu.org/licenses/gpl-3.0.html>.
This is free software, and you are welcome to redistribute it
under certain conditions; see <https://www.gnu.org/licenses/gpl-3.0.html> for details.";

/// The name of the keypair to work with.
/// This struct help to get the keypair name from the command line.
#[derive(Debug, Clone)]
pub struct KeypairName {
    name: Option<String>,
}

impl KeypairName {
    /// Get the name of the keypair, if the name is not provided, the default keypair name will be used.
    pub fn name(&self, wallet: &Wallet, args: &AppArgs) -> SolwalrsResult<String> {
        self.name
            .clone()
            .map(Ok)
            .unwrap_or_else(|| Ok(wallet.default_keypair(args)?.name.clone()))
    }
}

impl ValueParserFactory for KeypairName {
    type Parser = ValueParser;

    fn value_parser() -> Self::Parser {
        ValueParser::string()
    }
}

#[derive(Parser, Debug)]
pub struct AppArgs {
    /// The path to the app file
    ///
    /// don't recommend to change this, default is `apps_data_directory/solwalrs.json`
    #[clap(long)]
    pub app_file: Option<String>,
    /// Verbose mode, for debugging
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[clap(subcommand, visible_alias = "kp")]
    Keypair(keypair::KeypairCommand),
    #[clap(visible_alias = "n")]
    New(NewCommand),
    #[clap(visible_alias = "ls")]
    List(ListCommand),
    #[clap(visible_alias = "i")]
    Import(ImportCommand),
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = COPYRIGHT)]
pub struct App {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub args: AppArgs,
}
