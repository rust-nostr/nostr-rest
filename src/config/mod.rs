// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use nostr_sdk::Url;
use tracing::Level;

pub mod model;

pub use self::model::Config;
use self::model::{ConfigFile, Limit, Network, Nostr, Redis};

fn default_dir() -> PathBuf {
    let home: PathBuf = dirs::home_dir().unwrap_or_else(|| {
        panic!("Unknown home directory");
    });
    let path = home.join(".nostr-rest");
    fs::create_dir_all(&path).expect("Impossible to create default dir");
    path
}

fn default_config_file() -> PathBuf {
    let mut default = default_dir().join("config");
    default.set_extension("toml");

    if default.exists() {
        default
    } else {
        let path = PathBuf::from("config.toml");
        if path.exists() {
            path
        } else {
            panic!("Config file not found.");
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    config: Option<PathBuf>,
}

impl Config {
    pub fn get() -> Self {
        let args: Args = Args::parse();

        let config_file_path: PathBuf = args.config.unwrap_or_else(default_config_file);
        let content = fs::read_to_string(config_file_path).expect("Impossible to read config file");
        let config_file: ConfigFile =
            toml::from_str(&content).expect("Impossible to parse config file");

        let log_level: Level = match config_file.log_level {
            Some(log_level) => Level::from_str(log_level.as_str()).unwrap_or(Level::INFO),
            None => Level::INFO,
        };

        Self {
            log_level,
            network: Network {
                listen_addr: config_file.network.listen_addr.unwrap_or_else(|| {
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7773)
                }),
                permissive_cors: config_file.network.permissive_cors.unwrap_or(false),
            },
            limit: Limit {
                max_filters: config_file.limit.max_filters.unwrap_or(10),
                //max_events_per_filter: config_file.limit.max_events_per_filter.unwrap_or(100),
            },
            nostr: Nostr {
                relays: config_file.nostr.relays,
                discovery: config_file.nostr.discovery,
                gossip: config_file.nostr.gossip,
            },
            redis: Redis {
                enabled: config_file.redis.enabled.unwrap_or(false),
                endpoint: config_file.redis.endpoint.unwrap_or_else(|| {
                    Url::parse("redis://127.0.0.1").expect("Invalid default redis endpoint")
                }),
                expiration: config_file.redis.expiration.unwrap_or(60),
            },
        }
    }
}
