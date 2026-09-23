# Load-test baselines

Command shape: `oha -n <total> -c <concurrent> <url>` (server always `--release`).

## Run 1 — single-threaded `/greet` (2026-09-11)

- Runtime: `flavor = "current_thread"` (1 OS thread)
- Server: `cargo run --release`, route `/greet` → plain `"hello"` (5 B)
- Load: `oha -n 50000 -c 100 http://127.0.0.1:3000/greet`

| Metric | Value | Meaning |
|--------|-------|---------|
| Success rate | 100.00% | all 50000 got a response |
| Status codes | 50000 × `[200]` | zero 4xx/5xx |
| Requests/sec | **161,038** | headline number — compare all future runs against this |
| Total time | 310 ms | wall time for all 50k requests |
| Average | 0.61 ms | mean latency per request |
| p50 (median) | 0.54 ms | half of requests faster than this |
| p90 | 0.89 ms | 90% faster than this |
| p99 | 1.44 ms | worst 1% starts here — tail latency |
| p99.9 | 4.40 ms | worst 0.1% — outliers (OS scheduling, not your code) |
| Slowest | 5.20 ms | single worst request |
| Size/request | 5 B | body = `hello` |
| DNS+dialup avg | 1.75 ms | one-time TCP+connection setup per connection, not per request |

Takeaway: one Rust thread serves ~161k tiny responses/sec. Next: flip to
`multi_thread` and re-run the same command — speedup ratio goes here.

## Run 2 — multi-threaded `/greet` (2026-09-11)

- Runtime: `flavor = "multi_thread"` (default pool = all cores)
- Same binary shape, same load: `oha -n 50000 -c 100 http://127.0.0.1:3000/greet`

| Metric | Single (run 1) | Multi (run 2) |
|--------|---------------|---------------|
| Requests/sec | 161,038 | **173,039 (+7.5%)** |
| Average | 0.61 ms | 0.57 ms |
| p50 | 0.54 ms | 0.54 ms |
| p99 | 1.44 ms | 1.27 ms |
| Slowest | 5.20 ms | 3.39 ms |

Takeaway: threads barely moved throughput (+7.5%) on a trivial handler.
Why (see chat 2026-09-11): the server was never the bottleneck — see below.

## Run 3 — multi-threaded `/json` (2026-09-12)

- Runtime: `flavor = "multi_thread"`, restructured files (`app.rs` + `handlers/`)
- Route `/json` → `Json(HelloJson { message: "Hello" }}` (19 B)
- Load: `oha -n 50000 -c 100 http://127.0.0.1:3000/json`

| Metric | `/greet` multi (run 2) | `/json` multi (run 3) |
|--------|----------------------|---------------------|
| Requests/sec | 173,039 | **172,121 (-0.5%)** |
| Average | 0.57 ms | 0.57 ms |
| p50 | 0.54 ms | 0.54 ms |
| p99 | 1.27 ms | 1.31 ms |
| Slowest | 3.39 ms | 6.21 ms |
| Size/request | 5 B | 19 B (`{"message":"Hello"}`) |

Takeaway: serializing a tiny struct costs ~nothing — same throughput as
plaintext. Threads still don't matter because there is no waiting or real
compute. Next: `/cpu` or `/sleep` endpoint where work actually happens.

## Run 4 — multi-threaded `/sleep` (100ms async) (2026-09-18)

- Runtime: `flavor = "multi_thread"`, handler awaits
  `tokio::time::sleep(100ms)` then returns `"Slept"` (5 B)
- Load: `oha -n 50000 -c 100 http://127.0.0.1:3000/sleep` (took ~51s)

| Metric | Value | Meaning |
|--------|-------|---------|
| Requests/sec | **977** | ≈ `concurrency / sleep` = 100 / 0.1s = 1000 — math checks out |
| Average | 102.3 ms | 100ms sleep + ~2ms overhead |
| p50 | 102.2 ms | histogram ultra-tight: every request ≈ sleep time |
| p99 | 104.7 ms | almost no tail — nothing to contend on |
| Slowest | 114.0 ms | single outlier |
| Success | 100%, 50000 × `[200]` | async multiplexed all 100 concurrent sleeps |

Takeaway: I/O wait is where async shines — threads don't matter here either,
but for the opposite reason vs `/greet`: one thread alone could do this too
(the 100 sleeps overlap, none blocks). Contrast next: `/sleep-blocking`
(`std::thread::sleep`) will collapse single-thread to ~10 req/s.

## Run 5 — multi-threaded `/sleep_block` (100ms BLOCKING) (2026-09-18)

- Handler awaits nothing: `std::thread::sleep(100ms)` holds the thread
- Load: `oha -n 200 -c 20 http://127.0.0.1:3000/sleep_block` (small n — it's slow)

| Metric | async `/sleep` (run 4) | blocking `/sleep_block` (run 5) |
|--------|----------------------|-------------------------------|
| Requests/sec | 977 | **63.7 (~15x slower)** |
| Average | 102 ms | 295 ms (~3x the sleep itself = queueing) |
| p50 | 102 ms | 308 ms |
| p99 | 105 ms | 435 ms |
| Histogram | spike at 102ms | smeared 100→441ms (queue depth varies) |
| Size/req | 5 B | 7 B (`blocked`) |

Takeaway: same 100ms, 15x slower — blocked threads can't overlap, so
requests queue behind each other (p50 = 3x the sleep = waiting for a free
thread). Under `current_thread` this would be ~10 req/s. Sleep lesson done.

## Run 6 — multi-threaded `/cpu` (10M wrapping ops) (2026-09-18)

- Handler: 10M `wrapping_add`/`wrapping_mul` loop, returns owned `String` (19 B)
- Load: `oha -n 500 -c 20 http://127.0.0.1:3000/cpu`

| Metric | Value | Meaning |
|--------|-------|---------|
| Requests/sec | **1,088** | compute-bound throughput |
| Average | 18.0 ms | per-request compute + overhead (fastest 7.2ms ≈ pure compute on free core) |
| p50 | 18.4 ms | mean ≈ median = healthy, no queue collapse |
| p99 | 27.3 ms | mild tail from cores oversubscribed (20 concurrent > core count) |
| Success | 100%, 500 × `[200]` | zero errors |

Takeaway: first endpoint where the work itself dominates. Throughput ≈
`cores / seconds-per-request`. Next (Run 7): same command on
`current_thread` — expect ≈ 1/core of this number. That ratio is the payoff.

## Run 7 — single-threaded `/cpu` (same 10M ops) (2026-09-18)

- Runtime flipped to `flavor = "current_thread"`, same binary otherwise
- Load: `oha -n 500 -c 20 http://127.0.0.1:3000/cpu`

| Metric | multi (run 6) | single (run 7) |
|--------|--------------|----------------|
| Requests/sec | 1,088 | **145.6 (~7.5x slower)** |
| Average | 18.0 ms | 134.8 ms |
| p50 | 18.4 ms | 135.9 ms |
| Histogram | bell around 18ms | spike at ~136ms = 20 queued × ~7ms each, one thread |
| Fastest | 7.2 ms | 11.7 ms (≈ pure compute, no queue) |

Takeaway: THE payoff number. 7.5x ≈ your core count — compute scales with
threads, almost linearly. Compare the trilogy: `/greet` +7.5% (nothing to
parallelize), `/sleep` 0% (waiting overlaps on 1 thread), `/cpu` +650%
(work splits across cores). Flip back to `multi_thread` now.
