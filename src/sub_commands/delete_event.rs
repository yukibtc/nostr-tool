use clap::Args;
use nostr_sdk::prelude::*;

use crate::error::Error;
use crate::utils::{create_client, parse_key};

#[derive(Args)]
pub struct DeleteEventSubCommand {
    /// Event id to delete
    #[arg(short, long)]
    event_id: String,
    /// Reason for deleting the events
    #[arg(short, long)]
    reason: Option<String>,
}

pub fn delete(
    keys: Keys,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &DeleteEventSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        return Err(Error::NoRelay.into());
    }

    let client = create_client(&keys, relays, difficulty_target)?;

    let event_id_to_delete_hex = parse_key(sub_command_args.event_id.clone())?;
    let event_id = EventId::from_hex(event_id_to_delete_hex)?;

    let event_id = client.delete_event(event_id, sub_command_args.reason.clone())?;
    println!("Deleted event with id: {}", event_id.to_bech32()?);
    Ok(())
}
