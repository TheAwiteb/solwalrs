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

mod app;
mod errors;
mod utils;
mod wallet;

use std::process::ExitCode as StdExitCode;

use crate::app::App;
use clap::Parser;
use errors::Result as SolwalrsResult;

fn try_main() -> SolwalrsResult<()> {
    let app = App::parse();
    if let Some(command) = app.command {
        use app::Commands::*;
        match command {
            Keypair(kaypair_command) => kaypair_command.run(&app.args)?,
        }
    }
    Ok(())
}

fn main() -> StdExitCode {
    if let Err(error) = try_main() {
        eprintln!("Solwalrs: {}", error);
        return error.exit_code();
    }
    StdExitCode::SUCCESS
}
