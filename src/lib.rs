//! System Index - System Information Tool
//!
//! A CLI and TUI tool for displaying comprehensive system information including
//! CPU, memory, disk, network, and operating system details.

pub mod models;
pub mod tui;

pub use models::*;

/// Application result type
pub type Result<T> = anyhow::Result<T>;
