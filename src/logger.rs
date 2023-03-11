// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use env_logger::{Builder, Env};
use log::Level;

use super::Config;

pub fn init(config: &Config) {
    let log_level: Level = if cfg!(debug_assertions) && config.log_level != Level::Trace {
        Level::Debug
    } else {
        config.log_level
    };

    Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();
}
