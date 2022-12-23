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

use std::process::{ExitCode as StdExitCode, Termination};
use sysexits::ExitCode;

/// Solwalrs errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error
    #[error("{0}")]
    IO(String),
    /// Any error with the app data directory
    #[error("{0}")]
    AppDataDir(String),
    /// Errors with wallet password
    #[error("{0}")]
    InvalidPassword(String),
    /// Error with the wallet
    #[error("{0}")]
    Wallet(String),
    /// Error with the keypair
    #[error("{0}")]
    Keypair(String),
    /// Error with the keypair name
    #[error("The keypair name `{0}` is already taken, please choose another name")]
    DuplicateKeyPairName(String),
    /// Error that keypair doesn't exist
    #[error("The keypair `{0}` doesn't exist")]
    KeyPairNotFound(String),
    /// Invalid private key
    #[error("The private key of `{0}` is invalid")]
    InvalidPrivateKey(String),
    /// No default keypair
    #[error("No default keypair is set, please set a default keypair using `solwalrs keypair set-default <keypair-name>`, or enter the keypair name after the command")]
    NoDefaultKeyPair,
    /// Invalid bytes length, not 32 and 64.
    /// 32 for secret key, 64 for private key
    #[error("Invalid bytes length: {0}. Secret key is 32 bytes, private key is 64 bytes")]
    InvalidBytesLength(usize),
    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Returns the exit code for the error
    pub fn exit_code(&self) -> StdExitCode {
        use Error::*;
        match self {
            AppDataDir(_) | IO(_) => ExitCode::IoErr.report(),
            InvalidPassword(_) => ExitCode::DataErr.report(),
            DuplicateKeyPairName(_) => ExitCode::Usage.report(),
            _ => ExitCode::Software.report(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
