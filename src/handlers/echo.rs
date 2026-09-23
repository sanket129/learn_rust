// Lesson: POST echo. Deserialize body -> owned String -> serialize back.
// Extractors as args auto-reject bad bodies with 4xx.

use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct EchoMsg {
    message: String,
}

pub async fn echo(Json(payload): Json<EchoMsg>) -> Json<EchoMsg> {
    Json(payload)
}
