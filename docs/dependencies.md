# Dependencies, modules & syntax used so far

Living doc — extend it every time we add a crate or use a new item.
(Original goal was Rust APIs + load testing; the single-threaded baseline is
done and the project has since pivoted — see `roadmap.md` for what's next.)

## Crates (`Cargo.toml`)

### `axum = "0.8"` — web framework (like Express / FastAPI's routing layer)
Handles routing + HTTP responses. Built on `tokio` + `hyper` + `tower`.

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Router` | Holds the URL map | `Router::new()` makes empty router; `.route(path, method)` registers a path |
| `routing::get` | Wraps a handler as "HTTP GET only" | `get(hello)` — pass function as value, no `()` |
| `routing::post` | Wraps a handler as "HTTP POST only" (wrong verb → `405`) | `post(echo)` |
| `axum::serve` | Runs the server forever | `axum::serve(listener, app).await` — takes socket + router, never returns unless it crashes |
| `axum::Json` | Wrapper turning a struct into `application/json` + `200 OK` | `Json(MyStruct { ... })` — inner type must impl `Serialize` |
| `extract::Path` | Pulls a route capture (`{id}`) into a typed arg; bad parse → `400` | `Path(id): Path<u32>` — route must declare `{id}` (axum 0.8 syntax, not `:id`) |
| `extract::Query` | Parses `?key=val` into a struct; missing field → `422` unless `#[serde(default)]` | `Query(q): Query<UserQuery>` |
| `extract::FromRequestParts` | Trait you implement to make your own type an extractor | `impl<S: Send + Sync> FromRequestParts<S> for AuthUser` — reads headers only, so a handler can *also* take a `Json` body (use `FromRequest` only if you need the body) |
| `http::request::Parts` | The headers half of a request, handed to your extractor | `parts.headers.get("Authorization")` |

### Writing a custom extractor

A handler argument that implements `FromRequestParts` becomes an extractor: axum
runs it *before* the handler body, and on `Err` it sends your `Rejection` and
never calls the handler. This is why `AuthUser` is better than an
`if let Some(user) = check_token(...)` guard in the body — the check is part of
the signature, so a route cannot forget it.

```rust
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = (StatusCode, String);   // you choose status AND body
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> { ... }
}
```

Order matters: axum runs extractors left-to-right and stops at the first
failure, so put cheap rejections (missing header) before expensive ones
(signature verification).

### `serde = "1"` (feature `derive`) + `serde_json = "1"` — serialization

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Serialize` | Trait (interface) meaning "can be converted to JSON" | `use serde::Serialize;` + `#[derive(Serialize)]` on struct |
| `Deserialize` | Trait meaning "can be built from JSON" (request bodies) | `#[derive(Deserialize)]`; `Json(payload): Json<EchoMsg>` extractor auto-rejects bad bodies with `422` |
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

### `jsonwebtoken = "9"` — JWT issue + verify (`/login`)

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Header::default()` | Token header; default = HS256 (HMAC-SHA256, symmetric) | `encode(&Header::default(), &claims, &key)` |
| `EncodingKey` | The signing key, built from raw secret bytes | `EncodingKey::from_secret(SECRET)` — `SECRET` is `&[u8]` |
| `encode` | Signs claims → a `xxx.yyy.zzz` token String | `encode(...)` returns `Result<String, Error>` |
| `DecodingKey` | The verifying key — same secret for HS256 | `DecodingKey::from_secret(SECRET)` |
| `Validation::default()` | Which checks to enforce on decode | `decode::<Claims>(token, &key, &Validation::default())` |
| `decode` | Verifies signature **and** `exp`; returns the claims back | `decode::<Claims>(...)` → `Result<TokenData<Claims>, Error>` |

A JWT is three base64url parts: `header.payload.signature`. Only the payload is
readable; the signature is what proves it wasn't tampered with. Verification never
trusts the payload — it re-signs and compares.

### `chrono = "0.4"` — dates & durations (token expiry)

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `Utc::now()` | Current UTC time | `(Utc::now() + Duration::hours(1)).timestamp()` |
| `Duration` | A span of time (hours/minutes/days) | `Duration::hours(1)`, `Duration::minutes(30)` |
| `.timestamp()` | Seconds since the Unix epoch (1970-01-01) | returns `i64`; `as usize` for the claim |

### `dotenvy = "0.15"` — load `.env` into the environment

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `dotenvy::dotenv()` | Reads `.env` and sets each key as a process env var | `dotenvy::dotenv().ok();` — must run **first** in `main`, before anything reads a var. `.ok()` swallows "no .env" |
| `std::env::var` | Read one env var by name (**not** `getenv` — that's C) | `std::env::var("JWT_SECRET")` → `Result<String, VarError>` |

Rust has no built-in `.env` support. The file is just text until something loads
it — `dotenvy` is that something. `.env` is gitignored so real secrets never
enter git; `.env.example` (committed) documents which keys are needed.

### `std::sync::LazyLock` — a global that can read the environment

| Item | What it is | Syntax it brings |
|------|-----------|------------------|
| `const` | Compile-time constant, inlined at every use site | `const SECRET: &[u8] = b"..."` — **can never read the environment**, because the value is fixed before the program runs |
| `static` | A single global living for the whole program, stored in memory | `static SECRET: LazyLock<Vec<u8>> = ...` |
| `LazyLock::new` | Takes a closure, runs it **once** on first access, caches the result | `LazyLock::new(\|\| { ... })` — the escape hatch for "global whose value isn't known until runtime" |
| deref | Accessing a `LazyLock` transparently yields the value inside | `SECRET.as_slice()` / `&SECRET` gives the `Vec<u8>` |


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
| `mod name;` | Declares a module = a sibling file. **A `.rs` file nobody declares is never compiled** — green `cargo check` proves only that what you declared is valid |
| `Result<T, E>` | Rust's error handling; no exceptions, no `try`/`catch` | `?` after a fallible call: on `Ok` bind the value, on `Err` return early from the function |
| `impl Trait for Type` | Implement a trait (interface) on your own type | `impl<S: Send + Sync> FromRequestParts<S> for AuthUser` — this is what turns a struct into an extractor |
| `<S: Send + Sync>` | Generic parameter with a bound: usable across async threads | Needed on extractor impls because axum calls them from any worker thread |
| `async fn` in a trait impl | The trait requires an async fn, so the impl is too | `async fn from_request_parts(...) -> Result<Self, Self::Rejection>` |
| `.ok_or(...)` / `.map_err(...)` | Convert an error into the type your function returns | `.map_err(\|e\| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?` |
| `.parse()` | String → number, fallibly | `"42".parse::<u32>()` → `Result<u32, _>`; `?` or `.unwrap()` to use it |

## Project notes

- Bind `127.0.0.1:3000` = loopback only (same machine). `0.0.0.0:3000` = all interfaces (needed for LAN/phone access). `192.168.1.1` is the router, not your machine.
- Always load-test with `cargo run --release` (debug builds are unoptimised).
- Load tester: `oha` (external binary, `cargo install oha`) — the autocannon equivalent.
- **Division of labour:** Sanket writes the Rust source; the assistant writes the docs and the `// Lesson:` comment headers.
- Generate a signing secret with `openssl rand -hex 32` (256 bits). Never reuse one secret across environments, and never commit a real one.

## Gotchas hit so far (each one cost a build)

| Gotcha | What happened | Rule |
|--------|---------------|------|
| `Path` vs `path` | `use axum::extract::path` → `cannot find tuple struct Path` | Rust is case-sensitive, always |
| `http:StatusCode` | Single colon in a `use` path | Paths use `::` |
| `:id` vs `{id}` | Axum 0.8 **panics at startup** on `:id` | 0.8 uses `{id}`; compiles fine, crashes on run |
| `/user` vs `/users` | Route and curl disagreed → `404` with an empty body | A 404 means the exact path+verb doesn't exist |
| Undeclared `mod` | A new file with errors still gave green `cargo check` | Add `mod name;` or the file is invisible to the compiler |
| `b"str".to_string()` | Byte literals don't implement `Display` | Use `"str"` unless you specifically want bytes |
| `str.to_string` | Method without `()` | Rust requires the call parentheses |
| `Path` vs `Json` errors | `400` for a bad URL segment, `422` for a bad body | URL parsing fails before body parsing |
