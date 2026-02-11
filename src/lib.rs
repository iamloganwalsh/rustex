pub mod types;
pub mod orderbook;
pub mod matching;

// Re-export important types for clean external use
pub use matching::MatchingEngine;
pub use types::{Order, Side};
