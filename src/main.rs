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
#[macro_use]
mod log;
mod utils;
mod wallet;

use clap::Parser;
use std::process::ExitCode as StdExitCode;

use crate::{app::App, wallet::Wallet};
use errors::Result as SolwalrsResult;

fn try_main(app: &App) -> SolwalrsResult<()> {
    use app::Commands::*;

    info!(&app.args, "Solwalrs v{}", env!("CARGO_PKG_VERSION"));
    info!(&app.args, "The app args is: {:?}", app.args);
    if let Some(command) = &app.command {
        info!(&app.args, "The command is {command:?}");

        let mut wallet = Wallet::new();
        let mut password = String::new();
        if command.needs_wallet() {
            password = utils::get_password()?;
            wallet = Wallet::load(&password, &app.args)?;
        }

        match command {
            Keypair(keypair_command) => keypair_command.run(&mut wallet, &app.args)?,
            New(new_command) => new_command.run(&mut wallet, &app.args)?,
            List(list_command) => list_command.run(&mut wallet, &app.args)?,
            Import(import_command) => import_command.run(&mut wallet, &app.args)?,
            Completions(completions_command) => completions_command.run(),
            Clean(clean_command) => clean_command.run(&app.args)?,
        };
        if command.needs_wallet() {
            wallet.export(&password, &app.args)?;
        }
    }
    Ok(())
}

fn main() -> StdExitCode {
    let app = App::parse();
    if let Err(error) = try_main(&app) {
        error!(&app.args, "There is an error: {error:?}");
        eprintln!("Solwalrs: {error}");
        return error.exit_code();
    }
    StdExitCode::SUCCESS
}
