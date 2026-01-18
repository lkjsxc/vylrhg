# System Architecture

## Layers

- UI shell: input collection, focus management, keymap resolution.
- Layout engine: deterministic spatial tree and constraint solver.
- Tab manager: lifecycle, isolation boundaries, session persistence.
- Renderer: tree diffing, render graph construction, render ops emission.
- VM: typed execution of assembly handlers and effect production.

## Cross-Cutting Concerns

- Async runtime: deterministic scheduling tiers and budgeted execution.
- Event bus: explicit message contracts and ordering guarantees.
- Persistence: session state snapshots and tab recovery.

## Invariants

- Rendering is state-driven: immutable trees + incremental diffs.
- Execution is capability-scoped: each tab has isolated VM context.
- Effects are explicit and traceable; no implicit IO.
