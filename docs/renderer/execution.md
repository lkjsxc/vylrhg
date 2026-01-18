# Execution

## Event Handling

- Event loop dispatches events to VM handlers.
- VM returns state diffs applied to render tree.
- Side effects are explicit and audited.

## Frame Lifecycle

- Input events are batched per frame.
- Renderer applies state diffs before layout.
- Render ops are emitted and submitted to backend.
