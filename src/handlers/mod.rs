// Every handler file must be declared here or Rust will not compile it. An
// undeclared file still sits on disk, silently, while `cargo check` stays green.
pub mod hello;
pub mod json;
pub mod sleep;
pub mod sleep_blocking;
pub mod cpu;
pub mod echo;
pub mod user;
pub mod auth;

