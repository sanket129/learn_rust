# learn_rust

An Axum HTTP server used to learn Rust web development, plus a running log of
load-test numbers. Every endpoint exists to demonstrate one framework or runtime
behaviour, so the "silly" ones (`/sleep_block`, `/cpu`) are the point — they're
what the benchmarks measure against.

## Run it

```sh
cp .env.example .env
echo "JWT_SECRET=$(openssl rand -hex 32)" >> .env   # only needed for /login
cargo run
```

Always `cargo run --release` before load testing — debug builds are unoptimised.

## Routes

| Method | Path | Handler | What it demonstrates |
|--------|------|---------|----------------------|
| GET | `/greet` | `hello` | plain `&'static str` → `200`; the throughput baseline |
| GET | `/json` | `json_hello` | `#[derive(Serialize)]` + the `Json` wrapper |
| POST | `/echo` | `echo` | `Deserialize` a body; extractors reject bad input with `4xx` |
| GET | `/users/{id}` | `user` | `Path` + `Query` extractors, `#[serde(default)]` |
| POST | `/login` | `login` | issues an HMAC-signed JWT, secret from `.env` |
| GET | `/me` | `me` | protected — the `AuthUser` extractor verifies the token |
| GET | `/sleep` | `sleep_hello` | `tokio::time::sleep` — yields the thread |
| GET | `/sleep_block` | `sleep_blocking_hello` | `std::thread::sleep` — the antipattern |
| GET | `/cpu` | `cpu_hello` | CPU-bound work; this is where threads pay off |

Try it:

```sh
curl http://127.0.0.1:3000/greet
curl "http://127.0.0.1:3000/users/42?active=true"

# auth round trip
TOKEN=$(curl -s -X POST http://127.0.0.1:3000/login \
  -H 'Content-Type: application/json' -d '{"user_id":42}' \
  | sed -E 's/.*"token":"([^"]+)".*/\1/')
curl -H "Authorization: Bearer $TOKEN" http://127.0.0.1:3000/me   # {"user_id":42}
curl http://127.0.0.1:3000/me                                     # 401 — no token
```

Load test:

```sh
oha -n 50000 -c 100 http://127.0.0.1:3000/greet
```

## Layout

```
src/
  main.rs        entry point: loads .env, binds the socket, serves forever
  app.rs         the route map — one .route() line per endpoint
  auth.rs        the JWT signing secret (LazyLock, read from the environment)
  handlers/      one file per endpoint, each opening with a `// Lesson:` comment
docs/
  dependencies.md  every crate and syntax item used, as a living reference
  benchmarks.md    numbered load-test runs with the reasoning behind each
  Network_codes.md HTTP status code quick reference
  roadmap.md       what's built, what's next, and why
```

## Conventions

- Each handler file starts with a `// Lesson:` comment explaining what it teaches.
- `docs/dependencies.md` is a living document — it gains a row every time a new
  crate or syntax item is used.
- `.env` holds real values and is gitignored; `.env.example` documents the keys.
- A new file is invisible to the compiler until it's declared with `mod`.
