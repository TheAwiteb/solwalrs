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

use clap::ValueEnum;
use name_variant::NamedVariant;
use solana_client::rpc_request::TokenAccountsFilter;

/// The supported spl tokens
#[derive(Debug, Clone, NamedVariant, ValueEnum)]
pub enum Tokens {
    Usdc,
    Usdt,
    Srm,
}

impl Tokens {
    /// Returns the token name
    pub const fn name(&self) -> &'static str {
        self.variant_name()
    }
    /// Returns the token mint address
    pub fn mint_address(&self) -> TokenAccountsFilter {
        use TokenAccountsFilter::Mint;
        use Tokens::*;
        let mint_address = match &self {
            Usdc => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            Usdt => "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
            Srm => "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt",
        };
        // SAFETY: The mint addresses are valid
        Mint(mint_address.parse().unwrap())
    }

    /// Returns the decimals number of the token
    pub fn decimals(&self) -> f64 {
        1e6
    }
}
