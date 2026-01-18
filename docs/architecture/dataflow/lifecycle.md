# Lifecycle

1. Input subsystem emits normalized events.
2. Event bus routes to tab manager and active tile.
3. VM executes handlers, producing state diffs + effects.
4. Renderer rebuilds render tree and graph from new state.
5. Layout engine resolves geometry and emits render ops.
6. Render backend draws and swaps buffers.
