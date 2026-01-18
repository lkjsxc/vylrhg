# System Architecture

- Layers: UI shell, layout engine, tab manager, renderer, VM.
- Cross-cutting: async runtime, event bus, persistence layer.
- Rendering is state-driven: immutable trees + incremental diffs.
- Execution is capability-scoped: each tab has isolated VM context.
