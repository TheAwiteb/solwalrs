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
use base58::ToBase58;
use ed25519_dalek::PublicKey;
use fernet::Fernet;
use solana_account_decoder::UiAccountData;
use solana_client::rpc_client::RpcClient;

use super::Tokens;

/// Returns the project directories
pub fn project_dirs() -> SolwalrsResult<directories::ProjectDirs> {
    directories::ProjectDirs::from("com", "solwalrs", "solwalrs")
        .ok_or_else(|| SolwalrsError::AppDataDir("Failed to get app data directory".to_string()))
}

/// Returns the path of the app data directory
pub fn app_data_dir() -> SolwalrsResult<PathBuf> {
    let proj_dir = project_dirs()?;
    if !proj_dir.data_local_dir().exists() {
        fs::create_dir_all(proj_dir.data_local_dir()).map_err(|err| {
            SolwalrsError::AppDataDir(format!("Failed to create app data directory: {}", err))
        })?;
    }
    Ok(proj_dir.data_local_dir().to_path_buf())
}

/// Returns the path of the app cache directory
pub fn app_cache_dir() -> SolwalrsResult<PathBuf> {
    let proj_dir = project_dirs()?;
    if !proj_dir.cache_dir().exists() {
        fs::create_dir_all(proj_dir.cache_dir()).map_err(|err| {
            SolwalrsError::AppDataDir(format!("Failed to create app cache directory: {}", err))
        })?;
    }
    Ok(proj_dir.cache_dir().to_path_buf())
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
        app_data_dir().map(|data| data.join("solwalrs.json"))
    }
}

/// Returns the app cache file
pub fn app_cache_file_path() -> SolwalrsResult<std::path::PathBuf> {
    app_cache_dir().map(|cache| cache.join("solwalrs.cache"))
}

/// Clean the wallet, it will remove the wallet file
pub fn clean_wallet(args: &AppArgs) -> SolwalrsResult<()> {
    crate::info!(args, "Trying to clean the wallet");
    let app_file = app_file_path(args)?;
    crate::info!(args, "Removing the wallet file");
    std::fs::remove_file(app_file)
        .map_err(|err| SolwalrsError::Wallet(format!("Failed to remove wallet file: {}", err)))?;
    crate::info!(args, "Wallet file removed successfully");
    let cache_file = app_cache_file_path()?;
    crate::info!(args, "Removing the cache file");
    std::fs::remove_file(cache_file)
        .map_err(|err| SolwalrsError::Wallet(format!("Failed to remove cache file: {}", err)))?;
    crate::info!(args, "Cache file removed successfully");
    Ok(())
}

/// Returns the rpc url
pub fn rpc_url(args: &AppArgs) -> SolwalrsResult<String> {
    let url = format!(
        "{}{}",
        args.rpc.to_string().trim_end_matches('/'),
        args.rpc
            .port()
            .map(|port| format!(":{}", port))
            .unwrap_or_default()
    );
    Ok(url)
}

/// Returns the RPC client, if the `--rpc` flag is not set, it will use the default RPC client.
/// The default RPC client is `https://api.mainnet-beta.solana.com`
pub fn rpc_client(args: &AppArgs) -> SolwalrsResult<RpcClient> {
    Ok(RpcClient::new(rpc_url(args)?))
}

/// Returns the SPL balance of the given public key
pub fn spl_balance(args: &AppArgs, public_key: &PublicKey, token: &Tokens) -> SolwalrsResult<u64> {
    let client = rpc_client(args)?;
    let short_pubk = short_public_key(public_key);
    let pubk = public_key.as_bytes().to_base58();
    let token_name = token.name();
    crate::info!(
        args,
        "Trying to get the {token_name} balance of {short_pubk}"
    );

    // SAFETY: This is safe because we are sure that the public key is valid
    match client
        .get_token_accounts_by_owner(&pubk.parse().unwrap(), token.mint_address())
        .map_err(|err| SolwalrsError::RpcError(err.to_string()))?
        .first()
        .ok_or_else(|| {
            SolwalrsError::Other(format!("No {token_name} account found for `{short_pubk}`",))
        })?
        .account
        .data
    {
        UiAccountData::Json(ref data) => data
            .parsed
            .get("info")
            .and_then(|info| {
                info.get("tokenAmount").and_then(|token_amount| {
                    token_amount
                        .get("amount")
                        .and_then(|amount| amount.as_str())
                        .and_then(|amount| amount.parse::<u64>().ok())
                })
            })
            .ok_or_else(|| {
                SolwalrsError::Other(format!(
                    "Failed to parse the {token_name} balanace of `{short_pubk}`",
                ))
            }),
        // This should never happen
        _ => Err(SolwalrsError::Other(format!(
            "Failed to parse the {token_name} balanace of `{short_pubk}`",
        ))),
    }
}

/// Request airdrop to the given public key, the amount is in lamports, so 1 SOL = 1_000_000_000 lamports
#[must_use = "This function returns a signature, you should check if the airdrop was successful"]
pub fn request_airdrop(
    args: &AppArgs,
    public_key: &PublicKey,
    amount: u64,
) -> SolwalrsResult<String> {
    crate::info!(
        args,
        "Requesting an airdrop of {} lamports to the keypair `{}`",
        amount,
        short_public_key(public_key)
    );
    let client = rpc_client(args)?;
    let pubk = public_key.as_bytes().to_base58();
    // SAFETY: This is safe because we are sure that the public key is valid
    let signature = client
        .request_airdrop(&pubk.parse().unwrap(), amount)
        .map_err(|err| {
            SolwalrsError::RpcError(format!(
                "Error while requesting an airdrop of {} lamports to the keypair `{}`: {err}",
                amount,
                short_public_key(public_key)
            ))
        })?;
    crate::info!(
        args,
        "Airdrop of {} lamports requested successfully, the singature is `{signature}`",
        amount
    );

    Ok(signature.to_string())
}

/// Confirm the given signature, if the signature is not confirmed, it will wait until it is confirmed
pub fn confirm_signature(args: &AppArgs, signature: &str) -> SolwalrsResult<()> {
    crate::info!(args, "Confirming the signature `{signature}`");
    let client = rpc_client(args)?;
    loop {
        // SAFETY: This is safe because we are sure that the signature is valid
        let status = client
            .confirm_transaction(&signature.parse().unwrap())
            .map_err(|err| {
                SolwalrsError::RpcError(format!(
                    "Error while getting the status of the airdrop singature `{signature}`: {err}"
                ))
            })?;
        if status {
            break;
        }
        crate::info!(args, "Waiting for the airdrop to be confirmed...");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    crate::info!(args, "Signature `{signature}` confirmed successfully");
    Ok(())
}

/// Returns the `solana.fm` rpc parameters
pub fn rpc_params(args: &AppArgs) -> SolwalrsResult<String> {
    let rpc = rpc_url(args)?;
    Ok(url::form_urlencoded::Serializer::new(String::new())
        .append_pair("cluster", &rpc)
        .append_pair("customUrl", &rpc)
        .finish())
}

/// Retuns the transaction on the explorer
pub fn transaction_url(signature: &str, args: &AppArgs) -> SolwalrsResult<String> {
    // encode the rpc url
    let params = rpc_params(args)?;
    Ok(format!("https://solana.fm//tx/{signature}?{params}"))
}

/// Returns the `solana.fm` url of the given public key
pub fn transactions_url(public_key: &PublicKey, args: &AppArgs) -> SolwalrsResult<String> {
    let params = rpc_params(args)?;
    Ok(format!(
        "https://solana.fm/address/{}/transfers?{params}&mode=lite",
        public_key.as_bytes().to_base58()
    ))
}

/// Returns the SOL balance of the given public key
pub fn sol_balance(args: &AppArgs, public_key: &PublicKey) -> SolwalrsResult<u64> {
    crate::info!(
        args,
        "Getting the balance of the keypair `{}`",
        short_public_key(public_key)
    );
    let client = rpc_client(args)?;
    let pubk = public_key.as_bytes().to_base58();
    // SAFETY: This is safe because we are sure that the public key is valid
    client.get_balance(&pubk.parse().unwrap()).map_err(|err| {
        SolwalrsError::RpcError(format!(
            "Error while getting the balance of the keypair `{}`: {err}",
            short_public_key(public_key)
        ))
    })
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
/// The password must be 32 bytes long. will return `Error::InvalidPassword` if the password is not 32 bytes long.
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

/// Shorten the given public key, by replacing the middle with `...`. take the first 4 and last 4 characters.
/// returned string will be base58 of the public key.
pub fn short_public_key(public_key: &PublicKey) -> String {
    let mut public_key = public_key.to_bytes().to_base58();
    public_key.replace_range(4..public_key.len() - 4, "...");
    public_key
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
#[cfg(not(target_os = "android"))]
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
#[cfg(target_os = "android")]
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
