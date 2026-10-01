// Lesson: a protected route. There is no `if logged_in` check in the body —
// taking `user: AuthUser` is the check. The `sub` claim from the token becomes
// the response, so this endpoint can only ever answer for the caller.

use crate::auth::AuthUser;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Me {
    user_id: u32,
}

pub async fn me(user: AuthUser) -> Json<Me> {
    Json(Me { user_id: user.0 })
}
