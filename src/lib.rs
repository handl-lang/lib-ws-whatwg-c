//! `http` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `http.hnl`.
pub const HTTP_HNL: &str = include_str!("http.hnl");
