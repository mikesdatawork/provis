//! Process monitoring module
//!
//! This module provides functionality for monitoring system processes
//! including data collection, statistics, and visualization.

pub mod stats;
pub mod graph;
pub mod display;

pub use stats::{ProcessStats, ProcessCollector};
pub use graph::{render_bar, render_bar_compact};
pub use display::display_processes;
