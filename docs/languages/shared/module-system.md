# Module System

## Modules

- Modules are immutable bundles: markup + assembly + assets.
- Module metadata is explicit and typed.

## Dependencies

- Dependency graph is acyclic and topologically loaded.
- Versioning is explicit in module metadata, not in names.

## Resolution

- Names are globally unique and resolved by registry.
- Resolution failures are fatal at load time.
