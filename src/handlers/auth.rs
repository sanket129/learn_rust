use axum::{Json, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use crate::auth::SECRET;


#[derive(Deserialize)]
pub struct LoginReq {
    user_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct LoginRes {
    token: String,
}

pub async fn login(Json(req): Json<LoginReq>) -> Result<Json<LoginRes>, (StatusCode,String)> {
    let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    let claims = Claims { sub: req.user_id.to_string(), exp};
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&SECRET)).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(LoginRes { token }))
}
