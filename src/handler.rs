// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use actix_web::{get, post, web, HttpResponse};
use nostr_sdk::Event;
use serde_json::json;

use crate::AppState;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "code": 200,
        "message": "NDK Rest API",
        "data": {
            "version": env!("CARGO_PKG_VERSION")
        },
    }))
}

#[post("/event")]
pub async fn publish_event(data: web::Data<AppState>, body: web::Json<Event>) -> HttpResponse {
    let event: Event = body.0;

    if let Err(e) = event.verify() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "code": 400,
            "message": e.to_string(),
            "data": {},
        }));
    }

    match data.client.send_event(event).await {
        Ok(event_id) => HttpResponse::Ok().json(json!({
            "success": true,
            "code": 200,
            "message": "Event published",
            "data": {
                "event_id": event_id
            },
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "code": 400,
            "message": e.to_string(),
            "data": {},
        })),
    }
}
