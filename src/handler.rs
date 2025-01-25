// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::hash::{DefaultHasher, Hash, Hasher};

use axum::extract::{Path, State};
use axum::response::Json;
use nostr_sdk::{Event, EventId, Filter};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::{AppError, AppJson};
use crate::AppState;

#[derive(Deserialize)]
pub struct GetEventByIdParams {
    event_id: EventId,
}

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

    let events: Vec<Event> = get_events_by_filters(state, filters).await?;

    Ok(AppJson(json!({
        "success": true,
        "message": format!("Got {} events", events.len()),
        "data": events,
    })))
}

pub async fn get_event_by_id(
    state: State<AppState>,
    path: Path<GetEventByIdParams>,
) -> Result<AppJson<Value>, AppError> {
    let event_id: EventId = path.event_id;
    let filter: Filter = Filter::new().id(event_id);
    let filters: Vec<Filter> = vec![filter];
    let events: Vec<Event> = get_events_by_filters(state, filters).await?;
    let event: &Event = events.first().ok_or(AppError::EventIdNotFound)?;
    Ok(AppJson(json!({
        "success": true,
        "message": "Got 1 events",
        "data": event,
    })))
}

async fn get_events_by_filters(
    state: State<AppState>,
    filters: Vec<Filter>,
) -> Result<Vec<Event>, AppError> {
    if let Some(redis) = &state.redis {
        let mut connection = redis.get_multiplexed_async_connection().await?;

        // Hash filters
        let hash: u64 = make_hash(&filters);

        // Try to get cached result
        match connection.get(hash).await {
            Ok(cached) => {
                let bytes: Vec<u8> = cached;
                let events: Vec<Event> = serde_json::from_slice(&bytes).unwrap(); // TODO: remove unwrap
                Ok(events)
            }
            Err(..) => {
                let events = state
                    .client
                    .fetch_events(filters, state.config.nostr.fetch_timeout)
                    .await?;
                let events: Vec<Event> = events.to_vec();
                let encoded: Vec<u8> = serde_json::to_vec(&events).unwrap();
                let _: () = connection
                    .set_ex(hash, encoded, state.config.redis.expiration)
                    .await?;
                Ok(events)
            }
        }
    } else {
        let events = state
            .client
            .fetch_events(filters, state.config.nostr.fetch_timeout)
            .await?;
        Ok(events.to_vec())
    }
}

fn make_hash<T: Hash>(t: &T) -> u64 {
    let mut s: DefaultHasher = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
