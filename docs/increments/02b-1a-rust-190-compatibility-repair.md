# Increment 2B-1A — Rust 1.90 compatibility repair

## Goal

Repair the incomplete local Increment 2B-1 application without expanding product scope.

This repair restores the platform-neutral Increment 2A Rust modules when they are absent and selects a `rusqlite` release compatible with the repository's pinned Rust 1.90 toolchain.

## Status

Status: **Verified complete on target Mac**

## Failures addressed

### Missing core modules

`src-tauri/src/lib.rs` declares these modules:

```text
agent
approvals
audit
memory
platform
policy
tools
storage
```

The original Increment 2B-1 overlay contained only files changed by 2B-1. It assumed the verified Increment 2A module tree was already present. A checkout based on the public baseline therefore failed with Rust error `E0583` because the Increment 2A directories were missing.

This repair overlay includes the complete platform-neutral Rust source tree through Increment 2B-1.

### Rust 1.90 dependency incompatibility

The first 2B-1 manifest selected:

```toml
rusqlite = { version = "=0.40.1", features = ["bundled-sqlcipher-vendored-openssl"] }
```

That resolves to `libsqlite3-sys 0.38.1`. Its build script uses the standard-library `cfg_select!` macro, which is unavailable on the repository's pinned Rust 1.90 toolchain.

The repaired manifest selects:

```toml
rusqlite = { version = "=0.37.0", features = ["bundled-sqlcipher-vendored-openssl"] }
```

`rusqlite 0.37.0` retains the required bundled SQLCipher and vendored OpenSSL feature while resolving to the earlier `libsqlite3-sys` line that does not use `cfg_select!`.

## Scope preserved

The repair does not add:

- new Tauri commands,
- UI changes,
- API keys,
- network access,
- Keychain integration,
- macOS permissions,
- Accessibility,
- screen capture,
- Apple Events,
- shell execution,
- product-data tables beyond `schema_migrations` and `app_metadata`.

## Required verification

Run on the target Mac:

```bash
cargo update --manifest-path src-tauri/Cargo.toml -p rusqlite --precise 0.37.0

cargo fmt --manifest-path src-tauri/Cargo.toml -- --check

cargo clippy --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --all-features \
  --locked \
  -- -D warnings

cargo test --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --locked

npm run typecheck
npm run build
npm run tauri -- dev
```

The project owner confirmed all commands passed and the existing application window launched without a permission prompt. The resolved dependency tree is `rusqlite 0.37.0` with `libsqlite3-sys 0.35.0`.
