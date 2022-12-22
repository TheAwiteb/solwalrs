use clap::Parser;

use crate::app::AppArgs;
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
    pub fn run(&self, wallet: &mut Wallet, args: &AppArgs) -> SolwalrsResult<()> {
        crate::info!(args, "Setting `{}` as a default in {:?}", self.name, wallet);
        let mut keypair = wallet.get_keypair(&self.name, args)?.clone();
        crate::info!(
            args,
            "The public key of the keypair is {:?}",
            keypair.public_key
        );
        keypair.is_default = true;
        wallet.delete_keypair(&self.name, args)?;
        wallet.add_keypair(keypair, args)?;
        println!("Done setting `{}` as a default!", self.name);
        Ok(())
    }
}
