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

use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use super::{Price, Tokens};
use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
    utils,
};

/// The chche file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    /// Prices of tokens
    pub prices: Vec<Price>,
}

impl Cache {
    /// Create a new cache instance
    pub fn new() -> Self {
        Self { prices: Vec::new() }
    }

    /// Load the cache from cache file, or create a new one if it doesn't exist
    pub fn load(args: &AppArgs) -> SolwalrsResult<Cache> {
        crate::info!(args, "Loading cache file");
        let cache_file = super::utils::app_cache_file_path()?;
        if cache_file.exists() {
            crate::info!(
                args,
                "Cache file exists, loading it `{}`",
                cache_file.display()
            );
            let file = File::open(cache_file)
                .map_err(|e| SolwalrsError::IO(format!("Failed to open cache file: {e}")))?;
            let reader = BufReader::new(file);
            let mut cache: Cache = serde_json::from_reader(reader)
                .map_err(|e| SolwalrsError::IO(format!("Failed to load cache file: {e}")))?;
            cache.clear_prices();
            crate::info!(args, "Cache file loaded: {:#?}", cache);
            Ok(cache)
        } else {
            crate::info!(
                args,
                "Cache file doesn't exist, creating a new one `{}`",
                cache_file.display()
            );
            Ok(Self::new())
        }
    }

    /// Clear prices from cache, old than 5 minutes
    /// This is to prevent the cache from getting too big
    pub fn clear_prices(&mut self) {
        self.prices
            .retain(|price| price.timestamp + (5 * 60) > utils::get_timestamp());
    }

    /// Add a price to the cache, returns the added price
    pub fn add_price(&mut self, price: Price) -> &Price {
        // Check if the price have a symbol
        if price.symbol.is_none() {
            // Should never happen, but if it does, panic to notifiy the developer to fix it
            panic!("Price symbol is None");
        }
        // Remove old prices of the same token
        self.prices.retain(|p| p != &price);
        self.prices.push(price);
        self.prices.last().unwrap()
    }

    /// Get a price from the cache, if it exists, pass `None` to token to get the price of SOL
    pub fn get_price(&self, token: Option<&Tokens>, args: &AppArgs) -> Option<&Price> {
        crate::info!(args, "Getting price from cache");
        let symbol = token.map(Tokens::name).unwrap_or("sol").to_uppercase();
        let price = self
            .prices
            .iter()
            .find(|price| price.symbol.as_ref().expect("Price symbol is None") == &symbol);
        crate::info_or_warn!(args, price, "Price found in cache: {price:?}"; "Price not found in cache");
        price
    }

    /// Save the cache to cache file
    pub fn save(&mut self, args: &AppArgs) -> SolwalrsResult<()> {
        crate::info!(args, "Saving cache file");
        let cache_file = super::utils::app_cache_file_path()?;
        let file = File::create(cache_file)
            .map_err(|e| SolwalrsError::IO(format!("Failed to create cache file: {e}")))?;
        serde_json::to_writer_pretty(file, self)
            .map_err(|e| SolwalrsError::IO(format!("Failed to save cache file: {e}")))?;
        crate::info!(args, "Cache file saved");
        Ok(())
    }
}
