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

use super::Tokens;
use crate::{
    app::AppArgs,
    errors::{Error as SolwalrsError, Result as SolwalrsResult},
};

const PRICE_API: &str = "https://api.solscan.io/market?symbol=";

/// Data that contains the price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    /// The price
    #[serde(rename = "priceUsdt")]
    pub price: f64,
    /// The price change in the last 24 hours
    #[serde(rename = "priceChange24h")]
    pub price_change_24h: f64,
}

/// Price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    /// Whether the request was successful
    pub success: bool,
    /// Data that contains the price
    pub data: Data,
}

impl Price {
    /// Returns the price, pass `None` to `token` to get SOL price
    pub fn new(token: Option<&Tokens>, args: &AppArgs) -> SolwalrsResult<Self> {
        crate::info!(args, "Getting price data...");
        // Send a GET request to the price API, and parse the response
        let response = reqwest::blocking::get(&format!(
            "{}{}",
            PRICE_API,
            token
                .map(|t| t.name().to_uppercase())
                .unwrap_or_else(|| "SOL".to_owned())
        ))
        .map_err(|e| SolwalrsError::RequestError(e.to_string()))?;
        crate::info!(args, "Got price data {response:?}");
        response
            .json::<Self>()
            .map_err(|e| SolwalrsError::Other(format!("Failed to parse price data: {e}")))
    }
}
