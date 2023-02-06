use clap::Args;
use nostr_sdk::prelude::*;

use crate::error::Error;
use crate::utils::create_client;

#[derive(Args)]
pub struct RecommendRelaySubCommand {
    /// Relay URL to recommend
    #[arg(short, long)]
    url: String,
}

pub fn recommend_relay(
    keys: Keys,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &RecommendRelaySubCommand,
) -> Result<()> {
    if relays.is_empty() {
        return Err(Error::NoRelay.into());
    }

    let client = create_client(&keys, relays, difficulty_target)?;

    client.add_recommended_relay(sub_command_args.url.clone())?;
    println!("Relay {} recommended", sub_command_args.url);

    Ok(())
}
