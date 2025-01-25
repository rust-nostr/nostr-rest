// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::net::SocketAddr;
use std::time::Duration;

use nostr_sdk::{RelayUrl, Url};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Network {
    pub listen_addr: SocketAddr,
    pub permissive_cors: bool,
}

#[derive(Deserialize)]
pub struct ConfigFileNetwork {
    pub listen_addr: Option<SocketAddr>,
    pub permissive_cors: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Limit {
    pub max_filters: usize,
    //pub max_events_per_filter: usize,
}

#[derive(Deserialize)]
pub struct ConfigFileLimit {
    pub max_filters: Option<usize>,
    //pub max_events_per_filter: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Nostr {
    pub relays: Vec<RelayUrl>,
    pub discovery: Vec<RelayUrl>,
    pub gossip: bool,
    pub fetch_timeout: Duration,
}

#[derive(Deserialize)]
pub struct ConfigFileNostr {
    pub relays: Vec<RelayUrl>,
    pub discovery: Vec<RelayUrl>,
    pub gossip: bool,
    #[serde(rename = "fetch-timeout")]
    pub fetch_timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub enabled: bool,
    pub endpoint: Url,
    pub expiration: u64,
}

#[derive(Deserialize)]
pub struct ConfigFileRedis {
    pub enabled: Option<bool>,
    pub endpoint: Option<Url>,
    pub expiration: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub log_level: tracing::Level,
    pub network: Network,
    pub limit: Limit,
    pub nostr: Nostr,
    pub redis: Redis,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    pub log_level: Option<String>,
    pub network: ConfigFileNetwork,
    pub limit: ConfigFileLimit,
    pub nostr: ConfigFileNostr,
    pub redis: ConfigFileRedis,
}
