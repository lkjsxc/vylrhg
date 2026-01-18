# Scope

## In Scope

- Super-app shell combining tabs, tiles, and renderer-backed content.
- Rust-based implementation with tokio-driven async runtime.
- Deterministic renderer pipeline that consumes markup + assembly outputs.
- Local execution model with per-tab VM isolation.
- Persistent session state stored on local disk.
- Custom languages: `vylrhg-assembly` and `vaylrhg-markup`, with shared module IDs.

## Out of Scope

- Remote execution or untrusted multi-tenant hosting.
- Legacy browser compatibility or web standards compliance.
- Backward compatibility across language or runtime versions.
- Third-party plugin ecosystems with dynamic code loading.
