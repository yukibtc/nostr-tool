use clap::Args;
use nostr_sdk::Result;

use crate::error::Error;
use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct RecommendRelaySubCommand {
    /// Relay URL to recommend
    #[arg(short, long)]
    url: String,
}

pub fn recommend_relay(
    private_key: Option<String>,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &RecommendRelaySubCommand,
) -> Result<()> {
    if relays.is_empty() {
        return Err(Error::NoRelay.into());
    }

    let keys = handle_keys(private_key)?;
    let client = create_client(&keys, relays, difficulty_target)?;

    client.add_recommended_relay(sub_command_args.url.clone())?;
    println!("Relay {} recommended", sub_command_args.url);

    Ok(())
}
