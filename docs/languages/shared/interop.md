# Interop

## Bindings

- Shared module IDs unify markup and assembly assets.
- Sidecar nodes reference assembly handlers by ID.
- Bindings are static and validated at load time.

## Constraints

- Module boundaries are explicit and versionless.
- Cyclic dependencies are rejected.
- All bindings are resolved before runtime activation.
