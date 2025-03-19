//! Multicrates library re-exporting all member crates.

// Re-export all member crates
pub use multibase;
pub use multicodec;
pub use multihash;
pub use multisig;
pub use multitrait;
pub use multiutil;

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

