// Lesson: the signing secret, shared by every place that issues or verifies a
// token. A `const` is baked in at compile time and can never read the
// environment, so this is a `static LazyLock` instead: the closure runs once on
// first access, reads JWT_SECRET, and the result is cached for the process.
// The fallback only exists so `cargo run` works without a .env — never ship it.

use std::sync::LazyLock;

pub static SECRET: LazyLock<Vec<u8>> = LazyLock::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret".to_string())
        .into_bytes()
});
