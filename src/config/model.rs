// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use std::fmt;
use std::net::SocketAddr;

use serde::Deserialize;

use nostr_sdk::Url;

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
    pub max_events_per_filter: usize,
}

#[derive(Deserialize)]
pub struct ConfigFileLimit {
    pub max_filters: Option<usize>,
    pub max_events_per_filter: Option<usize>,
}

#[derive(Clone, Deserialize)]
pub struct Nostr {
    pub relays: Vec<Url>,
}

impl fmt::Debug for Nostr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let relays: Vec<String> = self.relays.iter().map(|r| r.to_string()).collect();
        write!(f, "{{ relays: {:?} }}", relays)
    }
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub enabled: bool,
    pub endpoint: Url,
    pub expiration: usize,
}

#[derive(Deserialize)]
pub struct ConfigFileRedis {
    pub enabled: Option<bool>,
    pub endpoint: Option<Url>,
    pub expiration: Option<usize>,
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
    pub nostr: Nostr,
    pub redis: ConfigFileRedis,
}
