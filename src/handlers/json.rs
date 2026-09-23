// Lesson: JSON handler. Serialize struct -> Json wrapper -> application/json.
// Compare vs plain &str: serialization cost is what threads help with.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloJson {
    message: &'static str,
}
pub async fn json_hello() -> Json<HelloJson> {
    Json(HelloJson {message: "Hello"})
}
