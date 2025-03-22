//! Multicrates library re-exporting all member crates.

// Re-export all member crates with feature flags
#[cfg(feature = "multibase")]
/// Re-exports the `multibase` crate when the "multibase" feature is enabled.
///
/// The multibase crate provides functionality for encoding and decoding data in various base-encoding formats.
pub use multibase;

#[cfg(feature = "multicodec")]
/// Re-exports the `multicodec` crate when the "multicodec" feature is enabled.
///
/// The multicodec crate provides functionality for codec identification and transformation.
pub use multicodec;

#[cfg(feature = "multihash")]
/// Re-exports the `multihash` crate when the "multihash" feature is enabled.
///
/// The multihash crate provides functionality for cryptographic hash function identification and computation.
pub use multihash;

#[cfg(feature = "multisig")]
/// Re-exports the `multisig` crate when the "multisig" feature is enabled.
///
/// The multisig crate provides functionality for multi-signature schemes.
pub use multisig;

#[cfg(feature = "multitrait")]
/// Re-exports the `multitrait` crate when the "multitrait" feature is enabled.
///
/// The multitrait crate provides common traits used across the multi* ecosystem.
pub use multitrait;

#[cfg(feature = "multiutil")]
/// Re-exports the `multiutil` crate when the "multiutil" feature is enabled.
///
/// The multiutil crate provides utility functions used across the multi* ecosystem.
pub use multiutil;

/// Prelude module containing all crates for easy importing.
///
/// This module re-exports all the available crates based on enabled features,
/// allowing users to import everything with a single `use multicrates::prelude::*` statement.
pub mod prelude {
    #[cfg(feature = "multibase")]
    /// Re-exports the `multibase` crate in the prelude when the "multibase" feature is enabled.
    pub use crate::multibase;

    #[cfg(feature = "multicodec")]
    /// Re-exports the `multicodec` crate in the prelude when the "multicodec" feature is enabled.
    pub use crate::multicodec;

    #[cfg(feature = "multihash")]
    /// Re-exports the `multihash` crate in the prelude when the "multihash" feature is enabled.
    pub use crate::multihash;

    #[cfg(feature = "multisig")]
    /// Re-exports the `multisig` crate in the prelude when the "multisig" feature is enabled.
    pub use crate::multisig;

    #[cfg(feature = "multitrait")]
    /// Re-exports the `multitrait` crate in the prelude when the "multitrait" feature is enabled.
    pub use crate::multitrait;

    #[cfg(feature = "multiutil")]
    /// Re-exports the `multiutil` crate in the prelude when the "multiutil" feature is enabled.
    pub use crate::multiutil;
}

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

