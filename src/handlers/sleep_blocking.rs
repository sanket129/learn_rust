// Lesson: BLOCKING sleep inside async fn — holds the thread, kills overlap.
// Same 100ms, ~100x slower than tokio::sleep under load. Never do this.

pub async fn sleep_blocking_hello() -> &'static str {
    std::thread::sleep(std::time::Duration::from_millis(100));
    "blocked"
}
