//! HTTP surface of the query engine. P0 ships /healthz and a stub /search
//! that returns a contract-valid empty response; P1 wires the real index.

use std::time::Instant;

use axum::Router;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::routing::get;
use serde_json::{Value, json};

use loom_contracts::{Contract, validate};

/// Builds the full route table.
pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/search", get(search))
}

async fn healthz() -> Json<Value> {
    Json(json!({ "service": "engine-rust", "status": "ok" }))
}

async fn search(Query(params): Query<Vec<(String, String)>>) -> Response {
    let started = Instant::now();

    // Boundary rule: the request is validated as a contract document even
    // though it arrives as query parameters.
    let mut request = json!({ "schema": "search_request.v1" });
    for (key, value) in params {
        match key.as_str() {
            "q" | "query" => request["query"] = Value::String(value),
            "vertical" => request["vertical"] = Value::String(value),
            "offset" | "limit" => {
                if let Ok(n) = value.parse::<i64>() {
                    request[key] = Value::Number(n.into());
                }
            }
            _ => {} // unknown params rejected below by additionalProperties
        }
    }
    if let Err(violation) = validate(Contract::SearchRequestV1, &request) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": violation.to_string() })),
        )
            .into_response();
    }

    // P1 replaces this stub with real index lookups.
    let response = json!({
        "schema": "search_response.v1",
        "query": request["query"],
        "vertical": request.get("vertical").cloned().unwrap_or_else(|| "devdocs".into()),
        "took_ms": started.elapsed().as_secs_f64() * 1000.0,
        "degraded": false,
        "total": 0,
        "results": [],
    });
    if let Err(violation) = validate(Contract::SearchResponseV1, &response) {
        // We refuse to emit anything that violates our own contract. Detail
        // goes to the log, not the client.
        eprintln!("loom-server: self-emitted contract violation: {violation}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "internal error" })),
        )
            .into_response();
    }
    Json(response).into_response()
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    async fn get_json(uri: &str) -> (StatusCode, Value) {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("handler runs");
        let status = resp.status();
        let bytes = resp
            .into_body()
            .collect()
            .await
            .expect("body reads")
            .to_bytes();
        let value = serde_json::from_slice(&bytes).expect("body is JSON");
        (status, value)
    }

    #[tokio::test]
    async fn healthz_reports_ok() {
        let (status, body) = get_json("/healthz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn search_returns_contract_valid_response() {
        let (status, body) = get_json("/search?q=array+map").await;
        assert_eq!(status, StatusCode::OK);
        validate(Contract::SearchResponseV1, &body).expect("response honors contract");
        assert_eq!(body["degraded"], false);
    }

    #[tokio::test]
    async fn empty_query_is_rejected_by_contract() {
        let (status, _) = get_json("/search?q=").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn oversized_limit_is_rejected_by_contract() {
        let (status, _) = get_json("/search?q=x&limit=500").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
