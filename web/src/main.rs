// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Instant;

use axum::{
    extract::{State, Path},
    handler::HandlerWithoutStateExt,
    http::{StatusCode, HeaderMap},
    routing::*,
    Json,
    Router,
    response::{IntoResponse, Html},
};

use mango3_analysis::Report;
use mango3_catalog::{Catalog, WordTrait};
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
        .route("/info/verb/:verb", get(info_verb))
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

async fn info_verb(
    State(state): State<AppState>,
    Path(verb_query): Path<String>,
) -> impl IntoResponse {
    let Some((_, verb_word)) = state.catalog.find(&verb_query) else {
        return (StatusCode::NOT_FOUND, Html("Verb not found!".into()));
    };

    let Some(WordTrait::Verb { verb }) = verb_word.traits
            .iter()
            .find(|x| matches!(x, WordTrait::Verb {..})) else {
        return (StatusCode::NOT_FOUND, Html("Not a verb".into()));
    };

    let conjugations = state.catalog.find_indicative_conjugations(*verb);

    let mut html = r#"
       <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" crossorigin="anonymous">
        <table class=table>
            <thead>
                <tr>
                    <th>Vorm</th>
                    <th>Waarde</th>
                </tr>
            </head>
            <tbody>
    "#.to_string();

    html += &format!("<tr><td>Ik</td><td>{}</td></tr>", conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::FirstPerson, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", "));
    html += &format!("<tr><td>Jij</td><td>{}</td></tr>", conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::SecondPersonJe, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", "));
    html += &format!("<tr><td>U</td><td>{}</td></tr>", conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::SecondPersonU, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", "));
    html += &format!("<tr><td>Ge</td><td>{}</td></tr>", conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::SecondPersonGe, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", "));
    html += &format!("<tr><td>Hij/Zij</td><td>{}</td></tr>", conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::ThirdPerson, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", "));
    let plural = conjugations.iter().filter(|x| x.is_conjugation(mango3_catalog::ConjugationKind::Plural, false)).map(|x| x.text.as_ref()).collect::<Vec<&str>>().join(", ");
    html += &format!("<tr><td>Wij</td><td>{plural}</td></tr><tr><td>Jullie</td><td>{plural}</td></tr><tr><td>Zij</td><td>{plural}</td></tr>");

    html += "</tbody></table>";


    (StatusCode::OK, Html(html))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
