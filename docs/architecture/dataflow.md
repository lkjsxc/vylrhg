# Dataflow

## Primary Flow

- Input -> event bus -> tab manager -> layout engine -> renderer.
- Markup parse yields immutable node tree.
- Assembly parse yields typed IR and module bindings.
- Renderer produces render ops and VM effect requests.
- VM produces state updates and explicit side effects.

## Data Artifacts

- Render tree: markup + state integration snapshot.
- Render graph: post-layout graph for ops emission.
- Effect list: ordered, typed effect requests.
- State diff: minimal patch applied to render tree.
