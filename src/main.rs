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
