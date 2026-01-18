# Runtime

- Per-tab VM instance with isolated heap.
- Garbage collection: stop-the-world, precise, generational.
- Foreign calls: limited to UI and renderer capabilities.
- Deterministic scheduling through cooperative yields.
