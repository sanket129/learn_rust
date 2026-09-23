# Dependencies, modules & syntax used so far

Living doc — extend it every time we add a crate or use a new item.
(Project goal: Rust APIs + load testing. Single-threaded baseline first.)

## Crates (`Cargo.toml`)

### `axum = "0.8"` — web framework (like Express / FastAPI's routing layer)
Handles routing + HTTP responses. Built on `tokio` + `hyper` + `tower`.

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Router` | Holds the URL map | `Router::new()` makes empty router; `.route(path, method)` registers a path |
| `routing::get` | Wraps a handler as "HTTP GET only" | `get(hello)` — pass function as value, no `()` |
| `axum::serve` | Runs the server forever | `axum::serve(listener, app).await` — takes socket + router, never returns unless it crashes |
| `axum::Json` | Wrapper turning a struct into `application/json` + `200 OK` | `Json(MyStruct { ... })` — inner type must impl `Serialize` |

### `serde = "1"` (feature `derive`) + `serde_json = "1"` — serialization

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Serialize` | Trait (interface) meaning "can be converted to JSON" | `use serde::Serialize;` + `#[derive(Serialize)]` on struct |
| `#[derive(Serialize)]` | Auto-writes the struct→JSON mapping | Place directly above `struct` definition |

### `std::time` + `tokio::time` — async sleep (`/sleep`)

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `std::time::Duration` | Stdlib time length | `Duration::from_millis(100)` |
| `tokio::time::sleep` | Non-blocking sleep: yields thread, wakes after duration | `sleep(dur).await` — other requests run meanwhile |
| `std::thread::sleep` | BLOCKING sleep: holds the OS thread, nothing else runs on it | `std::thread::sleep(dur)` — the async antipattern, demo only |

### Owned responses + burn loop (`/cpu`)

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `String` | Owned, heap-allocated text (vs borrowed `&str`) | `-> String` return; `format!("{x}")` builds it; axum sends it like `&str` |
| `mut` | Makes a binding mutable (immutable by default) | `let mut x = ...;` |
| `wrapping_add` / `wrapping_mul` | Overflow-wrapping arithmetic (no debug panic) | `x.wrapping_add(i).wrapping_mul(31)` |
| `0..N` ranges | Iterator over numbers | `for i in 0..10_000_000` (`_` = readability separator) |

### `tokio = "1"` (feature `full`) — async runtime (like `asyncio`'s event loop)
Rust has no built-in event loop, so this provides it. `full` enables all
features so we don't pick them one by one (macros, runtime, networking…).

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `#[tokio::main]` | Attribute macro: generates a hidden normal `main` that builds the runtime and runs your `async main` inside it | `#[tokio::main(flavor = "current_thread")]` — flavors: `current_thread` (1 OS thread, our baseline), `multi_thread` (default, one thread per core), `local` |
| `tokio::net::TcpListener` | Async TCP socket waiting for connections | `TcpListener::bind("127.0.0.1:3000").await` — `bind` returns a Future, `.await` waits for the OS |

## Rust syntax / keywords met so far

| Syntax | Meaning |
|--------|---------|
| `use axum::{Router, routing::get};` | Import names into scope (like `from x import y`) |
| `async fn name() -> Type { }` | Async function — returns a Future, does nothing until the runtime polls it; only `async` fns can use `.await` |
| `.await` | Pause this function until the Future resolves (same idea as Python `await`) |
| `let x = ...;` | Variable binding, immutable by default |
| `::` | Path separator: `Crate::module::Item` / `Type::associated_fn()` (like a static method call) |
| `-> &'static str` | Return type (compulsory, checked at compile time). `&'static str` = borrowed text living for the whole program (`"hello"` is baked into the binary). `axum` converts it into a `200 OK` response |
| `.unwrap()` | On a `Result`: if `Ok`, give the value; if `Err`, crash (panic). OK for learning, never for prod. Rust has no exceptions — fallible ops return `Result<Ok, Err>` |
| `#[...]` | Attribute (macro) — code that rewrites the item below it at compile time |

## Project notes

- Bind `127.0.0.1:3000` = loopback only (same machine). `0.0.0.0:3000` = all interfaces (needed for LAN/phone access). `192.168.1.1` is the router, not your machine.
- Always load-test with `cargo run --release` (debug builds are unoptimised).
- Load tester: `oha` (external binary, `cargo install oha`) — the autocannon equivalent.
