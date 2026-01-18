# Semantics

## Component Mapping

- Elements map to render components.
- Component resolution is deterministic.

## Bindings

- Sidecar nodes bind to assembly modules by ID.
- Handler references are resolved at load time.

## Layout

- Layout hints are typed nodes, not attributes.
- Layout hints are validated during parse.

## Events

- Events are routed through explicit handler nodes.
- Unhandled events are ignored by default.
