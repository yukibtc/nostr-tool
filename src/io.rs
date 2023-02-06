// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use dialoguer::{Confirm, Input, Password};
use nostr_sdk::prelude::*;

pub fn get_input<S>(prompt: S) -> Result<String>
where
    S: Into<String>,
{
    Ok(Input::new().with_prompt(prompt).interact_text()?)
}

pub fn get_secret_key() -> Result<Keys> {
    let sk = Password::new().with_prompt("Secret key").interact()?;
    Ok(Keys::from_sk_str(&sk)?)
}

pub fn ask<S>(prompt: S) -> Result<bool>
where
    S: Into<String> + std::marker::Copy,
{
    if Confirm::new()
        .with_prompt(prompt)
        .default(false)
        .interact()?
    {
        Ok(true)
    } else {
        Ok(false)
    }
}
