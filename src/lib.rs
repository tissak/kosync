// ╦  ┌─┐┬ ┬┌─┐┬─┐ Lzyor Studio
// ║  ┌─┘└┬┘│ │├┬┘ kosync-project
// ╩═╝└─┘ ┴ └─┘┴└─ https://lzyor.work/koreader/
// 2023 (c) Lzyor

//! KOSync - A progress synchronization server for KOReader
//!
//! This library provides functionality for synchronizing reading progress
//! across multiple devices using KOReader.

pub mod api;
pub mod db;
pub mod defs;
pub mod utils;

use shadow_rs::shadow;
shadow!(build);