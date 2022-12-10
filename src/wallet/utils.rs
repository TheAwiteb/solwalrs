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

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
};
use fernet::Fernet;

/// Returns `ProjectDirs` containing all the project directories
pub fn app_data_dir() -> SolwalrsResult<PathBuf> {
    let proj_dir = directories::ProjectDirs::from("com", "solwalrs", "solwalrs")
        .ok_or_else(|| SolwalrsError::AppDataDir("Failed to get app data directory".to_string()))?;
    if !proj_dir.data_local_dir().exists() {
        fs::create_dir_all(proj_dir.data_local_dir()).map_err(|err| {
            SolwalrsError::AppDataDir(format!("Failed to create app data directory: {}", err))
        })?;
    }
    Ok(proj_dir.data_local_dir().to_path_buf())
}

/// Returns the app data file
pub fn app_file_path(args: &AppArgs) -> SolwalrsResult<std::path::PathBuf> {
    if let Some(app_file) = &args.app_file {
        let app_file = Path::new(app_file);

        if app_file.exists() {
            Ok(app_file.to_path_buf())
        } else {
            let parent = app_file.parent().ok_or_else(|| {
                SolwalrsError::AppDataDir("Failed to get app data directory".to_string())
            })?;
            fs::create_dir_all(parent).map_err(|err| {
                SolwalrsError::AppDataDir(format!("Failed to create app data directory: {}", err))
            })?;
            Ok(app_file.to_path_buf())
        }
    } else {
        let app_data_dir = app_data_dir()?;
        Ok(app_data_dir.join("solwalrs.json"))
    }
}

/// Create a fernet by the given key, using it to encrypt and decrypt.
/// The key must be 32 bytes long.
pub fn get_fernet(key: &[u8]) -> SolwalrsResult<Fernet> {
    let encoded_key = base64::encode(key);
    Fernet::new(&encoded_key).ok_or_else(|| {
        SolwalrsError::InvalidPassword("The password is not 32 bytes long".to_owned())
    })
}

/// Encrypt the given plaintext with the given key.
/// The password must be 32 bytes long. will return `None` if the password is not 32 bytes long.
pub fn encrypt(password: &[u8], plaintext: &[u8]) -> SolwalrsResult<String> {
    let fernet = get_fernet(password)?;
    Ok(fernet.encrypt(plaintext))
}

/// Decrypt the given ciphertext with the given password.
/// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
/// Will return `Error::InvalidPassword` if the password is not correct.
pub fn decrypt(password: &[u8], ciphertext: &str) -> SolwalrsResult<String> {
    let fernet = get_fernet(password)?;
    fernet
        .decrypt(ciphertext)
        .map(|x| String::from_utf8_lossy(&x).to_string())
        .map_err(|_| SolwalrsError::InvalidPassword("The password is not correct".to_owned()))
}

/// Create a rows for the tables
fn create_rows(header: Vec<&str>, rows: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    // Check if the number of columns in the header and rows are the same
    if !rows.iter().all(|row| row.len() == header.len()) {
        panic!("The number of columns in the header and rows must be the same");
    }
    rows.into_iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(idx, column)| format!("{}: {column}", header[idx],))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Create a table with the given header and rows, table will be vertical
#[cfg(not(target_os = "Android"))]
pub fn print_table(header: Vec<&str>, rows: Vec<Vec<&str>>) {
    let rows = create_rows(header, rows);
    let max_len = rows
        .iter()
        .flat_map(|row| row.iter())
        .map(|column| column.chars().count())
        .max()
        .unwrap_or(0);
    // The divider of the table
    let divider = format!("+{:-<1$}+", "", max_len + 2);
    // Print the table
    for row in rows {
        println!("{}", divider);
        for column in row {
            // pritn the column
            print!("| {}", column);
            // print the last character of the column
            println!("{: <1$}|", "", max_len - column.chars().count() + 1);
        }
    }
    println!("{}", divider);
}

/// Create a table with the given header and rows, table will be vertical
#[cfg(target_os = "Android")]
pub fn print_table(header: Vec<&str>, rows: Vec<Vec<&str>>) {
    let rows = create_rows(header, rows);
    // The divider of the table
    let divider = format!("+={:-<1$}=+", "", 10);
    // Print the table
    for row in rows {
        println!("{}", divider);
        for column in row {
            println!("- {}", column)
        }
    }
    println!("{}", divider);
}
