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

use serde::{Deserialize, Serialize};

use super::{cache::Cache, Tokens};
use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
};

const PRICE_API: &str = "https://api.solscan.io/market?symbol=";

/// Data that contains the price
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    /// The price
    #[serde(rename = "priceUsdt")]
    pub price: f64,
    /// The price change in the last 24 hours
    #[serde(rename = "priceChange24h")]
    pub price_change_24h: f64,
}

/// Price data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Price {
    /// Whether the request was successful
    pub success: bool,
    /// Data that contains the price
    pub data: Data,
    /// The timestamp of the request (in milliseconds since the Unix epoch)
    #[serde(default = "crate::utils::get_timestamp")]
    pub timestamp: u64,
    /// The symbol of the token
    pub symbol: Option<String>,
}

impl Price {
    /// Send request to get the price of a token/SOL in USDT
    pub fn send_request(token: Option<&Tokens>, args: &AppArgs) -> SolwalrsResult<Self> {
        crate::info!(args, "Sending request to get price data");
        let symbol = token.map(Tokens::name).unwrap_or_else(|| "SOL".to_owned());
        // Send a GET request to the price API, and parse the response
        let response = reqwest::blocking::get(format!("{PRICE_API}{symbol}",))
            .map_err(|e| SolwalrsError::RequestError(e.to_string()))?;
        crate::info!(args, "Got price data {response:?}");
        let mut price = response
            .json::<Self>()
            .map_err(|e| SolwalrsError::Other(format!("Failed to parse price data: {e}")))?;
        price.symbol = Some(symbol);
        crate::info!(args, "Parsed price data {price:?}");
        Ok(price)
    }

    /// Returns the price, pass `None` to `token` to get SOL price.
    /// If the lsat price data is less than 15 minutes old, it will be used instead of sending a new request.
    pub fn get_price(
        token: Option<&Tokens>,
        args: &AppArgs,
        cache: &mut Cache,
    ) -> SolwalrsResult<Self> {
        cache
            .get_price(token, args)
            .map(|p| Ok(p.clone()))
            .unwrap_or_else(|| Ok(cache.add_price(Self::send_request(token, args)?).clone()))
    }
}
