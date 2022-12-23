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

use clap::Parser;
pub use commands::*;

use crate::errors::Result as SolwalrsResult;
use crate::wallet::Wallet;

const COPYRIGHT: &str = "Solwalrs  Copyright (C) 2022  Solwalrs contributors <https://github.com/TheAwiteb/solwalrs/graphs/contributors>
This program comes with ABSOLUTELY NO WARRANTY; for details see <https://www.gnu.org/licenses/gpl-3.0.html>.
This is free software, and you are welcome to redistribute it
under certain conditions; see <https://www.gnu.org/licenses/gpl-3.0.html> for details.";

/// Trait to get the keypair name if provided or the default keypair name
pub trait GetKeypairName {
    /// Get the keypair name if provided or the default keypair name
    fn get_keypair_name(&self, wallet: &Wallet, args: &AppArgs) -> SolwalrsResult<String>;
}

impl<T> GetKeypairName for Option<T>
where
    T: ToString,
{
    fn get_keypair_name(&self, wallet: &Wallet, args: &AppArgs) -> SolwalrsResult<String> {
        self.as_ref()
            .map(|name| name.to_string())
            .map(Ok)
            .unwrap_or_else(|| wallet.default_keypair(args).map(|kp| kp.name.clone()))
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
    #[clap(visible_alias = "cp")]
    Completions(CompletionsCommand),
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = COPYRIGHT)]
pub struct App {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub args: AppArgs,
}

impl Commands {
    /// Whether the command needs a wallet
    pub fn needs_wallet(&self) -> bool {
        use Commands::*;
        !matches!(self, Completions(_))
    }
}
