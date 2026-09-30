# Roadmap

Where the project is, and what comes next.

## Where we are

Load testing (7 benchmark runs) is done — see `benchmarks.md`. The concurrency
conclusion: threads only matter when there's work to spread across them.
`/greet` gained nothing, `/sleep` gained nothing (waits already overlap),
`/cpu` gained 650% because compute is the one thing a thread genuinely holds.

JWT auth is half built. Token *issuing* works end to end; token *verification*
does not exist yet, so `POST /login` currently hands out a token nothing checks.

## Auth, remaining

| Step | Status |
|------|--------|
| `POST /login` issues a signed JWT | done |
| Secret from `.env` via `LazyLock` (not a `const`) | done |
| `sub` as a JSON string per RFC 7519 | done |
| `Claims` returned inside the token | done |
| **Round-trip test** (`cargo test` has never run here) | todo |
| **`AuthUser` extractor** — a `FromRequestParts` impl that reads `Authorization: Bearer`, verifies signature + expiry, rejects with `401` | todo |
| **`GET /me`** — a protected route reading the `sub` claim | todo |
| **Move routes under `/api/*`** | todo |

The extractor is the interesting part: putting the check in the handler
*signature* rather than the body means a route can't forget it. Any handler that
takes `user: AuthUser` is protected by construction. `FromRequestParts` rather
than `FromRequest` because it only touches headers — that leaves room for a
`Json` body extractor in the same handler, which slice 1 of the app needs.

## The app: vector search over documents

Decision: build the whole search machinery against a **deterministic local
embedder** first. Chunking, indexing, cosine similarity and top-k are the same
Rust whichever embedder feeds them, and a local function runs offline with no
API key. Swapping in a real model later is a one-function change. A local model
in Rust (`candle`/`fastembed`) was rejected for now: a 10+ minute first compile
and a model download is a session lost to fighting the build.

| Slice | Build | New Rust |
|-------|-------|----------|
| 1 | `POST /api/docs` — upload title + text, store | `Arc<Mutex<T>>` shared state |
| 2 | Chunking — split stored text into overlapping chunks | string slicing, `chars()`, ranges |
| 3 | Embed + index — a `Vec<f32>` per chunk | `f32` math, sorting |
| 4 | `POST /api/search` — embed the query, cosine similarity, top-k | the payoff endpoint |
| 5 | Swap the embedder for a real model | one function |
| 6 | Frontend | — |

## Frontend

React + Vite, chosen for job relevance. Vite's dev proxy (`/api` →
`localhost:3000`) means **no CORS middleware and no `tower-http`** in Rust — two
processes in development, zero extra Rust dependencies.

Don't stub UI for routes that don't exist yet. Add a card per lesson as the
lesson lands; a page of greyed-out placeholders teaches nothing and all of it
gets rewritten.

## Why `/api/*` before the frontend

Free now, annoying to retrofit. Every new route goes under `/api/` so the UI can
call `/api/login` and the browser preview endpoints stay at the root.
