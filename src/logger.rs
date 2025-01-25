// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use tracing::Level;

use super::Config;

pub fn init(config: &Config) {
    let log_level: Level = if cfg!(debug_assertions) && config.log_level != Level::TRACE {
        Level::DEBUG
    } else {
        config.log_level
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();
}
