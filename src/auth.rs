// Lesson: everything auth needs in one module — the signing secret, the token
// claims, and (next) the extractor that verifies a Bearer token. It lives apart
// from the /login handler because both issuing and verifying depend on it, and
// a handler is the wrong home for shared machinery.

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// A `const` is baked in at compile time and can never read the environment, so
// this is a `static LazyLock` instead: the closure runs once on first access,
// reads JWT_SECRET, and the result is cached for the process. The fallback only
// exists so `cargo run` works without a .env — never ship it.
pub static SECRET: LazyLock<Vec<u8>> = LazyLock::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret".to_string())
        .into_bytes()
});

// The JWT payload. `sub` is a string because RFC 7519 defines it as one, and
// `exp` is a Unix timestamp that jsonwebtoken checks automatically on decode.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// A custom extractor: reads `Authorization: Bearer <token>`, verifies the
// signature and expiry, and rejects with 401 otherwise. Any handler taking
// `user: AuthUser` is therefore protected by construction — the check lives in
// the signature rather than the body, so it cannot be forgotten.
pub struct AuthUser(pub u32);

// FromRequestParts (not FromRequest) because Parts is only the headers. That
// leaves room for a Json body extractor in the same handler, which the document
// upload endpoint will need.
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    // What gets sent back when verification fails — you choose both the status
    // and the body, which is how the 400/401/422 distinctions get made.
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 1. Find the Authorization header. Missing -> 401.
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "missing Authorization header".into()))?;

        // 2. Strip "Bearer " -> raw token. Wrong prefix -> 401.
        let token = header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "expected Bearer token".into()))?;

        // 3. Verify signature + expiry. Bad or expired -> 401.
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&SECRET),
            &Validation::default(),
        )
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        // 4. sub is a String per RFC 7519 but the app wants a number, so parse
        //    it once here instead of in every handler that reads the id.
        Ok(AuthUser(data.claims.sub.parse().map_err(|_| {
            (StatusCode::UNAUTHORIZED, "sub is not a number".to_string())
        })?))
    }
}

