// Lesson: plain-text handler. Returns &'static str, axum converts it to 200 OK.
// Baseline: ~161k req/s single-thread (see docs/benchmarks.md Run 1).

pub async fn hello() -> &'static str{
    "Hello"
}
