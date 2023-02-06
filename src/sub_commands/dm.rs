use std::str::FromStr;

use clap::Args;
use nostr_sdk::prelude::*;

use crate::error::Error;
use crate::utils::{create_client, parse_key};

#[derive(Args)]
pub struct SendDirectMessageSubCommand {
    /// Receiver public key. Both hex and bech32 encoded keys are supported.
    #[arg(short, long)]
    receiver: String,
    /// Message to send
    #[arg(short, long)]
    message: String,
}

pub fn send(
    keys: Keys,
    relays: Vec<String>,
    difficulty_target: u8,
    sub_command_args: &SendDirectMessageSubCommand,
) -> Result<()> {
    if relays.is_empty() {
        return Err(Error::NoRelay.into());
    }

    let client = create_client(&keys, relays, difficulty_target)?;

    let hex_pubkey = parse_key(sub_command_args.receiver.clone())?;
    let receiver = XOnlyPublicKey::from_str(&hex_pubkey)?;

    let event_id = client.send_direct_msg(receiver, sub_command_args.message.clone())?;
    println!(
        "Message sent to {}, event id: {}",
        receiver.to_bech32()?,
        event_id.to_bech32()?
    );
    Ok(())
}
