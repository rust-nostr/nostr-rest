// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use nostr_sdk::{Client, Options, Result};
use redis::Client as RedisClient;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod config;
mod error;
mod handler;
mod logger;

use self::config::Config;

#[derive(Clone)]
pub struct AppState {
    config: Config,
    client: Client,
    redis: Option<RedisClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::get();

    #[cfg(debug_assertions)]
    println!("{:?}\n", config);

    logger::init(&config);

    let opts: Options = Options::new().gossip(config.nostr.gossip);
    let client: Client = Client::builder().opts(opts).build();

    for relay in config.nostr.relays.iter() {
        client.add_relay(relay).await?;
    }

    for relay in config.nostr.discovery.iter() {
        client.add_discovery_relay(relay).await?;
    }

    client.connect().await;

    let redis: Option<RedisClient> = if config.redis.enabled {
        Some(RedisClient::open(config.redis.endpoint.clone())?)
    } else {
        None
    };

    let state = AppState {
        config: config.clone(),
        client,
        redis,
    };

    let app = Router::new()
        .route("/ping", get(handler::ping))
        .route("/event", post(handler::publish_event))
        .route("/event/{event_id}", get(handler::get_event_by_id))
        .route("/events", post(handler::get_events))
        .layer(if config.network.permissive_cors {
            CorsLayer::permissive()
        } else {
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any)
                .max_age(Duration::from_secs(3600))
        })
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.network.listen_addr).await?;

    tracing::info!("REST API listening on {}", listener.local_addr()?);

    Ok(axum::serve(listener, app).await?)
}
