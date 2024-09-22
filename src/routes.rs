use async_sqlite::rusqlite::params;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    Json,
};
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct UrlRequest {
    url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    short_url: String,
}

pub async fn shorten_url(State(state): State<Arc<AppState>>, Json(payload): Json<UrlRequest>) -> Result<Json<UrlResponse>, StatusCode> {
    // Validate URL
    if !is_valid_url(&payload.url) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let short_code = generate_short_code();
    let db = state.db.clone();

    let now = Utc::now().to_rfc3339();
    let short_code_cloned = short_code.clone();
    db.conn(move |c| {
        c.execute(
            "INSERT INTO urls (short_code, original_url, created_at, last_accessed) VALUES (?, ?, ?, ?)",
            params![short_code_cloned, payload.url, now, now],
        )
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UrlResponse {
        short_url: format!("https://s.phill030.de/{}", short_code),
    }))
}

pub async fn redirect_url(State(state): State<Arc<AppState>>, Path(short_code): Path<String>) -> Result<Redirect, StatusCode> {
    let db = state.db.clone();
    let now = Utc::now().to_rfc3339();

    let short_code_cloned = short_code.clone();
    let original_url: String = db
        .conn(move |c| {
            c.query_row(
                "SELECT original_url FROM urls WHERE short_code = ? LIMIT 1",
                params![short_code_cloned],
                |row| row.get(0),
            )
        })
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    db.conn(move |c| c.execute("UPDATE urls SET last_accessed = ? WHERE short_code = ?", params![now, short_code]))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&original_url))
}

fn is_valid_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(parsed_url) => parsed_url.scheme() == "http" || parsed_url.scheme() == "https",
        Err(_) => false,
    }
}

fn generate_short_code() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect()
}
