// Lesson: async sleep yields the thread — 100 concurrent sleeps share 1 thread.
// Expect ~1000 req/s at c=100, same single vs multi.

use std::time::Duration;
pub async fn sleep_hello() -> &'static str {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "Slept"
}
