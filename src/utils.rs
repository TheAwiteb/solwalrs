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

use super::errors::{Error as SolwalrsError, Result as SolwalrsResult};

/// Get the password from stdin, and return it as a String
pub fn get_password() -> SolwalrsResult<String> {
    let password = rpassword::prompt_password("Enter the wallet password: ")
        .map_err(|err| SolwalrsError::Other(format!("Failed to get password: {}", err)))?;

    if password.len() != 32 {
        return Err(SolwalrsError::InvalidPassword(
            "The password must be 32 bytes long".to_owned(),
        ));
    }
    Ok(password)
}
