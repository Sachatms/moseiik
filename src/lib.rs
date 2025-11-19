#[path = "main.rs"]
pub mod main;

// Re-export public items for easier access from integration tests
pub use main::{Options, compute_mosaic};
