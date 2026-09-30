// Lesson: two extractors in one signature. `Path<u32>` takes the `{id}` segment
// and rejects non-numeric input with 400 before the body ever runs; `Query`
// parses `?active=true`, and `#[serde(default)]` makes the field optional so
// the parameter can be omitted. Note `{id}` — axum 0.8 replaced 0.7's `:id`,
// and the old form compiles but panics at startup.

use axum::{Json,extract::{Path,Query}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserQuery {
    #[serde(default)]
    active: bool,
}

#[derive(Serialize)]
pub struct User {
    id: u32,
    active: bool,
}

pub async fn user(Path(id):Path<u32>, Query(q): Query<UserQuery>) -> Json<User> {
    Json(User {id, active: q.active})
}
