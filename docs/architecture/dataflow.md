# Dataflow

- Input -> event bus -> tab manager -> layout engine -> renderer.
- Markup parse yields DOM-like tree; assembly parse yields typed IR.
- Renderer produces render ops and VM effect requests.
- VM produces side effects and state updates consumed by renderer.
