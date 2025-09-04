//! A simple grep-like tool written in Rust
//!
//! This library provides pattern matching functionality similar to grep,
//! with support for case-insensitive matching and line number display.

pub mod app;
pub mod cli;
pub mod errors;
pub mod io;
pub mod search;

// Re-export commonly used types
pub use cli::{CliAction, Config};
pub use errors::{ExitCode, RgrepError};
