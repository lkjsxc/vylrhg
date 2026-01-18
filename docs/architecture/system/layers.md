# Layers

## UI Shell

- Collects input, resolves modes, and emits normalized events.
- Maintains focus and selection context for tabs and tiles.

## Layout Engine

- Holds immutable layout tree with split/leaf nodes.
- Resolves constraints into concrete geometry for rendering.

## Tab Manager

- Owns tab lifecycles and session persistence.
- Enforces VM isolation boundaries per tab.

## Renderer

- Builds render tree from markup + state.
- Produces render graph and emits render ops.

## VM

- Executes assembly handlers on events and timers.
- Emits state diffs and typed effect requests.
