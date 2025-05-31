use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::json;

pub async fn matrix_server() -> Response {
    let body = json!({
        "m.server": "matrix.typester.dev:443"
    });

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        ],
        body.to_string(),
    )
        .into_response()
}

pub async fn matrix_client() -> Response {
    let body = json!({
        "m.homeserver": {
            "base_url": "https://matrix.typester.dev"
        },
        "org.matrix.msc3575.proxy": {
            "url": "https://sync.typester.dev"
        }
    });

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        ],
        body.to_string(),
    )
        .into_response()
}
