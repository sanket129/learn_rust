// Lesson: CPU-bound handler. 10M wrapping ops, returns owned String.
// Compute holds the thread: 1 thread = 1 req at a time. Threads scale here.

pub async fn cpu_hello() -> String {
    let mut x: u64 = 0;
    for i in 0..10_000_000 {
        x = x.wrapping_add(i).wrapping_mul(31);
    }
    format!("{x}")
}
