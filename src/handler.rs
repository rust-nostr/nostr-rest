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

    if let Err(e) = event.verify() {
        return Ok(AppJson(json!({
            "success": false,
            "message": e.to_string(),
            "data": {},
        })));
    }

    match state.client.send_event(event).await {
        Ok(_) => Ok(AppJson(json!({
            "success": true,
            "message": "Event published",
            "data": {},
        }))),
        Err(e) => Ok(AppJson(json!({
            "success": false,
            "message": e.to_string(),
            "data": {},
        }))),
    }
}

pub async fn get_events(
    state: State<AppState>,
    body: AppJson<Vec<Filter>>,
) -> Result<AppJson<Value>, AppError> {
    let filters: Vec<Filter> = body.0;

    if filters.len() > state.config.limit.max_filters {
        return Ok(AppJson(json!({
            "success": false,
            "message": format!("Too many filters (max allowed {})", state.config.limit.max_filters),
            "data": {},
        })));
    }

    if let Some(redis) = &state.redis {
        let mut connection = redis.get_async_connection().await.unwrap();
        let hash: String = Sha256Hash::hash(format!("{filters:?}").as_bytes()).to_string();
        match connection.exists::<&str, bool>(&hash).await {
            Ok(exists) => {
                if exists {
                    match connection.get(&hash).await {
                        Ok(result) => {
                            let bytes: Vec<u8> = result;
                            let events: Vec<Event> = bincode::deserialize(&bytes).unwrap();
                            Ok(AppJson(json!({
                                "success": true,
                                "message": format!("Got {} events", events.len()),
                                "data": events,
                            })))
                        }
                        Err(e) => Ok(AppJson(json!({
                            "success": false,
                            "message": e.to_string(),
                            "data": {},
                        }))),
                    }
                } else {
                    match state.client.get_events_of(filters, None).await {
                        Ok(events) => {
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
                        Err(e) => Ok(AppJson(json!({
                            "success": false,
                            "message": e.to_string(),
                            "data": {},
                        }))),
                    }
                }
            }
            Err(e) => Ok(AppJson(json!({
                "success": false,
                "message": e.to_string(),
                "data": {},
            }))),
        }
    } else {
        // TODO: add a timeout
        match state.client.get_events_of(filters, None).await {
            Ok(events) => Ok(AppJson(json!({
                "success": true,
                "message": format!("Got {} events", events.len()),
                "data": events,
            }))),
            Err(e) => Ok(AppJson(json!({
                "success": false,
                "message": e.to_string(),
                "data": {},
            }))),
        }
    }
}
