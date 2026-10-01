// Lesson: everything auth needs in one module — the signing secret, the token
// claims, and (next) the extractor that verifies a Bearer token. It lives apart
// from the /login handler because both issuing and verifying depend on it, and
// a handler is the wrong home for shared machinery.

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

