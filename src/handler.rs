// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use actix_web::{get, post, web, HttpResponse};
use nostr_sdk::{Event, Filter};
use serde_json::json;

use crate::AppState;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "code": 200,
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
            "code": 400,
            "message": e.to_string(),
            "data": {},
        }));
    }

    match data.client.send_event(event).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "code": 200,
            "message": "Event published",
            "data": {},
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "code": 400,
            "message": e.to_string(),
            "data": {},
        })),
    }
}

#[post("/events")]
pub async fn get_events(data: web::Data<AppState>, body: web::Json<Vec<Filter>>) -> HttpResponse {
    let filters: Vec<Filter> = body.0;

    // TODO: add a timeout
    match data.client.get_events_of(filters, None).await {
        Ok(events) => HttpResponse::Ok().json(json!({
            "success": true,
            "code": 200,
            "message": "Events obtained successfully",
            "data": events,
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "code": 400,
            "message": e.to_string(),
            "data": {},
        })),
    }
}
