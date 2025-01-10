# `risc0-hack` Feature

## Issue Overview

The goal is to run Iota execution layer inside of RISC Zero guest. RISC Zero guest is somewhat limited, it doesn't fully support filesystem, networking,(POSIX I/O is partially supported though), etc.. `iota-adapter` crate and its dependencies are structured very poorly, crates expose API that might not be used by an external crate but that still depends on many other crates which again might not be used. In some cases this simplifies development cycle, but in our case it makes it impossible to use RISC Zero.

### Curious Cases Of Weird Dependencies

`A => B` means that crate `A` *depends on* crate `B`.

- `iota-protocol-config` => `clap`
- `fastcrypto` => `tokio`
- `fastcrypto-zkp` => `reqwest`, `rustls-webpki`
- `move-disassembler` => `colored`
- `iota-types` => `move-command-line-common` => `walkdir`

Note, these weird dependencies might be outdated and already fixed.

## The Correct Solution

The correct solution would be to completely reorganize crates structure, decompose crates in a way that they can expose only necessary API without dependencies on unused crates. This can be done via cargo features or by decomposing crates into smaller but more encapsulated ones. This solution however is not feasible at the moment.

## The Hacky Solution

The quick and dirty solution is to introduce modifications in `Cargo.toml` and source files of bad dependency crates that will disable unnecessary dependencies. This is done with `risc0-hack` solution. The name is chosen to signify its "hacky" nature and to be able to easily locate the corresponding modifications.

### Adding `risc0-hack` Feature

1. import bad external dependency (those not supported by risc0 guest: `clap`, `tokio`, `reqwest`, `walkdir`, etc.) as `optional = true`;
2. import bad internal dependency (those depending on bad external/internal dependencies: `iota-types`, `iota-protocol-config`, etc.) in workspace `Cargo.toml` with `default-features = false`;
3. introduce `risc0-hack` feature in all bad internal crates: it will enable optional bad external dependencies and recursively enable `risc0-hack` feature in bad internal dependencies;
4. add `risc0-hack` feature as `default`;
5. in the source code guard pieces of code that uses optional crates or features from crates with `risc0-hack` using `#[cfg(feature = "risc0-hack")]` guard.

This way, by default the `risc0-hack` feature is enabled and all dependencies are enabled. If there is a need to disable all bad and unnecessary dependencies, the crate can be imported with `default-features = false`.

NOTE: `default-features = false` must be specified in workspace `Cargo.toml`, in member crate `Cargo.toml` it does not have any effect!

### Example

Workspace `Cargo.toml`:

```toml
iota-types = { path = "crates/iota-types", default-features = false }
member-crate = { path = "crates/member-crate", default-features = false }
```

Member crate `Cargo.toml`:

```toml
iota-types.workspace = true
clap = { version = "4", optional = true }

[features]
default = ["risc0-hack"]
risc0-hack = ["dep:clap", "iota-types/risc0-hack",]
```

`lib.rs`:

```rs
#[cfg(feature = "risc0-hack")]
pub fn use_clap() {
  use clap::*;
}

#[cfg_attr(feature = "risc0-hack", derive(clap::ValueEnum))]
pub enum E {}
```

### Identifying Bad Dependencies

The offending dependencies can be identified with `cargo tree` command:

```sh
cargo tree -vv --offline --depth 10
cargo tree -vv --offline -e all --depth 10
cargo tree -vv --offline -i iota-types --depth 10 --no-default-features
cargo tree -vv --offline -e all -i iota-types --depth 10 --no-default-features
cargo tree -vv --offline -i fastcrypto --no-default-features
```
