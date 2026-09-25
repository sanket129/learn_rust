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
