# Goals

## Product Goals

- Provide a super-app shell that can host multiple tabs with independent state and layout.
- Preserve deterministic behavior under load; same inputs yield same outputs.
- Guarantee recoverable state through persistent session snapshots.

## Engineering Goals

- Use a unified VM to execute assembly handlers and drive render updates.
- Ensure layout and rendering are pure functions over immutable trees and diffs.
- Keep subsystems loosely coupled via explicit message contracts.

## Language Goals

- Maintain a strict, attribute-free syntax model for markup and assembly.
- Enforce static typing across assembly modules and module boundaries.
- Enable interop via shared module IDs and explicit bindings.

## Operational Goals

- Support reproducible builds across local and containerized environments.
- Keep config deterministic and reload-driven rather than hot-patched.
- Provide strict failure visibility through explicit effect tracing.
