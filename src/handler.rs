// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use axum::extract::State;
use axum::response::Json;
use nostr_sdk::hashes::sha256::Hash as Sha256Hash;
use nostr_sdk::hashes::Hash;
use nostr_sdk::{Event, Filter};
use redis::AsyncCommands;
use serde_json::{json, Value};

use crate::error::{AppError, AppJson};
use crate::AppState;

pub async fn ping() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "pong",
        "data": {},
    }))
}

pub async fn publish_event(
    state: State<AppState>,
    body: AppJson<Event>,
) -> Result<AppJson<Value>, AppError> {
    let event: Event = body.0;
    event.verify()?;
    state.client.send_event(event).await?;
    Ok(AppJson(json!({
        "success": true,
        "message": "Event published",
        "data": {},
    })))
}

pub async fn get_events(
    state: State<AppState>,
    body: AppJson<Vec<Filter>>,
) -> Result<AppJson<Value>, AppError> {
    let filters: Vec<Filter> = body.0;

    if filters.len() > state.config.limit.max_filters {
        return Err(AppError::FilterError(state.config.limit.max_filters));
    }

    if let Some(redis) = &state.redis {
        let mut connection = redis.get_multiplexed_async_connection().await.unwrap();
        let hash: String = Sha256Hash::hash(format!("{filters:?}").as_bytes()).to_string();
        let exists = connection.exists::<&str, bool>(&hash).await?;
        if exists {
            let result = connection.get(&hash).await?;
            let bytes: Vec<u8> = result;
            let events: Vec<Event> = bincode::deserialize(&bytes).unwrap();
            Ok(AppJson(json!({
                "success": true,
                "message": format!("Got {} events", events.len()),
                "data": events,
            })))
        } else {
            let events = state.client.get_events_of(filters, None).await?;
            let encoded: Vec<u8> = bincode::serialize(&events).unwrap();
            let _: () = connection
                .set_ex(hash, encoded, state.config.redis.expiration)
                .await
                .unwrap();
            Ok(AppJson(json!({
                "success": true,
                "message": format!("Got {} events", events.len()),
                "data": events,
            })))
        }
    } else {
        // TODO: add a timeout
        let events = state.client.get_events_of(filters, None).await?;
        Ok(AppJson(json!({
            "success": true,
            "message": format!("Got {} events", events.len()),
            "data": events,
        })))
    }
}
