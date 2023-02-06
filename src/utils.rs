use nostr_sdk::blocking::Client;
use nostr_sdk::prelude::*;

pub fn create_client(keys: &Keys, relays: Vec<String>, difficulty: u8) -> Result<Client> {
    let opts = Options::new().wait_for_send(true).difficulty(difficulty);
    let client = Client::new_with_opts(keys, opts);
    let relays = relays.iter().map(|url| (url, None)).collect();
    client.add_relays(relays)?;
    client.connect();
    Ok(client)
}

pub fn parse_key(key: String) -> Result<String> {
    // Check if the key is a bech32 encoded key
    let key = if key.starts_with("npub") {
        XOnlyPublicKey::from_bech32(key)?.to_string()
    } else if key.starts_with("nsec") {
        SecretKey::from_bech32(key)?.display_secret().to_string()
    } else if key.starts_with("note") {
        EventId::from_bech32(key)?.to_string()
    } else {
        // If the key is not bech32 encoded, return it as is
        key
    };
    Ok(key)
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Prefix {
    Npub,
    Nsec,
    Note,
}
