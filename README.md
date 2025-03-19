# Multicrates

This repo combines multiple crates into a single workspace to make the dependencies between them easier to manage.

## Crates

- [./crates/multibase](./crates/multibase) from [https://github.com/cryptidtech/rust-multibase](https://github.com/cryptidtech/rust-multibase)
- [./crates/multicodec](./crates/multicodec) from [https://github.com/cryptidtech/rust-multicodec](https://github.com/cryptidtech/rst-multicodec)
- [./crates/multihash](./crates/multihash) from [https://github.com/cryptidtech/multihash](https://github.com/cryptidtech/multihash)
- [./crates/multisig](./crates/multisig) from [https://github.com/cryptidtech/multisig](https://github.com/cryptidtech/multisig)
- [./crates/multitrait](./crates/multitrait) from [https://github.com/cryptidtech/multitrait](https://github.com/cryptidtech/multitrait)
- [./crates/multiutil](./crates/multiutil) from [https://github.com/cryptidtech/multiutil](https://github.com/cryptidtech/multiutil)

## Usage

Enable the crate you need with a feature flag:

```toml
[dependencies]
multicrates = { version = "*", features = ["multibase", "multicodec", "multihash", "multisig", "multitrait", "multiutil"] }
```

To use the member crates, import the re-export:

```rust
use multicrates::{multibase, multicodec, multihash, multisig, multitrait, multiutil};
```
