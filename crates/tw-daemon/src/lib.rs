//! `tw-daemon` library: all logic lives here so `main.rs` stays a thin shell and every module is
//! reachable from tests (and pub items in a lib are not "dead code" under `-D warnings`).
pub mod config;
pub mod supervise;
