// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use std::fmt;
use std::net::SocketAddr;

use serde::Deserialize;

use nostr_sdk::Url;

#[derive(Debug)]
pub struct Network {
    pub listen_addr: SocketAddr,
}

#[derive(Deserialize)]
pub struct ConfigFileNetwork {
    pub listen_addr: Option<SocketAddr>,
}

/* #[derive(Debug)]
pub struct Limit {}

#[derive(Deserialize)]
pub struct ConfigFileLimit {} */

#[derive(Deserialize)]
pub struct Nostr {
    pub relays: Vec<Url>,
}

impl fmt::Debug for Nostr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let relays: Vec<String> = self.relays.iter().map(|r| r.to_string()).collect();
        write!(f, "{{ relays: {:?} }}", relays)
    }
}

#[derive(Debug)]
pub struct Config {
    pub log_level: log::Level,
    pub network: Network,
    //pub limit: Limit,
    pub nostr: Nostr,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    pub log_level: Option<String>,
    pub network: ConfigFileNetwork,
    //pub limit: ConfigFileLimit,
    pub nostr: Nostr,
}
