// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Instant;

use axum::{
    extract::State,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, HeaderMap},
    routing::*,
    Json,
    Router,
    response::IntoResponse,
};

use mango3_analysis::Report;
use mango3_catalog::Catalog;
use mango3_parser::{parse_document, Input};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    catalog: Catalog,
}

#[tokio::main]
async fn main() {
    // Clear Screen
    print!("\x1b[1;1H\x1b[2J");

    println!("Mango3 is loading...");

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("trace")
    ).init();

    let state = AppState {
        catalog: Catalog::new(),
    };

    // you can convert handler function to service
    let fallback_service = handle_404.into_service();
    let fallback_service = ServeDir::new("web/wwwroot").not_found_service(fallback_service);

    let app = Router::new()
        .route("/api/v1/analyze", post(api_analyze))
        .fallback_service(fallback_service)
        .with_state(state);

    let address = "0.0.0.0:8080";

    println!("Listening on {address}");

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct AnalyzeRequestPayload {
    text: String,
}

#[derive(Serialize)]
struct AnalyzeResponsePayload {
    analysis: Vec<Report>,
}

async fn api_analyze(
    State(state): State<AppState>,
    Json(payload): Json<AnalyzeRequestPayload>,
) -> impl IntoResponse {
    let start = Instant::now();

    let input = &mut Input::new(&payload.text);
    let document = parse_document(input, &state.catalog);

    let parse_time = start.elapsed().as_millis();
    let start = Instant::now();

    let analysis = mango3_analysis::analyze(&document, &state.catalog);
    let analysis_time = start.elapsed().as_millis();

    let mut headers = HeaderMap::new();
    headers.append("Server-Timing", format!("parse;dur={parse_time}, analysis;dur={analysis_time}").try_into().unwrap());

    (headers, Json(AnalyzeResponsePayload { analysis }))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
