# Runtime to Renderer Interface

- Renderer consumes state snapshots and diffs from runtime.
- Runtime publishes effect outputs that influence render state.
- Renderer emits render ops; runtime does not issue draw calls.
- Contract is versioned by schema identifiers.
