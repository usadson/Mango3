// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use axum::{
    extract::State, handler::HandlerWithoutStateExt, http::StatusCode, routing::*, Json, Router,
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

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
) -> Json<AnalyzeResponsePayload> {
    let input = &mut Input::new(&payload.text);
    let document = parse_document(input, &state.catalog);
    let analysis = mango3_analysis::analyze(&document, &state.catalog);
    Json(AnalyzeResponsePayload { analysis })
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
