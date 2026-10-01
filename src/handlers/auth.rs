// Lesson: POST /login issues a JWT. The API takes a numeric `user_id` but RFC
// 7519 defines the `sub` claim as a string, so the conversion happens here at
// the signing boundary. Returns Result so a signing failure surfaces as a 500
// rather than a panic; `exp` is one hour out and jsonwebtoken enforces it when
// a token is later decoded.

use crate::auth::{Claims, SECRET};
use axum::{Json, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LoginReq {
    user_id: u32,
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

// Tests live here rather than in src/auth.rs because they use `use super::*`,
// and only this module imports Claims / encode / Header / EncodingKey / Utc.
#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation, decode};

    fn valid_claims() -> Claims {
        Claims {
            sub: "42".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        }
    }

    #[test]
    fn token_round_trips() {
        let token = encode(
            &Header::default(),
            &valid_claims(),
            &EncodingKey::from_secret(&SECRET),
        )
        .unwrap();
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&SECRET),
            &Validation::default(),
        )
        .unwrap();
        assert_eq!(decoded.claims.sub, "42");
    }

    #[test]
    fn expired_token_is_rejected() {
        let expired = Claims {
            sub: "42".to_string(),
            exp: (Utc::now() - Duration::hours(1)).timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &expired,
            &EncodingKey::from_secret(&SECRET),
        )
        .unwrap();
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&SECRET),
            &Validation::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn token_signed_with_another_secret_is_rejected() {
        let other = EncodingKey::from_secret(b"a completely different secret");
        let token = encode(&Header::default(), &valid_claims(), &other).unwrap();
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&SECRET),
            &Validation::default(),
        );
        assert!(result.is_err());
    }
}

