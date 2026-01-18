# Assumptions

- Local storage is available and writable for session data.
- Single-user environment; no concurrent multi-tenant sessions.
- Deterministic ordering of events from the input subsystem.
- Stable clock source for runtime scheduling and time slicing.
- Render targets support incremental updates and double buffering.
