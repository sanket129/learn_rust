use std::sync::LazyLock;

pub static SECRET: LazyLock<Vec<u8>> = LazyLock::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret".to_string())
        .into_bytes()
});
