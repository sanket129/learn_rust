// Lesson: route map. One .route() line per endpoint, no handlers here.
use axum::{Router,routing::{get, post}};
use crate::handlers::hello::hello;
use crate::handlers::json::json_hello;
use crate::handlers::sleep::sleep_hello;
use crate::handlers::sleep_blocking::sleep_blocking_hello;
use crate::handlers::cpu::cpu_hello;
use crate::handlers::echo::echo;
use crate::handlers::user::user;

pub fn create_router() -> Router {
    Router::new()
        .route("/cpu", get(cpu_hello))
        .route("/sleep_block",get(sleep_blocking_hello))
        .route("/sleep", get(sleep_hello))
        .route("/greet", get(hello))
        .route("/json", get(json_hello))
        .route("/echo", post(echo))
        .route("/users/{id}", get(user))
}
