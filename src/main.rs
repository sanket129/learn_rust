// Lesson: process entry point; order matters. The `mod` lines declare sibling
// files — a .rs file nobody declares is never compiled at all. `#[tokio::main]`
// builds the async runtime that runs the body, and dotenvy must load .env
// before any code reads an env var (the secret in src/auth.rs does).
mod app;
mod handlers;
mod auth;

#[tokio::main(flavor="multi_thread")]
async fn main() {
    dotenvy::dotenv().ok();
    let app = app::create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
