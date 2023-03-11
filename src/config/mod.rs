// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser;
use dirs::home_dir;
use log::Level;

pub mod model;

pub use self::model::Config;
use self::model::{ConfigFile, Network, Nostr};

fn default_dir() -> PathBuf {
    let home: PathBuf = home_dir().unwrap_or_else(|| {
        log::error!("Unknown home directory");
        std::process::exit(1)
    });
    home.join(".ndk-rest")
}

fn default_config_file() -> PathBuf {
    let mut default = default_dir().join("config");
    default.set_extension("toml");
    default
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

        let config_file_path: PathBuf = match args.config {
            Some(path) => path,
            None => default_config_file(),
        };

        let config_file: ConfigFile =
            Self::read_config_file(&config_file_path).expect("Impossible to read config file");

        let log_level: Level = match config_file.log_level {
            Some(log_level) => Level::from_str(log_level.as_str()).unwrap_or(Level::Info),
            None => Level::Info,
        };

        let config = Self {
            log_level,
            network: Network {
                listen_addr: config_file.network.listen_addr.unwrap_or_else(|| {
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7773)
                }),
            },
            //limit: Limit {},
            nostr: Nostr {
                relays: config_file.nostr.relays,
            },
        };

        println!("{config:?}");

        config
    }

    fn read_config_file(path: &Path) -> std::io::Result<ConfigFile> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
