use clap::Parser;

use crate::errors::Result as SolwalrsResult;
use crate::wallet::Wallet;
// use crate::app::commands::utils::

/// set a keypair as the default
#[derive(Debug, Parser)]
pub struct DefaultCommand {
    /// The name of the keypair to set as default
    pub name: String,
}

impl DefaultCommand {
    /// set a keypair as the default
    /// Note: You need to export the wallet after running this command, using `Wallet::export`
    pub fn run(&self, wallet: &mut Wallet) -> SolwalrsResult<()> {
        let keypair = wallet.get_keypair(&self.name)?.clone();
        wallet.delete_keypair(&self.name)?;
        wallet.add_keypair(keypair, true)?;
        Ok(())
    }
}
