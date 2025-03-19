//! Multicrates library re-exporting all member crates.

// Re-export all member crates with feature flags
#[cfg(feature = "multibase")]
pub use multibase;

#[cfg(feature = "multicodec")]
pub use multicodec;

#[cfg(feature = "multihash")]
pub use multihash;

#[cfg(feature = "multisig")]
pub use multisig;

#[cfg(feature = "multitrait")]
pub use multitrait;

#[cfg(feature = "multiutil")]
pub use multiutil;

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

