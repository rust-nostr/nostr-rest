// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use actix_web::{get, post, web, HttpResponse};
use nostr_sdk::hashes::sha256::Hash as Sha256Hash;
use nostr_sdk::hashes::Hash;
use nostr_sdk::{Event, Filter};
use redis::AsyncCommands;
use serde_json::json;

use crate::AppState;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "pong",
        "data": {},
    }))
}

#[post("/event")]
pub async fn publish_event(data: web::Data<AppState>, body: web::Json<Event>) -> HttpResponse {
    let event: Event = body.0;

    if let Err(e) = event.verify() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": e.to_string(),
            "data": {},
        }));
    }

    match data.client.send_event(event).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Event published",
            "data": {},
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": e.to_string(),
            "data": {},
        })),
    }
}

#[post("/events")]
pub async fn get_events(data: web::Data<AppState>, body: web::Json<Vec<Filter>>) -> HttpResponse {
    let filters: Vec<Filter> = body.0;

    if let Some(redis) = &data.redis {
        let mut connection = redis.get_async_connection().await.unwrap();
        let hash: String = Sha256Hash::hash(format!("{filters:?}").as_bytes()).to_string();
        match connection.exists::<&str, bool>(&hash).await {
            Ok(exists) => {
                if exists {
                    match connection.get(&hash).await {
                        Ok(result) => {
                            let bytes: Vec<u8> = result;
                            let events: Vec<Event> = bincode::deserialize(&bytes).unwrap();
                            HttpResponse::Ok().json(json!({
                                "success": true,
                                "message": format!("Got {} events", events.len()),
                                "data": events,
                            }))
                        }
                        Err(e) => HttpResponse::BadRequest().json(json!({
                            "success": false,
                            "message": e.to_string(),
                            "data": {},
                        })),
                    }
                } else {
                    match data.client.get_events_of(filters, None).await {
                        Ok(events) => {
                            let encoded: Vec<u8> = bincode::serialize(&events).unwrap();
                            let _: () = connection
                                .set_ex(hash, encoded, data.config.redis.expiration)
                                .await
                                .unwrap();
                            HttpResponse::Ok().json(json!({
                                "success": true,
                                "message": format!("Got {} events", events.len()),
                                "data": events,
                            }))
                        }
                        Err(e) => HttpResponse::BadRequest().json(json!({
                            "success": false,
                            "message": e.to_string(),
                            "data": {},
                        })),
                    }
                }
            }
            Err(e) => HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": e.to_string(),
                "data": {},
            })),
        }
    } else {
        // TODO: add a timeout
        match data.client.get_events_of(filters, None).await {
            Ok(events) => HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Got {} events", events.len()),
                "data": events,
            })),
            Err(e) => HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": e.to_string(),
                "data": {},
            })),
        }
    }
}
